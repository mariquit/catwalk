// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>

mod setup;
mod tui;

use catwalk::CatwalkError;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let text = err.to_string();
            if text.contains('\n') {
                eprintln!("{text}");
            } else {
                eprintln!("catwalk: {text}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), CatwalkError> {
    let mut path = env::var_os("CATWALK_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("TODO.md"));
    let mut args = env::args().skip(1);
    let mut command = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--file" || arg == "-f" {
            path = PathBuf::from(
                args.next()
                    .ok_or_else(|| CatwalkError::Message("--file requires a path".into()))?,
            );
        } else if let Some(value) = arg.strip_prefix("--file=") {
            path = PathBuf::from(value);
        } else {
            command.push(arg);
        }
    }
    match command.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        [] => tui::run(&path).map_err(CatwalkError::from),
        ["setup"] => setup::run("opencode"),
        ["setup", target] => setup::run(target),
        ["--version" | "-V"] => {
            println!("catwalk v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        ["help" | "--help" | "-h"] => {
            println!("\
Catwalk: Terminal viewer for agent-owned TODO lists
Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>

Usage:
  catwalk [--file PATH]  View an existing TODO.md and add human annotations
  catwalk setup [AGENT]  Install the guidance skill (default: opencode)

Agents: opencode, claude, codex, cursor
PATH defaults to TODO.md, or $CATWALK_FILE.

The agent creates and manages TODO.md directly, including task IDs, status,
priority, and Git commits. Catwalk does not create tasks or manage their progress.

Getting started:
  The catwalk skill is not loaded automatically, and Catwalk does not create TODO.md.

  1. Run `catwalk setup` for your agent.
  2. Ask your agent to load the skill first, for example:
       load the catwalk skill
       load the catwalk skill before you create a todo list
  3. Ask your agent to create a todo list.
  4. Run `catwalk` to view it and leave annotations.

  See GETTING_STARTED.md.");
            Ok(())
        }
        _ => Err(CatwalkError::Message(
            "unknown command. Use catwalk --help. Agents manage TODO.md directly, not through the Catwalk CLI.".into(),
        )),
    }
}
