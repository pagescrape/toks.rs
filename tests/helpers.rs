extern crate html5ever;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{RcDom, Handle};
use self::html5ever::tendril::{StrTendril, TendrilSink};

pub fn files_out13_handle() -> Handle {
    parse_document(RcDom::default(), Default::default())
        .one(StrTendril::from(include_str!("./files/out13.com.html")))
        .document
}
