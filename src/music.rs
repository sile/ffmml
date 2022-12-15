use crate::{
    channel::{Channel, ChannelName},
    comment::{Comment, MaybeComment},
    definitions::{Composer, Definition, Programer, Title},
    macros::Macros,
};
use std::{collections::BTreeMap, path::Path};
use textparse::{
    components::{Either, Maybe, While, Whitespaces},
    Parse, ParseError, ParseResult, Parser, Position, Span,
};

#[derive(Debug, Clone)]
pub struct Music {
    title: Option<Title>,
    composer: Option<Composer>,
    programer: Option<Programer>, // pub macros: Macros,
                                  // pub channels: BTreeMap<ChannelName, Channel>,
}

impl Music {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let mut title = None;
        let mut composer = None;
        let mut programer = None;

        loop {
            let _: While<Either<Whitespaces, Comment>> = parser.parse()?;
            if parser.peek_char() != Some('#') {
                break;
            }
            match parser.parse()? {
                Definition::Title(x) => {
                    title = Some(x);
                }
                Definition::Composer(x) => {
                    composer = Some(x);
                }
                Definition::Programer(x) => {
                    programer = Some(x);
                }
            }
        }

        let _: While<Either<Whitespaces, Comment>> = parser.parse()?;
        if !parser.is_eos() {
            return Err(ParseError);
        }

        Ok(Self {
            title,
            composer,
            programer,
        })
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|x| x.get())
    }

    pub fn composer(&self) -> Option<&str> {
        self.composer.as_ref().map(|x| x.get())
    }

    pub fn programer(&self) -> Option<&str> {
        self.programer.as_ref().map(|x| x.get())
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
