use std::sync::Arc;

use crate::{
    channel::Channels,
    comment::CommentsOrWhitespaces,
    definitions::{Composer, Definition, Programer, Title},
    macros::Macros,
    oscillators::Oscillator,
    player::MusicPlayer,
};
use textparse::{ParseResult, Parser};

#[derive(Debug, Clone)]
pub struct Music {
    title: Option<Title>,
    composer: Option<Composer>,
    programer: Option<Programer>,
    macros: Arc<Macros>,
    channels: Channels,
}

impl Music {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let mut channels = Channels::new();

        let mut title = None;
        let mut composer = None;
        let mut programer = None;
        loop {
            let _: CommentsOrWhitespaces = parser.parse()?;
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
                Definition::Channel(x) => {
                    for name in x.channel_names().names() {
                        channels.add_channel(*name, Oscillator::from_kind(x.oscillator_kind()));
                    }
                }
            }
        }

        // TODOO: macro / definition order fix
        let _: CommentsOrWhitespaces = parser.parse()?;
        let mut macros = Macros::default();
        macros.parse(parser)?;

        let _: CommentsOrWhitespaces = parser.parse()?;
        channels.parse(parser)?;

        Ok(Self {
            title,
            composer,
            programer,
            macros: Arc::new(macros),
            channels,
        })
    }

    // TODO: validate

    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|x| x.get())
    }

    pub fn composer(&self) -> Option<&str> {
        self.composer.as_ref().map(|x| x.get())
    }

    pub fn programer(&self) -> Option<&str> {
        self.programer.as_ref().map(|x| x.get())
    }

    pub fn macros(&self) -> Arc<Macros> {
        self.macros.clone()
    }

    pub fn channels(&self) -> &Channels {
        &self.channels
    }

    pub fn into_channels(self) -> Channels {
        self.channels
    }

    pub fn play(self, sample_rate: u16) -> MusicPlayer {
        MusicPlayer::new(self, sample_rate)
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

impl ParseMusicError {
    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_owned());
        self
    }
}

impl std::fmt::Display for ParseMusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .parser
            .error_message_builder()
            .filename(self.filename.as_ref().map_or("<SCRIPT>", |s| s.as_str()))
            .build();
        write!(f, "{message}")
    }
}
