use crate::pgn::{ParsePair, ParseRule};
use crate::pgn::tag_pair::*;

// TODO: Allocate tag_pairs with some initial capacity. 18-32?
// TODO: Movetext parsing/validation
pub struct PgnGame {
    tag_pairs: TagPairs,
    movetext: String,
}

impl PgnGame {
    pub fn from(pair: ParsePair) -> PgnGame {
        assert_eq!(pair.as_rule(), ParseRule::pgn_game);

        let mut pairs = pair.into_inner();
        let mut tag_pairs = TagPairs::new();
        let mut movetext = String::new();

        PgnGame::populate_tag_pairs(&mut tag_pairs, pairs.next().unwrap());
        PgnGame::populate_movetext(&mut movetext, pairs.next().unwrap());

        PgnGame {
            tag_pairs,
            movetext
        }
    }

    pub fn clone_from(&mut self, pair: ParsePair) {
        assert_eq!(pair.as_rule(), ParseRule::pgn_game);

        let mut pairs = pair.into_inner();

        PgnGame::populate_tag_pairs(&mut self.tag_pairs, pairs.next().unwrap());
        PgnGame::populate_movetext(&mut self.movetext, pairs.next().unwrap());
    }

    pub fn event(&self) -> TagValueRef {
        &self.tag_pairs[0]
    }

    pub fn site(&self) -> TagValueRef {
        &self.tag_pairs[1]
    }

    pub fn date(&self) -> TagValueRef {
        &self.tag_pairs[2]
    }

    pub fn round(&self) -> TagValueRef {
        &self.tag_pairs[3]
    }

    pub fn white(&self) -> TagValueRef {
        &self.tag_pairs[4]
    }

    pub fn black(&self) -> TagValueRef {
        &self.tag_pairs[5]
    }

    pub fn result(&self) -> TagValueRef {
        &self.tag_pairs[6]
    }

    pub fn seven_tag_roster(&self) -> impl Iterator<Item=TagPairsIterItem> {
        self.tag_pairs.iter().take(7)
    }

    pub fn tag_pair(&self, tag_name: impl AsRef<str>) -> Option<TagPairRef> {
        self.tag_pairs.get_key_value(tag_name.as_ref())
    }

    pub fn tag_value(&self, tag_name: impl AsRef<str>) -> Option<TagValueRef> {
        self.tag_pairs.get(tag_name.as_ref())
    }

    pub fn tag_pairs(&self) -> impl Iterator<Item=TagPairsIterItem> {
        self.tag_pairs.iter()
    }

    pub fn optional_tag_pairs(&self) -> impl Iterator<Item=TagPairsIterItem> {
        self.tag_pairs.iter().skip(7)
    }

    // TODO: TagValueRef?
    pub fn movetext(&self) -> TagValueRef {
        &self.movetext
    }

    fn populate_tag_pairs(tag_pairs: &mut TagPairs, pair: ParsePair) {
        let mut pairs = pair.into_inner();

        tag_pairs.clear();
        PgnGame::populate_seven_tag_roster(tag_pairs, pairs.next().unwrap());
        PgnGame::populate_optional_tags(tag_pairs, pairs.next());
    }

    fn populate_movetext(movetext: &mut String, pair: ParsePair) {
        movetext.clear();
        movetext.push_str(pair.as_str());
    }

    fn populate_seven_tag_roster(tag_pairs: &mut TagPairs, pair: ParsePair) {

        fn populate(tag_pairs: &mut TagPairs, tag_name: &str, pair: ParsePair) {
            let tag_value = pair.into_inner().next().unwrap().as_str();
            tag_pairs.insert(tag_name.into(), tag_value.into());
        }
        
        let mut pairs = pair.into_inner();

        populate(tag_pairs, "Event", pairs.next().unwrap());
        populate(tag_pairs, "Site", pairs.next().unwrap());
        populate(tag_pairs, "Date", pairs.next().unwrap());
        populate(tag_pairs, "Round", pairs.next().unwrap());
        populate(tag_pairs, "White", pairs.next().unwrap());
        populate(tag_pairs, "Black", pairs.next().unwrap());
        populate(tag_pairs, "Result", pairs.next().unwrap());
    }

    fn populate_optional_tags(tag_pairs: &mut TagPairs, pair_opt: Option<ParsePair>) {
        if let Some(pair) = pair_opt {
            for opt_tag in pair.into_inner() {
                let mut pairs = opt_tag.into_inner();
                let tag_name = String::from(pairs.next().unwrap().as_str());
                let tag_value = String::from(pairs.next().unwrap().as_str());
                tag_pairs.insert(tag_name, tag_value);
            }
        }
    }
}

impl Default for PgnGame {
    fn default() -> Self {
        let mut tag_pairs = TagPairs::with_capacity(7);
        let uninitialized = String::from("Uninitialized");
        let movetext = uninitialized.clone();

        // Seven tag roster getters won't panic
        for _ in 1..=7 {
            tag_pairs.insert(uninitialized.clone(), uninitialized.clone());
        }

        PgnGame {
            tag_pairs,
            movetext,
        }
    }
}
