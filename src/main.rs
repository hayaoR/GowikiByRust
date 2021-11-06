use gowiki::page::{Page, load_page};

fn main() {
   let mut p1 = Page::new("TestPage".to_string(), b"This is a sample Page.".to_vec());
   p1.save().expect("Can not save a file");

   let page = load_page("TestPage").expect("Can not load a page");
   println!("{}", String::from_utf8(page.body).expect("Can not convert to String from Vec<u8>"));
}
