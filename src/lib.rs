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
//!     fn process(&mut self, _: &mut Vec<Attribute>, _: &mut Vec<Handle>) {
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
#![deny(
    warnings,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    deprecated,
    unconditional_recursion,
    unknown_lints,
    unreachable_code,
    unused_mut
)]

mod tok;
mod toks;

pub use crate::toks::{recursion, Toks};
pub use tok::Tok;

/// Prelude module contains several important traits that provide many
/// of the convenience imports in advance.
pub mod prelude {
    pub use html5ever;
    #[doc(no_inline)]
    pub use html5ever::interface::Attribute;
    pub use html5ever::tendril::TendrilSink;
    pub use html5ever::{parse_document, QualName};
    pub use markup5ever_rcdom::{Handle, NodeData, RcDom};
    pub use std::cell::RefCell;

    // Re-export
    pub use crate::{recursion, Tok, Toks};
}
