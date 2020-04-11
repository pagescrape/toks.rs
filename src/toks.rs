use crate::tok::Tok;
use markup5ever_rcdom::{Handle, NodeData};

/// Toks convenience type alias to Vec of Tok's.
pub type Toks<'s> = Vec<&'s mut dyn Tok>;

/// Helper function which walks through `html5ever::rcdom::Handle`
/// `NodeData::Element` branch recursively and fires `Tok``process`
/// function if `QualName` is found by `is_match`.
pub fn recursion(toks: &mut Toks, handle: Handle) {
    if let NodeData::Element { name, attrs, .. } = &handle.data {
        for tok in toks.iter_mut() {
            if tok.is_match(&name) {
                tok.process(&mut attrs.borrow_mut(), &mut handle.children.borrow_mut())
            }
        }
    }

    for child in handle.children.borrow_mut().iter() {
        recursion(toks, child.clone());
    }
}
