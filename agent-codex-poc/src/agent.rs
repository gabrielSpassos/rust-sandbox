/// Helpers for building a Codex CLI invocation.
pub fn build_command(model: &str, prompt: &str) -> (String, Vec<String>) {
    (
        "codex".to_string(),
        vec![
            "exec".to_string(),
            "--full-auto".to_string(),
            "-m".to_string(),
            model.to_string(),
            prompt.to_string(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::build_command;

    #[test]
    fn builder_includes_flags_and_prompt() {
        let (cmd, args) = build_command("gpt-5.4", "Hello Codex");
        assert_eq!(cmd, "codex");
        assert_eq!(
            args,
            vec![
                "exec",
                "--full-auto",
                "-m",
                "gpt-5.1-codex-mini",
                "Hello Codex"
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
        );
    }
}
