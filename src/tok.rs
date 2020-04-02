use html5ever::interface::Attribute;
use html5ever::QualName;
use markup5ever_rcdom::Handle;
use std::cell::RefCell;

/// Tok in short for token which is used to recursively walk
/// through `rcdom::Handle` when `QualName` match is found
/// `process` function is called.
///
/// Combining Tok's into Vec makes it advantageous to walk HTML DOM
/// only once and collect all interested elements.
pub trait Tok {
    /// Matcher function which matches predefined QualName.
    ///
    /// Depending on the result of boolean match this
    /// allows process function to proceed.
    fn is_match(&self, qual_name: &QualName) -> bool;

    /// Process function gets matched QualName attributes and children of
    /// matched element.
    fn process(&mut self, attributes: RefCell<Vec<Attribute>>, children: RefCell<Vec<Handle>>);
}
