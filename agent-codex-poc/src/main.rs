mod agent;

use clap::Parser;
use std::io;
use std::process::Command;

/// Command-line arguments for the Codex agent wrapper.
#[derive(Parser)]
#[command(author, version, about = "Wraps the Codex CLI into a simple agent entry point.")]
struct CliArgs {
    /// Codex model to target.
    #[arg(long, default_value = "gpt-5.1-codex-mini")]
    model: String,

    /// Prompt text that should be executed by Codex. Wrap multi-word prompts in quotes.
    #[arg(value_name = "PROMPT", num_args = 1..)]
    prompt: Vec<String>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("agent-codex-poc: {err}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args = CliArgs::parse();
    let prompt = args.prompt.join(" ");
    let (executable, cmd_args) = agent::build_command(&args.model, &prompt);
    let display_args = cmd_args.join(" ");
    println!("Running agent command: {} {}", executable, display_args);

    let status = Command::new(&executable).args(&cmd_args).status()?;
    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    Ok(())
}
