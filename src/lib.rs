use log::info;
use std::time::Instant;

pub mod gitemail;
pub mod passgen;
pub mod pdfcrop;
pub mod pdfembed;
pub mod plain_photos;
pub mod semver;
pub mod vp9;
pub mod wifiqr;

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
