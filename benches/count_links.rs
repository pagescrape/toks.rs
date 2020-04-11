#![feature(test)]
extern crate test;
extern crate toks;
#[macro_use]
extern crate html5ever;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{Handle, RcDom};
use self::html5ever::tendril::{StrTendril, TendrilSink};
use test::Bencher;
use toks::prelude::*;
use toks::{recursion, Tok};

pub struct LinkTok {
    total: u32,
}

impl Tok for LinkTok {
    fn is_match(&self, qn: &QualName) -> bool {
        qn.local == local_name!("a")
    }

    fn process(&mut self, _: &mut Vec<Attribute>, _: &mut Vec<Handle>) {
        self.total += 1;
    }
}

#[bench]
fn bench_count_links(b: &mut Bencher) {
    let document = parse_document(RcDom::default(), Default::default())
        .one(StrTendril::from(include_str!(
            "../tests/files/out13.com.html"
        )))
        .document;

    let mut lt = LinkTok { total: 0 };

    b.iter(|| {
        recursion(&mut vec![&mut lt], document.clone());
    })
}
