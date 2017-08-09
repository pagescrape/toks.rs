//! Efficient tokens for `rcdom::RcDom` `Handle` parsing
//! aiming for O(1) HTML DOM walking.
//!
//! This library aims to provide convenient and efficient handling of
//! HTML DOM elements.
//!
//! ## Examples
//!
//!```
//! extern crate toks;
//! #[macro_use]
//! extern crate html5ever;
//!
//! use html5ever::parse_document;
//! use html5ever::rcdom::{RcDom, Handle};
//! use html5ever::tendril::TendrilSink;
//!
//! use toks::{Tok, recursion};
//! use toks::prelude::*;
//! use std::io::{self, Read};
//!
//! pub struct LinkTok {
//!     total: u32,
//! }
//!
//! impl Tok for LinkTok {
//!     fn is_match(&self, qn: &QualName) -> bool {
//!         qn.local == local_name!("a")
//!     }
//!
//!     fn process(&mut self, _: RefCell<Vec<Attribute>>, _: RefCell<Vec<Handle>>) {
//!         self.total += 1;
//!     }
//! }
//!
//! // How to use
//! // $ cargo build --example count_links
//! // $ cat your.html | ./target/debug/examples/count_links
//! // Link <a> count 9
//! fn main() {
//!     let mut chunk = String::new();
//!     io::stdin().read_to_string(&mut chunk).unwrap();
//!
//!     let dom = parse_document(RcDom::default(), Default::default()).one(chunk);
//!
//!     let mut lt = LinkTok { total: 0 };
//!
//!     // Dropping mut reference
//!     {
//!         recursion(&mut vec![&mut lt], dom.document);
//!     }
//!
//!     println!("Link <a> count {}", lt.total);
//! }
//!```
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]

extern crate html5ever;

mod tok;
mod toks;

pub use tok::Tok;
pub use toks::*;

/// Prelude module contains several important traits that provide many
/// of the convenience imports in advance.
pub mod prelude {
    #[doc(no_inline)]
    pub use html5ever::interface::Attribute;
    pub use html5ever::QualName;
    pub use html5ever::rcdom::Handle;
    pub use std::cell::RefCell;
}
