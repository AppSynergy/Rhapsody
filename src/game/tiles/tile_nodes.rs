pub trait NodeTraits {
    fn children(&self) -> u8;
    fn distance(&self) -> u8;
}
