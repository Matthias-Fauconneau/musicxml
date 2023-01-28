mod xml {
    pub use roxmltree::{Document, Node};
    pub type Result<T> = std::result::Result<T, roxmltree::Error>;
    pub fn find<'t, 'i>(e: Node<'t, 'i>, name: &str) -> Option<Node<'t, 'i>> { e.children().find(|c| c.tag_name().name() == name) }
    pub fn has(e: Node<'_, '_>, name: &'static str) -> bool { find(e, name).is_some() }
    pub fn filter<'t, 'i>(e: Node<'t, 'i>, name: &'static str) -> impl Iterator<Item=Node<'t, 'i>> { e.children().filter(move |e| e.tag_name().name() == name) }
    pub fn count(e: Node<'_, '_>, name: &'static str) -> u8 { filter(e, name).count().try_into().unwrap() }
}
pub use xml::{Result, Document, Node, has, count};

pub trait FromStr { fn from_str(s: &str) -> Self; }
impl<T:std::str::FromStr> FromStr for T where <T as std::str::FromStr>::Err: std::fmt::Debug { #[track_caller] fn from_str(s: &str) -> Self { s.parse().expect(s) } }

pub fn try_attribute<T:FromStr>(e: Node<'_, '_>, name: &'static str) -> Option<T> { Some(FromStr::from_str(e.attribute(name)?)) }
#[track_caller] pub fn attribute<T:FromStr>(e: Node<'_, '_>, name: &'static str) -> T { try_attribute(e, name).expect(name) }

pub trait FromElement : Sized {
    #[track_caller] fn try_from<'t, 'input>(element: Node<'t, 'input>) -> Option<Self> { Some(Self::from(element)) }
    #[track_caller] fn from<'t, 'input>(element: Node<'t, 'input>) -> Self { Self::try_from(element).expect(&format!("{}",element.tag_name().name()))/*unwrap_or_else(|| panic!("{}", element.tag_name().name()))*/ }
}
impl<T:FromStr> FromElement for T { fn try_from<'t, 'input>(e: Node<'t, 'input>) -> Option<Self> { Some(FromStr::from_str(e.text()?)) } } // prevents impl for Box<T>

#[track_caller] pub fn find<T:FromElement>(e: Node<'_, '_>, name: &'static str) -> T { T::from(xml::find(e, name).expect(name)) }
pub fn option<T:FromElement>(e: Node<'_, '_>, name: &'static str) -> Option<T> { T::try_from(xml::find(e, name)?) }
pub fn seq<T:FromElement>(e: Node<'_, '_>) -> Box<[T]> { e.children().filter(|e| e.is_element()).filter_map(|e| T::try_from(e)).collect() }
pub fn find_seq<T:FromElement>(e: Node<'_, '_>, name: &'static str) -> Box<[T]> { xml::find(e, name).map(|e| seq(e)).unwrap_or_default() }
pub fn filter<T:FromElement>(e: Node<'_, '_>, name: &'static str) -> Box<[T]> { xml::filter(e, name).map(|e| T::from(e)).collect() }
pub fn filter_seq<T:FromElement>(e: Node<'_, '_>, name: &'static str) -> Box<[Box<[T]>]> { xml::filter(e, name).map(|e| seq(e)).collect() }
pub fn find_filter_seq<T:FromElement>(e: Node<'_, '_>, name: &'static str, filter: &'static str) -> Box<[Box<[T]>]> { filter_seq(xml::find(e, name).expect(name), filter) }