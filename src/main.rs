extern crate cargo_metadata;

extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use cargo_metadata::{diagnostic, parse_messages, Message};
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

use notify_rust::Notification;

use std::collections::HashMap;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(2).collect();
    Notification::new()
        .summary("Cargo Notifier")
        .subtitle(&format!("🚧 Running {}", args.join(" ")))
        .show()
        .unwrap();
    let mut command = Command::new("cargo")
        .args(args)
        .arg("--message-format")
        .arg("json-diagnostic-rendered-ansi")
        .arg("-q")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut errors: HashMap<String, usize> = HashMap::new();
    for message in parse_messages(command.stdout.take().unwrap()) {
        match message {
            Ok(Message::CompilerMessage(msg)) => {
                if let Some(rendered) = msg.message.rendered {
                    eprintln!("{}\n", rendered);
                }
                match msg.message.level {
                    diagnostic::DiagnosticLevel::Error => {
                        *errors.entry(msg.target.name).or_insert(0) += 1
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    let mut errors_list: Vec<_> = errors.iter().collect();
    errors_list.sort_by(|a, b| b.1.cmp(a.1));

    let mut error_subtitle = format!("✅ 0 Errors ");
    let mut error_message = "Command succeeded".to_string();
    if errors_list.len() > 0 {
        let total_errors = errors_list.iter().fold(0_usize, |acc, x| acc + *x.1);
        error_subtitle = format!("❌ {} Errors ", total_errors);
        error_message = format!("{} errors in {}", errors_list[0].1, errors_list[0].0);
    }
    Notification::new()
        .summary("Cargo Notifier")
        .subtitle(&error_subtitle)
        .body(&error_message)
        .show()
        .unwrap();
    Ok(())
}
