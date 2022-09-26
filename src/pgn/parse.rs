use std::io::Read;
use fallible_iterator::FallibleIterator;
use pest::iterators::Pair;
use crate::pgn::PgnGame;
use crate::sak::sak_error::*;

mod parse_impl {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "../pest/pgn.pest"]
    pub(super) struct ParseImpl;
}

pub(crate) type ParseRule = parse_impl::Rule;
pub(crate) type ParsePair<'i> = Pair<'i, ParseRule>;

pub struct ParseResult<R: Read> {
    reader: R,
}

impl<R: Read> FallibleIterator for ParseResult<R> {
    type Item = PgnGame;
    type Error = SakError;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        // use pest::Parser;
        // use parse_impl::ParseImpl;

        // TODO: Read from reader in to str buffer
        // TODO: Detect EOF and return Ok(None)
        unimplemented!();

        // let parse_str = "";
        // let parse_pair = ParseImpl::parse(ParseRule::pgn_game, parse_str)?.next().unwrap();

        // Ok(Some(PgnGame::from(parse_pair)))
    }
}

pub fn pgn_parse<R: Read>(reader: R) -> ParseResult<R> {
    ParseResult {
        reader,
    }
}