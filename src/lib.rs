use log::info;
use std::time::Instant;

pub mod gitemail;
pub mod passgen;
pub mod pdfcrop;
pub mod pdfembed;
pub mod plain_photos;
pub mod semver;
pub mod wifiqr;

#[macro_export]
// From: https://docs.rs/once_cell/latest/once_cell/#lazily-compiled-regex
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub struct Section {
    pub name: &'static str,
    pub start: Instant,
}

impl Section {
    fn new(name: &'static str) -> Section {
        info!("===> {}", name);
        let start = Instant::now();
        Section { name, start }
    }
}

impl Drop for Section {
    fn drop(&mut self) {
        info!("     {}: {:.2?}", self.name, self.start.elapsed());
    }
}
