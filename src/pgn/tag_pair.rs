use indexmap::{IndexMap, map::Iter as IndexMapIter};

pub(crate) type TagName = String;
pub(crate) type TagNameRef<'a> = &'a String;
pub(crate) type TagValue = String;
pub(crate) type TagValueRef<'a> = &'a String;
pub(crate) type TagPair = (TagName, TagValue);
pub(crate) type TagPairRef<'a> = (TagNameRef<'a>, TagValueRef<'a>);
pub(crate) type TagPairs = IndexMap<TagName, TagValue>;
pub(crate) type TagPairsIter<'a> = IndexMapIter<'a, TagName, TagValue>;
pub(crate) type TagPairsIterItem<'a> = <TagPairsIter<'a> as Iterator>::Item;