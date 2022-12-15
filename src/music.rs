use crate::{
    channel::{Channel, ChannelName},
    definitions::{Composer, Programer, Title},
    macros::Macros,
};
use std::{collections::BTreeMap, path::Path};
use textparse::{components::Maybe, Parse, ParseError, ParseResult, Parser, Position, Span};

#[derive(Debug, Clone)]
pub struct Music {
    title: Maybe<Title>,
    composer: Maybe<Composer>,
    programer: Maybe<Programer>, // pub macros: Macros,
                                 // pub channels: BTreeMap<ChannelName, Channel>,
}

impl Music {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        todo!()
    }

    pub fn title(&self) -> Option<&str> {
        self.title.get().map(|x| x.get())
    }

    pub fn composer(&self) -> Option<&str> {
        self.composer.get().map(|x| x.get())
    }

    pub fn programer(&self) -> Option<&str> {
        self.programer.get().map(|x| x.get())
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

// TODO: implement Error
#[derive(Debug)]
pub struct ParseMusicError {
    parser: Parser<'static>,
    filename: Option<String>,
}
