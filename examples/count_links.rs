#[macro_use]
extern crate html5ever;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;

use std::io::{self, Read};
use toks::prelude::*;

pub struct LinkTok {
    total: u32,
}

impl Tok for LinkTok {
    fn is_match(&self, qn: &QualName) -> bool {
        qn.local == local_name!("a")
    }

    fn process(&mut self, _: RefCell<Vec<Attribute>>, _: RefCell<Vec<Handle>>) {
        self.total += 1;
    }
}

// How to use
// $ cargo build --example count_links
// $ cat your.html | ./target/debug/examples/count_links
// Link <a> count 9
fn main() {
    let mut chunk = String::new();
    io::stdin().read_to_string(&mut chunk).unwrap();

    let dom = parse_document(RcDom::default(), Default::default()).one(chunk);

    let mut lt = LinkTok { total: 0 };

    // Dropping mut reference
    {
        recursion(&mut vec![&mut lt], dom.document);
    }

    println!("Link <a> count {}", lt.total);
}
