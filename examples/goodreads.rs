#[macro_use]
extern crate html5ever;
extern crate toks;

use toks::prelude::*;

use std::io::{self, Read};

pub struct TitleTok;

impl Tok for TitleTok {
    fn is_match(&self, qn: &QualName) -> bool {
        qn.local == local_name!("a")
    }

    fn process(&mut self, attrs: &mut Vec<Attribute>, _: &mut Vec<Handle>) {
        for attr in attrs.iter() {
            if attr.name.local == local_name!("title") {
                let title = attr.value.to_string();
                if let Some(first_letter) = title.get(0..1) {
                    if first_letter == first_letter.to_uppercase() && !title.starts_with("View all")
                    {
                        println!("{}", title);
                    }
                }
            }
        }
    }
}

// How to use
// $ cat your.html | cargo run --example goodreads
fn main() {
    let mut chunk = String::new();
    io::stdin().read_to_string(&mut chunk).unwrap();

    let dom = parse_document(RcDom::default(), Default::default()).one(chunk);

    let mut lt = TitleTok;

    // Dropping mut reference
    {
        recursion(&mut vec![&mut lt], dom.document);
    }
}
