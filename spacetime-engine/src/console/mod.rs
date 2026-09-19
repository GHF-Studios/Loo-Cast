//! Compact developer console and extensible command dispatch.
//!
//! The console owns text parsing, history, completion, focus and presentation.
//! Commands are registrations with metadata plus an exclusive-World handler.

use std::collections::{BTreeMap, VecDeque};

use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPrimaryContextPass, PrimaryEguiContext, egui};

use crate::input_focus::{InputFocus, InputFocusSet};

const CONSOLE_FOCUS_OWNER: &str = "developer_console";
const MAX_SCROLLBACK: usize = 512;
const MAX_HISTORY: usize = 128;

pub type ConsoleCommandHandler =
    fn(&mut World, &ConsoleCommandInvocation) -> ConsoleCommandResult;

#[derive(Debug, Clone, Copy)]
pub struct ConsoleCommandSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct RegisteredConsoleCommand {
    spec: ConsoleCommandSpec,
    handler: ConsoleCommandHandler,
}

#[derive(Resource, Default)]
pub struct ConsoleCommandRegistry {
    commands: BTreeMap<String, RegisteredConsoleCommand>,
    aliases: BTreeMap<String, String>,
}

impl ConsoleCommandRegistry {
    pub fn register(&mut self, spec: ConsoleCommandSpec, handler: ConsoleCommandHandler) {
        let canonical = normalize_name(spec.name);
        assert!(!canonical.is_empty(), "console command names must not be empty");
        assert!(
            !self.commands.contains_key(&canonical),
            "duplicate console command `{canonical}`"
        );

        for &alias in spec.aliases {
            let alias = normalize_name(alias);
            assert!(
                !alias.is_empty() && !self.aliases.contains_key(&alias),
                "duplicate/empty console command alias `{alias}`"
            );
            self.aliases.insert(alias, canonical.clone());
        }

        self.commands
            .insert(canonical, RegisteredConsoleCommand { spec, handler });
    }

    fn resolve(&self, name: &str) -> Option<RegisteredConsoleCommand> {
        let name = normalize_name(name);
        if let Some(command) = self.commands.get(&name) {
            return Some(*command);
        }
        let canonical = self.aliases.get(&name)?;
        self.commands.get(canonical).copied()
    }

    fn command_names(&self) -> impl Iterator<Item = &str> {
        self.commands.keys().map(String::as_str)
    }

    fn completions(&self, prefix: &str) -> Vec<String> {
        let prefix = normalize_name(prefix);
        self.command_names()
            .filter(|name| name.starts_with(&prefix))
            .map(ToOwned::to_owned)
            .collect()
    }
}

pub trait AppConsoleExt {
    fn register_console_command(
        &mut self,
        spec: ConsoleCommandSpec,
        handler: ConsoleCommandHandler,
    ) -> &mut Self;
}

impl AppConsoleExt for App {
    fn register_console_command(
        &mut self,
        spec: ConsoleCommandSpec,
        handler: ConsoleCommandHandler,
    ) -> &mut Self {
        self.init_resource::<ConsoleCommandRegistry>();
        self.world_mut()
            .resource_mut::<ConsoleCommandRegistry>()
            .register(spec, handler);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleCommandInvocation {
    raw: String,
    name: String,
    args: Vec<String>,
}

impl ConsoleCommandInvocation {
    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleFocusDisposition {
    KeepConsole,
    ReturnToGameplay,
}

#[derive(Debug)]
pub enum ConsoleCommandResult {
    Silent,
    Success {
        lines: Vec<String>,
        focus: ConsoleFocusDisposition,
    },
    Error(String),
}

impl ConsoleCommandResult {
    pub fn success(line: impl Into<String>) -> Self {
        Self::Success {
            lines: vec![line.into()],
            focus: ConsoleFocusDisposition::KeepConsole,
        }
    }

    pub fn success_and_return_to_gameplay(line: impl Into<String>) -> Self {
        Self::Success {
            lines: vec![line.into()],
            focus: ConsoleFocusDisposition::ReturnToGameplay,
        }
    }

    pub fn lines(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Success {
            lines: lines.into_iter().map(Into::into).collect(),
            focus: ConsoleFocusDisposition::KeepConsole,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }
}

#[derive(Debug, Clone, Copy)]
enum ConsoleLineKind {
    Command,
    Info,
    Error,
}

#[derive(Debug, Clone)]
struct ConsoleLine {
    kind: ConsoleLineKind,
    text: String,
}

#[derive(Resource)]
struct DeveloperConsole {
    open: bool,
    opened_this_frame: bool,
    input: String,
    history: Vec<String>,
    history_cursor: Option<usize>,
    scrollback: VecDeque<ConsoleLine>,
    pending: VecDeque<String>,
}

impl Default for DeveloperConsole {
    fn default() -> Self {
        let mut scrollback = VecDeque::new();
        scrollback.push_back(ConsoleLine {
            kind: ConsoleLineKind::Info,
            text: "Spacetime Engine developer console — type `help`.".to_string(),
        });
        Self {
            open: false,
            opened_this_frame: false,
            input: String::new(),
            history: Vec::new(),
            history_cursor: None,
            scrollback,
            pending: VecDeque::new(),
        }
    }
}

impl DeveloperConsole {
    fn push(&mut self, kind: ConsoleLineKind, text: impl Into<String>) {
        self.scrollback.push_back(ConsoleLine {
            kind,
            text: text.into(),
        });
        while self.scrollback.len() > MAX_SCROLLBACK {
            self.scrollback.pop_front();
        }
    }

    fn submit(&mut self) {
        let command = self.input.trim().to_string();
        self.input.clear();
        self.history_cursor = None;
        if command.is_empty() {
            return;
        }

        if self.history.last() != Some(&command) {
            self.history.push(command.clone());
            if self.history.len() > MAX_HISTORY {
                self.history.remove(0);
            }
        }

        self.push(ConsoleLineKind::Command, format!("] {command}"));
        self.pending.push_back(command);
    }

    fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let next = match self.history_cursor {
            None => self.history.len() - 1,
            Some(0) => 0,
            Some(index) => index - 1,
        };
        self.history_cursor = Some(next);
        self.input.clone_from(&self.history[next]);
    }

    fn history_down(&mut self) {
        let Some(index) = self.history_cursor else {
            return;
        };
        if index + 1 >= self.history.len() {
            self.history_cursor = None;
            self.input.clear();
        } else {
            let next = index + 1;
            self.history_cursor = Some(next);
            self.input.clone_from(&self.history[next]);
        }
    }
}

pub struct DeveloperConsolePlugin;

impl Plugin for DeveloperConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeveloperConsole>()
            .init_resource::<ConsoleCommandRegistry>()
            .init_resource::<InputFocus>()
            .add_systems(PreUpdate, toggle_console.before(InputFocusSet::Resolve))
            .add_systems(Update, dispatch_console_commands)
            .add_systems(EguiPrimaryContextPass, draw_console);

        app.register_console_command(
            ConsoleCommandSpec {
                name: "help",
                aliases: &["?", "commands"],
                usage: "help [command]",
                summary: "List commands or show detailed help for one command.",
            },
            help_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "clear",
                aliases: &["cls"],
                usage: "clear",
                summary: "Clear console scrollback.",
            },
            clear_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "echo",
                aliases: &[],
                usage: "echo <text...>",
                summary: "Print text to the console.",
            },
            echo_command,
        );
    }
}

fn toggle_console(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut console: ResMut<DeveloperConsole>,
    mut focus: ResMut<InputFocus>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        console.open = !console.open;
        console.opened_this_frame = console.open;
        console.history_cursor = None;
    } else if console.open && keyboard.just_pressed(KeyCode::Escape) {
        console.open = false;
        console.history_cursor = None;
    }

    focus.set_modal_claim(CONSOLE_FOCUS_OWNER, console.open);
}

fn dispatch_console_commands(world: &mut World) {
    let pending = {
        let mut console = world.resource_mut::<DeveloperConsole>();
        std::mem::take(&mut console.pending)
    };

    for raw in pending {
        let invocation = match parse_command(&raw) {
            Ok(invocation) => invocation,
            Err(error) => {
                world
                    .resource_mut::<DeveloperConsole>()
                    .push(ConsoleLineKind::Error, error);
                continue;
            }
        };

        let command = {
            let registry = world.resource::<ConsoleCommandRegistry>();
            registry.resolve(invocation.name())
        };

        let Some(command) = command else {
            world.resource_mut::<DeveloperConsole>().push(
                ConsoleLineKind::Error,
                format!(
                    "unknown command `{}` — type `help` to list commands",
                    invocation.name()
                ),
            );
            continue;
        };

        match (command.handler)(world, &invocation) {
            ConsoleCommandResult::Silent => {}
            ConsoleCommandResult::Success { lines, focus } => {
                let mut console = world.resource_mut::<DeveloperConsole>();
                for line in lines {
                    console.push(ConsoleLineKind::Info, line);
                }
                if focus == ConsoleFocusDisposition::ReturnToGameplay {
                    console.open = false;
                    console.history_cursor = None;
                }
            }
            ConsoleCommandResult::Error(error) => {
                world
                    .resource_mut::<DeveloperConsole>()
                    .push(ConsoleLineKind::Error, error);
            }
        }
    }
}

fn draw_console(
    mut contexts: Query<&mut EguiContext, With<PrimaryEguiContext>>,
    mut console: ResMut<DeveloperConsole>,
    registry: Res<ConsoleCommandRegistry>,
) {
    if !console.open {
        return;
    }

    let Ok(mut context) = contexts.single_mut() else {
        return;
    };
    let ctx = context.get_mut();
    let content_rect = ctx.input(|input| input.content_rect());
    let console_height = (content_rect.height() * 0.46).clamp(220.0, 560.0);
    let console_width = content_rect.width();

    // Use an ordinary foreground Area rather than egui's deprecated top-level
    // panel compatibility API. The console still owns a fixed Source-like strip
    // at the top of the viewport, independent of the editor's dock layout.
    egui::Area::new(egui::Id::new("spacetime_developer_console"))
        .order(egui::Order::Foreground)
        .fixed_pos(content_rect.left_top())
        .default_size(egui::vec2(console_width, console_height))
        .show(ctx, |ui| {
            ui.set_width(console_width);
            ui.set_height(console_height);

            egui::Frame::new()
                .fill(egui::Color32::from_rgba_unmultiplied(10, 12, 14, 248))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(72, 82, 92),
                ))
                .show(ui, |ui| {
                    ui.set_width(console_width);
                    ui.set_height(console_height);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("SPACETIME CONSOLE")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(220, 224, 228)),
                        );
                        ui.separator();
                        ui.label(
                            egui::RichText::new(
                                "` toggle   ↑/↓ history   Tab complete   Esc close",
                            )
                            .monospace()
                            .small()
                            .color(egui::Color32::from_rgb(130, 140, 150)),
                        );
                    });
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .auto_shrink([false, false])
                        .max_height((console_height - 62.0).max(80.0))
                        .show(ui, |ui| {
                            for line in &console.scrollback {
                                let color = match line.kind {
                                    ConsoleLineKind::Command => {
                                        egui::Color32::from_rgb(185, 195, 205)
                                    }
                                    ConsoleLineKind::Info => {
                                        egui::Color32::from_rgb(205, 210, 214)
                                    }
                                    ConsoleLineKind::Error => {
                                        egui::Color32::from_rgb(255, 118, 105)
                                    }
                                };
                                ui.label(
                                    egui::RichText::new(&line.text)
                                        .monospace()
                                        .color(color),
                                );
                            }
                        });

                    ui.separator();

                    let response = ui.add(
                        egui::TextEdit::singleline(&mut console.input)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .hint_text("command"),
                    );

                    if console.opened_this_frame {
                        console.input.clear();
                        response.request_focus();
                        console.opened_this_frame = false;
                    }

                    let prompt_active = response.has_focus() || response.lost_focus();
                    if prompt_active {
                        let history_up =
                            ui.input(|input| input.key_pressed(egui::Key::ArrowUp));
                        let history_down =
                            ui.input(|input| input.key_pressed(egui::Key::ArrowDown));
                        let complete =
                            ui.input(|input| input.key_pressed(egui::Key::Tab));
                        let submit =
                            ui.input(|input| input.key_pressed(egui::Key::Enter));

                        if history_up {
                            console.history_up();
                            response.request_focus();
                        }
                        if history_down {
                            console.history_down();
                            response.request_focus();
                        }
                        if complete {
                            complete_command_input(&mut console.input, &registry);
                            response.request_focus();
                        }
                        if submit {
                            console.submit();
                            response.request_focus();
                        }
                    }

                    let prefix = command_prefix(&console.input);
                    let completions = registry.completions(prefix);
                    if !prefix.is_empty() && !completions.is_empty() {
                        let preview = completions
                            .into_iter()
                            .take(8)
                            .collect::<Vec<_>>()
                            .join("  ");
                        ui.label(
                            egui::RichText::new(preview)
                                .monospace()
                                .small()
                                .color(egui::Color32::from_rgb(108, 136, 160)),
                        );
                    }
                });
        });
}

fn complete_command_input(input: &mut String, registry: &ConsoleCommandRegistry) {
    let prefix = command_prefix(input);
    if prefix.is_empty() {
        return;
    }

    let completions = registry.completions(prefix);
    if completions.is_empty() {
        return;
    }

    let completed = if completions.len() == 1 {
        completions[0].clone()
    } else {
        common_prefix(&completions)
    };
    if completed.len() <= prefix.len() {
        return;
    }

    let slash = input.trim_start().starts_with('/');
    *input = format!("{}{} ", if slash { "/" } else { "" }, completed);
}

fn command_prefix(input: &str) -> &str {
    input
        .trim_start()
        .strip_prefix('/')
        .unwrap_or_else(|| input.trim_start())
        .split_whitespace()
        .next()
        .unwrap_or("")
}

fn common_prefix(values: &[String]) -> String {
    let Some(first) = values.first() else {
        return String::new();
    };
    let mut length = first.len();
    for value in &values[1..] {
        length = first
            .bytes()
            .zip(value.bytes())
            .take_while(|(left, right)| left == right)
            .count()
            .min(length);
    }
    first[..length].to_string()
}

fn help_command(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    let registry = world.resource::<ConsoleCommandRegistry>();

    if let Some(name) = invocation.args().first() {
        let Some(command) = registry.resolve(name) else {
            return ConsoleCommandResult::error(format!("unknown command `{name}`"));
        };
        let mut lines = vec![format!(
            "{} — {}",
            command.spec.usage, command.spec.summary
        )];
        if !command.spec.aliases.is_empty() {
            lines.push(format!("aliases: {}", command.spec.aliases.join(", ")));
        }
        return ConsoleCommandResult::Success {
            lines,
            focus: ConsoleFocusDisposition::KeepConsole,
        };
    }

    ConsoleCommandResult::lines(registry.commands.values().map(|command| {
        format!("{:<34} {}", command.spec.usage, command.spec.summary)
    }))
}

fn clear_command(world: &mut World, _: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    world
        .resource_mut::<DeveloperConsole>()
        .scrollback
        .clear();
    ConsoleCommandResult::Silent
}

fn echo_command(_: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    ConsoleCommandResult::success(invocation.args().join(" "))
}

fn parse_command(raw: &str) -> Result<ConsoleCommandInvocation, String> {
    let source = raw.trim();
    let source = source.strip_prefix('/').unwrap_or(source).trim();
    if source.is_empty() {
        return Err("empty command".to_string());
    }

    let tokens = tokenize(source)?;
    let Some((name, args)) = tokens.split_first() else {
        return Err("empty command".to_string());
    };

    Ok(ConsoleCommandInvocation {
        raw: raw.to_string(),
        name: normalize_name(name),
        args: args.to_vec(),
    })
}

fn tokenize(source: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;

    for character in source.chars() {
        if escaped {
            current.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else {
                current.push(character);
            }
            continue;
        }

        match character {
            '"' | '\'' => quote = Some(character),
            character if character.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(character),
        }
    }

    if escaped {
        current.push('\\');
    }
    if quote.is_some() {
        return Err("unterminated quoted argument".to_string());
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn normalize_name(name: &str) -> String {
    name.trim().trim_start_matches('/').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_supports_slash_quotes_and_escapes() {
        let parsed = parse_command(r#"/echo "hello universe" moon\ base"#).unwrap();
        assert_eq!(parsed.name(), "echo");
        assert_eq!(
            parsed.args(),
            &["hello universe".to_string(), "moon base".to_string()]
        );
    }
}
