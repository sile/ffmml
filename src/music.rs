use crate::{
    channel::{Channel, ChannelName},
    definitions::LineString,
    macros::Macros,
};
use std::{collections::BTreeMap, path::Path};
use textparse::{components::Maybe, Parse, ParseError, ParseResult, Parser, Position, Span};

#[derive(Debug, Clone)]
pub struct Music {
    title: Maybe<LineString>,
    //     pub composer: Option<String>,
    //     pub programer: Option<String>,
    // pub macros: Macros,
    // pub channels: BTreeMap<ChannelName, Channel>,
}

impl Music {
    pub fn title(&self) -> Option<&str> {
        self.title.get().map(|x| x.get())
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        todo!()
    }
}

impl std::str::FromStr for Music {
    type Err = ParseMusicError;

    fn from_str(script: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(script);
        Self::parse(&mut parser).map_err(|_| ParseMusicError {
            parser: parser.into_owned(),
            filename: None,
        })
    }
}

#[derive(Debug)]
pub struct ParseMusicError {
    parser: Parser<'static>,
    filename: Option<String>,
}
