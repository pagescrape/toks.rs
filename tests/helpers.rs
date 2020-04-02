use html5ever::parse_document;
use html5ever::tendril::{StrTendril, TendrilSink};
use markup5ever_rcdom::{Handle, RcDom};
use toks::prelude::*;

pub fn files_out13_handle() -> Handle {
    parse_document(RcDom::default(), Default::default())
        .one(StrTendril::from(include_str!("./files/out13.com.html")))
        .document
}
