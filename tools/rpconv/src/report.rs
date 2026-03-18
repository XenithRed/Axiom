use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Severity { Info, Warn, Error }

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub a: Severity,
    pub b: String,
}

#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub converted: u32,
    pub issues: Vec<Issue>,
}

impl Report {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, i: Issue) { self.issues.push(i); }

    pub fn errors(&self) -> usize {
        self.issues.iter().filter(|i| i.a == Severity::Error).count()
    }

    pub fn warnings(&self) -> usize {
        self.issues.iter().filter(|i| i.a == Severity::Warn).count()
    }

    pub fn print(&self) {
        println!();
        println!("─────────────────────────────────────────────");
        println!("  Conversion complete");
        println!("  Models converted : {}", self.converted);
        println!("  Warnings         : {}", self.warnings());
        println!("  Errors           : {}", self.errors());
        if !self.issues.is_empty() {
            println!();
            for issue in &self.issues {
                let tag = match issue.a {
                    Severity::Info  => "[info ]",
                    Severity::Warn  => "[warn ]",
                    Severity::Error => "[error]",
                };
                println!("  {tag} {}", issue.b);
            }
        }
        println!("─────────────────────────────────────────────");
    }
}
