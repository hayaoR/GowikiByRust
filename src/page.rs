use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Page {
    pub title: String,
    pub body: Vec<u8>,
}

impl Page {
    pub fn new(title: String, body: Vec<u8>) -> Self {
        Self { title, body }
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        let filename = self.title.clone() + ".txt";
        let mut file = File::create(filename)?;
        file.write_all(&self.body)?;
        Ok(())
    }
}

pub fn load_page(title: &str) -> std::io::Result<Page> {
    let filename = format!("{}.txt", title);
    let mut file = File::open(filename)?;
    let mut page = Page::new(title.to_string(), vec![]);
    file.read_to_end(&mut page.body)?;
    Ok(page)
}
