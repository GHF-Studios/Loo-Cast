use eframe::egui;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

const BUILTIN_MODPACKS: &[ModpackDef] = &[
    ModpackDef {
        id: "vanilla",
        display_name: "Vanilla",
        mods: &["base_mod", "core_mod"],
    },
    ModpackDef {
        id: "kitchen_sink",
        display_name: "Kitchen Sink",
        mods: &["base_mod", "core_mod", "your_extra_mod_here"],
    },
];

#[derive(Clone, Copy)]
struct ModpackDef {
    id: &'static str,
    display_name: &'static str,
    mods: &'static [&'static str],
}

struct LauncherApp {
    sdk_root: String,
    workspace_root: String,
    toolchain: String,
    modpack_idx: usize,
    status: String,
    log_text: String,
    running: Option<RunningTask>,
}

struct RunningTask {
    label: String,
    rx: Receiver<TaskEvent>,
}

enum TaskEvent {
    Log(String),
    Done(Result<(), String>),
}

#[derive(Clone)]
struct LauncherConfig {
    sdk_root: PathBuf,
    workspace_root: PathBuf,
    toolchain: String,
    selected_modpack: ModpackDef,
}

#[derive(Clone)]
struct CommandSpec {
    program: PathBuf,
    args: Vec<String>,
    cwd: PathBuf,
    env: Vec<(String, OsString)>,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Loo-Cast Launcher (MVP)", options, Box::new(|_cc| Ok(Box::new(LauncherApp::new()))))
}

impl LauncherApp {
    fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let workspace_root = find_workspace_root(&cwd);
        let running_from_workspace_root = workspace_root.as_ref().map(|root| root == &cwd).unwrap_or(false);
        let sdk_root = if running_from_workspace_root {
            cwd.join("build").join("sdk")
        } else {
            cwd.join("sdk")
        };
        let workspace_root = workspace_root.unwrap_or_else(|| sdk_root.join("workspace"));

        Self {
            sdk_root: sdk_root.display().to_string(),
            workspace_root: workspace_root.display().to_string(),
            toolchain: "nightly-2026-04-28".to_string(),
            modpack_idx: 0,
            status: "Idle".to_string(),
            log_text: String::new(),
            running: None,
        }
    }

    fn task_running(&self) -> bool {
        self.running.is_some()
    }

    fn append_log_line(&mut self, message: impl AsRef<str>) {
        self.log_text.push_str(message.as_ref());
        self.log_text.push('\n');
    }

    fn update_task_events(&mut self) {
        loop {
            let Some(task) = &self.running else {
                return;
            };

            let event = match task.rx.try_recv() {
                Ok(event) => event,
                Err(mpsc::TryRecvError::Empty) => return,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.status = "Task channel disconnected".to_string();
                    self.running = None;
                    return;
                }
            };

            match event {
                TaskEvent::Log(line) => self.append_log_line(line),
                TaskEvent::Done(result) => {
                    match result {
                        Ok(()) => {
                            self.status = format!("{} completed", task.label);
                            self.append_log_line(format!("== {} completed ==", task.label));
                        }
                        Err(err) => {
                            self.status = format!("{} failed", task.label);
                            self.append_log_line(format!("== {} failed: {} ==", task.label, err));
                        }
                    }
                    self.running = None;
                    return;
                }
            }
        }
    }

    fn build_config(&self) -> Result<LauncherConfig, String> {
        if BUILTIN_MODPACKS.is_empty() {
            return Err("no built-in modpacks configured".to_string());
        }
        if self.modpack_idx >= BUILTIN_MODPACKS.len() {
            return Err("selected modpack index is out of range".to_string());
        }

        let sdk_root = PathBuf::from(self.sdk_root.trim());
        if self.sdk_root.trim().is_empty() {
            return Err("SDK root is empty".to_string());
        }

        let workspace_root = PathBuf::from(self.workspace_root.trim());
        if self.workspace_root.trim().is_empty() {
            return Err("Workspace root is empty".to_string());
        }

        if self.toolchain.trim().is_empty() {
            return Err("Toolchain is empty".to_string());
        }

        Ok(LauncherConfig {
            sdk_root,
            workspace_root,
            toolchain: self.toolchain.trim().to_string(),
            selected_modpack: BUILTIN_MODPACKS[self.modpack_idx],
        })
    }

    fn start_task<F>(&mut self, label: &str, work: F)
    where
        F: FnOnce(Sender<TaskEvent>) -> Result<(), String> + Send + 'static,
    {
        if self.task_running() {
            return;
        }

        let label_owned = label.to_string();
        let (tx, rx) = mpsc::channel();
        self.append_log_line(format!("== starting task: {} ==", label_owned));
        self.status = format!("Running {}", label_owned);
        self.running = Some(RunningTask {
            label: label_owned.clone(),
            rx,
        });

        thread::spawn(move || {
            let result = work(tx.clone());
            let _ = tx.send(TaskEvent::Done(result));
        });
    }

    fn trigger_fetch(&mut self) {
        let cfg = match self.build_config() {
            Ok(cfg) => cfg,
            Err(err) => {
                self.status = "Invalid launcher config".to_string();
                self.append_log_line(format!("config error: {}", err));
                return;
            }
        };

        self.start_task("fetch-deps", move |tx| run_fetch(cfg, tx));
    }

    fn trigger_build(&mut self) {
        let cfg = match self.build_config() {
            Ok(cfg) => cfg,
            Err(err) => {
                self.status = "Invalid launcher config".to_string();
                self.append_log_line(format!("config error: {}", err));
                return;
            }
        };

        self.start_task("build-game", move |tx| run_build(cfg, tx));
    }

    fn trigger_run(&mut self) {
        let cfg = match self.build_config() {
            Ok(cfg) => cfg,
            Err(err) => {
                self.status = "Invalid launcher config".to_string();
                self.append_log_line(format!("config error: {}", err));
                return;
            }
        };

        self.start_task("run-game", move |tx| run_game(cfg, tx));
    }

    fn trigger_build_and_run(&mut self) {
        let cfg = match self.build_config() {
            Ok(cfg) => cfg,
            Err(err) => {
                self.status = "Invalid launcher config".to_string();
                self.append_log_line(format!("config error: {}", err));
                return;
            }
        };

        self.start_task("build-and-run", move |tx| {
            run_build(cfg.clone(), tx.clone())?;
            run_game(cfg, tx)
        });
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_task_events();
        ctx.request_repaint_after(Duration::from_millis(100));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Loo-Cast Launcher (MVP)");
            ui.label(format!("Status: {}", self.status));
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("SDK Root");
                ui.text_edit_singleline(&mut self.sdk_root);
            });
            ui.horizontal(|ui| {
                ui.label("Workspace Root");
                ui.text_edit_singleline(&mut self.workspace_root);
            });
            ui.horizontal(|ui| {
                ui.label("Toolchain");
                ui.text_edit_singleline(&mut self.toolchain);
            });

            ui.horizontal(|ui| {
                ui.label("Built-in Modpack");
                egui::ComboBox::from_id_salt("modpack_combo")
                    .selected_text(BUILTIN_MODPACKS[self.modpack_idx].display_name)
                    .show_ui(ui, |ui| {
                        for (idx, modpack) in BUILTIN_MODPACKS.iter().enumerate() {
                            ui.selectable_value(&mut self.modpack_idx, idx, modpack.display_name);
                        }
                    });
            });

            ui.collapsing("Selected Modpack Mods", |ui| {
                let modpack = BUILTIN_MODPACKS[self.modpack_idx];
                ui.monospace(format!("id: {}", modpack.id));
                for mod_id in modpack.mods {
                    ui.monospace(*mod_id);
                }
            });

            ui.separator();
            let can_click = !self.task_running();
            ui.horizontal(|ui| {
                if ui.add_enabled(can_click, egui::Button::new("Clean")).clicked() {
                    self.trigger_fetch();
                }
                if ui.add_enabled(can_click, egui::Button::new("Build")).clicked() {
                    self.trigger_build();
                }
                if ui.add_enabled(can_click, egui::Button::new("Run")).clicked() {
                    self.trigger_run();
                }
                if ui.add_enabled(can_click, egui::Button::new("Build + Run")).clicked() {
                    self.trigger_build_and_run();
                }
            });

            ui.separator();
            ui.label("Logs");
            ui.horizontal(|ui| {
                if ui.button("Copy All").clicked() {
                    ui.ctx().copy_text(self.log_text.clone());
                }
                if ui.button("Clear").clicked() {
                    self.log_text.clear();
                }
            });
            let viewport_height = ui.available_height().max(200.0);
            egui::Frame::default().show(ui, |ui| {
                ui.set_min_height(viewport_height);
                egui::ScrollArea::vertical()
                    .scroll_source(egui::containers::scroll_area::ScrollSource {
                        drag: false,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        let mut log_view = self.log_text.as_str();
                        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
                            let layout = build_ansi_log_layout(ui, text.as_str(), wrap_width);
                            ui.fonts_mut(|fonts| fonts.layout_job(layout))
                        };
                        ui.add(
                            egui::TextEdit::multiline(&mut log_view)
                                .font(egui::TextStyle::Monospace)
                                .horizontal_align(egui::Align::Min)
                                .desired_width(f32::INFINITY)
                                .desired_rows(1)
                                .layouter(&mut layouter)
                                .code_editor(),
                        );
                    });
            });
        });
    }
}

fn run_fetch(cfg: LauncherConfig, tx: Sender<TaskEvent>) -> Result<(), String> {
    ensure_sdk_dirs(&cfg)?;
    run_xtask(&cfg, &tx, &["clean"])
}

fn run_build(cfg: LauncherConfig, tx: Sender<TaskEvent>) -> Result<(), String> {
    ensure_sdk_dirs(&cfg)?;
    tx.send(TaskEvent::Log(format!(
        "building modpack '{}' [{} mods]",
        cfg.selected_modpack.id,
        cfg.selected_modpack.mods.len()
    )))
    .map_err(|err| err.to_string())?;

    run_xtask(&cfg, &tx, &["build"])
}

fn run_game(cfg: LauncherConfig, tx: Sender<TaskEvent>) -> Result<(), String> {
    ensure_sdk_dirs(&cfg)?;
    run_xtask(&cfg, &tx, &["debug"])
}

fn run_xtask(cfg: &LauncherConfig, tx: &Sender<TaskEvent>, args: &[&str]) -> Result<(), String> {
    let mut command_args = vec!["run".to_string(), "-p".to_string(), "xtask".to_string(), "--".to_string()];
    command_args.extend(args.iter().map(|arg| arg.to_string()));
    run_spec(
        &CommandSpec {
            program: cargo_path(cfg),
            args: command_args,
            cwd: cfg.workspace_root.clone(),
            env: sdk_env(cfg)?,
        },
        tx,
    )
}

fn run_spec(spec: &CommandSpec, tx: &Sender<TaskEvent>) -> Result<(), String> {
    let mut preview = spec.program.display().to_string();
    if !spec.args.is_empty() {
        preview.push(' ');
        preview.push_str(&spec.args.join(" "));
    }

    tx.send(TaskEvent::Log(format!("$ {}", preview))).map_err(|err| err.to_string())?;
    tx.send(TaskEvent::Log(format!("cwd: {}", spec.cwd.display()))).map_err(|err| err.to_string())?;

    let output = Command::new(&spec.program)
        .args(&spec.args)
        .current_dir(&spec.cwd)
        .envs(spec.env.iter().map(|(k, v)| (k, v)))
        .output()
        .map_err(|err| format!("failed to run '{}': {}", spec.program.display(), err))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        tx.send(TaskEvent::Log(stdout.to_string())).map_err(|err| err.to_string())?;
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        tx.send(TaskEvent::Log(stderr.to_string())).map_err(|err| err.to_string())?;
    }

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("command failed with status {}", output.status))
    }
}

fn ensure_sdk_dirs(cfg: &LauncherConfig) -> Result<(), String> {
    fs::create_dir_all(&cfg.sdk_root).map_err(|err| err.to_string())?;
    fs::create_dir_all(rustup_home(&cfg.sdk_root)).map_err(|err| err.to_string())?;
    fs::create_dir_all(cargo_home(&cfg.sdk_root)).map_err(|err| err.to_string())?;
    fs::create_dir_all(cfg.sdk_root.join("target")).map_err(|err| err.to_string())?;
    Ok(())
}

fn sdk_env(cfg: &LauncherConfig) -> Result<Vec<(String, OsString)>, String> {
    let rustup_home = rustup_home(&cfg.sdk_root);
    let cargo_home = cargo_home(&cfg.sdk_root);
    let cargo_bin = cargo_home.join("bin");
    let linker_bin = cfg.sdk_root.join("toolchains").join("llvm-mingw").join("bin");

    let mut path_entries = Vec::<PathBuf>::new();
    path_entries.push(cargo_bin);
    if linker_bin.is_dir() {
        path_entries.push(linker_bin);
    }
    if let Some(existing) = env::var_os("PATH") {
        path_entries.extend(env::split_paths(&existing));
    }
    let joined_path = env::join_paths(path_entries).map_err(|err| err.to_string())?;

    Ok(vec![
        ("RUSTUP_HOME".to_string(), rustup_home.into_os_string()),
        ("CARGO_HOME".to_string(), cargo_home.into_os_string()),
        ("CARGO_TARGET_DIR".to_string(), cfg.sdk_root.join("target").into_os_string()),
        ("RUSTUP_TOOLCHAIN".to_string(), OsString::from(cfg.toolchain.clone())),
        ("RUSTUP_AUTO_INSTALL".to_string(), OsString::from("0")),
        ("CARGO_TERM_COLOR".to_string(), OsString::from("always")),
        ("CLICOLOR_FORCE".to_string(), OsString::from("1")),
        ("PATH".to_string(), joined_path),
    ])
}

#[derive(Clone, Copy)]
struct AnsiStyle {
    fg: egui::Color32,
    bold: bool,
}

fn build_ansi_log_layout(ui: &egui::Ui, text: &str, wrap_width: f32) -> egui::text::LayoutJob {
    let font_id = egui::TextStyle::Monospace.resolve(ui.style());
    let default_color = ui.visuals().text_color();
    let mut style = AnsiStyle {
        fg: default_color,
        bold: false,
    };

    let mut layout = egui::text::LayoutJob::default();
    layout.wrap.max_width = wrap_width.max(10.0);
    layout.wrap.break_anywhere = false;
    layout.halign = egui::Align::Min;
    layout.justify = false;

    let bytes = text.as_bytes();
    let mut i = 0usize;
    let mut run_start = 0usize;
    while i < bytes.len() {
        if bytes[i] == 0x1b {
            if let Some((consumed, next_style)) = parse_ansi_escape(bytes, i, style, default_color) {
                if run_start < i {
                    let segment = &text[run_start..i];
                    layout.append(segment, 0.0, text_format_for_style(&font_id, style));
                }
                style = next_style;
                i += consumed;
                run_start = i;
                continue;
            }
        }
        i += 1;
    }

    if run_start < bytes.len() {
        let tail = &text[run_start..];
        layout.append(tail, 0.0, text_format_for_style(&font_id, style));
    }

    layout
}

fn text_format_for_style(font_id: &egui::FontId, style: AnsiStyle) -> egui::TextFormat {
    let mut format = egui::TextFormat {
        font_id: font_id.clone(),
        color: style.fg,
        ..Default::default()
    };
    if style.bold {
        format.italics = false;
        format.underline = egui::Stroke::NONE;
    }
    format
}

fn parse_ansi_escape(bytes: &[u8], start: usize, current: AnsiStyle, default_color: egui::Color32) -> Option<(usize, AnsiStyle)> {
    if start + 1 >= bytes.len() || bytes[start] != 0x1b || bytes[start + 1] != b'[' {
        return None;
    }

    let mut end = start + 2;
    while end < bytes.len() {
        let b = bytes[end];
        if (0x40..=0x7e).contains(&b) {
            break;
        }
        end += 1;
    }
    if end >= bytes.len() {
        return None;
    }

    let final_byte = bytes[end];
    let consumed = end + 1 - start;
    if final_byte != b'm' {
        return Some((consumed, current));
    }

    let params_bytes = &bytes[start + 2..end];
    let params = parse_sgr_params(params_bytes);
    let next = apply_sgr(params.as_slice(), current, default_color);
    Some((consumed, next))
}

fn parse_sgr_params(bytes: &[u8]) -> Vec<i32> {
    if bytes.is_empty() {
        return vec![0];
    }
    let raw = String::from_utf8_lossy(bytes);
    let mut out = Vec::new();
    for part in raw.split(';') {
        if part.is_empty() {
            out.push(0);
        } else if let Ok(value) = part.parse::<i32>() {
            out.push(value);
        }
    }
    if out.is_empty() {
        out.push(0);
    }
    out
}

fn apply_sgr(params: &[i32], mut style: AnsiStyle, default_color: egui::Color32) -> AnsiStyle {
    let mut i = 0usize;
    while i < params.len() {
        match params[i] {
            0 => {
                style = AnsiStyle {
                    fg: default_color,
                    bold: false,
                };
            }
            1 => style.bold = true,
            22 => style.bold = false,
            39 => style.fg = default_color,
            30..=37 => style.fg = ansi_16_color((params[i] - 30) as usize, false),
            90..=97 => style.fg = ansi_16_color((params[i] - 90) as usize, true),
            38 => {
                if i + 2 < params.len() && params[i + 1] == 5 {
                    let idx = clamp_to_u8(params[i + 2]);
                    style.fg = ansi_256_color(idx);
                    i += 2;
                } else if i + 4 < params.len() && params[i + 1] == 2 {
                    let r = clamp_to_u8(params[i + 2]);
                    let g = clamp_to_u8(params[i + 3]);
                    let b = clamp_to_u8(params[i + 4]);
                    style.fg = egui::Color32::from_rgb(r, g, b);
                    i += 4;
                }
            }
            _ => {}
        }
        i += 1;
    }
    style
}

fn ansi_16_color(index: usize, bright: bool) -> egui::Color32 {
    const NORMAL: [(u8, u8, u8); 8] = [
        (0, 0, 0),
        (170, 0, 0),
        (0, 170, 0),
        (170, 85, 0),
        (0, 0, 170),
        (170, 0, 170),
        (0, 170, 170),
        (170, 170, 170),
    ];
    const BRIGHT: [(u8, u8, u8); 8] = [
        (85, 85, 85),
        (255, 85, 85),
        (85, 255, 85),
        (255, 255, 85),
        (85, 85, 255),
        (255, 85, 255),
        (85, 255, 255),
        (255, 255, 255),
    ];

    let palette = if bright { &BRIGHT } else { &NORMAL };
    let (r, g, b) = palette[index.min(7)];
    egui::Color32::from_rgb(r, g, b)
}

fn ansi_256_color(index: u8) -> egui::Color32 {
    match index {
        0..=7 => ansi_16_color(index as usize, false),
        8..=15 => ansi_16_color((index - 8) as usize, true),
        16..=231 => {
            let value = index - 16;
            let r = value / 36;
            let g = (value % 36) / 6;
            let b = value % 6;
            let to_rgb = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            egui::Color32::from_rgb(to_rgb(r), to_rgb(g), to_rgb(b))
        }
        232..=255 => {
            let gray = 8 + ((index - 232) * 10);
            egui::Color32::from_rgb(gray, gray, gray)
        }
    }
}

fn clamp_to_u8(value: i32) -> u8 {
    value.clamp(0, 255) as u8
}

fn cargo_path(cfg: &LauncherConfig) -> PathBuf {
    cargo_home(&cfg.sdk_root).join("bin").join(bin_name("cargo"))
}

fn rustup_home(sdk_root: &Path) -> PathBuf {
    sdk_root.join("rustup-home")
}

fn cargo_home(sdk_root: &Path) -> PathBuf {
    sdk_root.join("cargo-home")
}

fn bin_name(name: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{}.exe", name)
    } else {
        name.to_string()
    }
}

fn find_workspace_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|path| path.join("Cargo.toml").is_file() && path.join("crates").is_dir())
        .map(Path::to_path_buf)
}
