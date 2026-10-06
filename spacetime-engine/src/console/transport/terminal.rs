//! Native stdin and terminal record output.

use super::{
    queue::ConsoleTransport,
    record::{ConsoleRecord, ConsoleRecordKind},
};
use crate::console::ConsoleCommandSource;
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::{
    io::{BufRead, IsTerminal, Write},
    thread,
};

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::console) fn start_terminal_input(transport: Res<ConsoleTransport>) {
    let transport = transport.clone();

    if let Err(error) = thread::Builder::new()
        .name("spacetime-console-stdin".to_string())
        .spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(line) => transport.submit(ConsoleCommandSource::Terminal, line),
                    Err(error) => {
                        bevy::log::error!(
                            target: "developer_console",
                            error = %error,
                            "stdin console frontend stopped"
                        );
                        break;
                    }
                }
            }
        })
    {
        bevy::log::error!(
            target: "developer_console",
            error = %error,
            "failed to spawn stdin console frontend"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::console) fn write_terminal_record(record: &ConsoleRecord) {
    match record.kind {
        ConsoleRecordKind::Clear => {
            let stdout = std::io::stdout();
            if stdout.is_terminal() {
                let mut stdout = stdout.lock();
                let _ = write!(stdout, "\x1b[2J\x1b[H");
                let _ = stdout.flush();
            }
        }
        ConsoleRecordKind::Error => {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            let _ = writeln!(stderr, "{}", record.text);
        }
        ConsoleRecordKind::Command => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let source = record.source.map_or("command", ConsoleCommandSource::label);
            let _ = writeln!(stdout, "[{source}] {}", record.text);
        }
        ConsoleRecordKind::Output => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let _ = writeln!(stdout, "{}", record.text);
        }
        ConsoleRecordKind::Trace(_) => {
            // Bevy's normal tracing formatter already owns terminal log output.
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::console) fn write_terminal_record(_: &ConsoleRecord) {}
