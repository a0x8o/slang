use std::{fmt::Write, path::PathBuf};

pub struct MarkdownWriter {
    w: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        return Self { w: String::new() };
    }

    pub fn to_string(self) -> String {
        return self.w;
    }

    pub fn write_header(&mut self, level: usize, value: &str) {
        writeln!(self.w, "{prefix} {value}", prefix = "#".repeat(level)).unwrap();
    }

    pub fn write_comment(&mut self, value: &str) {
        writeln!(self.w, "<!-- {value} -->").unwrap();
    }

    pub fn write_snippet(&mut self, repo_root: &PathBuf, snippet_path: &PathBuf) {
        writeln!(
            self.w,
            "--8<-- \"{snippet_path}\"",
            snippet_path = snippet_path
                .strip_prefix(repo_root)
                .unwrap()
                .to_str()
                .unwrap()
        )
        .unwrap();
    }

    pub fn write_line(&mut self, value: &str) {
        writeln!(self.w, "{value}").unwrap();
    }

    pub fn write_newline(&mut self) {
        writeln!(self.w).unwrap();
    }
}
