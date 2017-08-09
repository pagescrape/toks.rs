extern crate toks;
#[macro_use]
extern crate html5ever;

use toks::{Tok, recursion};
use toks::prelude::*;
mod helpers;

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

#[test]
fn test_link_tok() {
    let doc = helpers::files_out13_handle();

    let mut lt = LinkTok { total: 0 };

    {
        recursion(&mut vec![&mut lt], doc);
    }

    assert_eq!(lt.total, 55);
}
