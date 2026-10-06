//! Prompt editing and bounded command history.

const MAX_HISTORY: usize = 128;

#[derive(Default)]
pub(in crate::console) struct ConsolePrompt {
    pub(super) input: String,
    history: Vec<String>,
    history_cursor: Option<usize>,
}

impl ConsolePrompt {
    pub(super) fn reset_navigation(&mut self) {
        self.history_cursor = None;
    }
    pub(super) fn submit(&mut self) -> Option<String> {
        let command = self.input.trim().to_string();
        self.input.clear();
        self.history_cursor = None;
        if command.is_empty() {
            return None;
        }

        if self.history.last() != Some(&command) {
            self.history.push(command.clone());
            if self.history.len() > MAX_HISTORY {
                self.history.remove(0);
            }
        }

        Some(command)
    }

    pub(super) fn history_up(&mut self) {
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

    pub(super) fn history_down(&mut self) {
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
