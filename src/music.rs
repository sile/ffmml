use crate::{
    channel::Channels,
    comment::CommentsOrWhitespaces,
    definitions::{Composer, Definition, Programer, Title},
    macros::Macros,
    oscillators::Oscillator,
    player::MusicPlayer,
};
use std::{
    borrow::Cow,
    error::Error,
    path::{Path, PathBuf},
    sync::Arc,
};
use textparse::{ParseError, Parser, Position, Span};

/// TODO: A music instance built from a MML script.
#[derive(Debug, Clone)]
pub struct Music {
    title: Option<Title>,
    composer: Option<Composer>,
    programer: Option<Programer>,
    macros: Arc<Macros>,
    channels: Channels,
}

impl Music {
    pub fn new(text: &str) -> Result<Self, ParseMusicError> {
        text.parse()
    }

    fn parse(parser: &mut Parser) -> Option<Result<Self, ParseMusicError>> {
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

        let _: CommentsOrWhitespaces = parser.parse()?;
        let mut macros = Macros::default();
        macros.parse(parser)?;

        let _: CommentsOrWhitespaces = parser.parse()?;
        if let Err(e) = channels.parse(parser)? {
            return Some(Err(e));
        }

        Some(Ok(Self {
            title,
            composer,
            programer,
            macros: Arc::new(macros),
            channels,
        }))
    }

    /// Music title defined by `#TITLE <VALUE>` in the script.
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|x| x.get())
    }

    pub fn composer(&self) -> Option<&str> {
        self.composer.as_ref().map(|x| x.get())
    }

    pub fn programer(&self) -> Option<&str> {
        self.programer.as_ref().map(|x| x.get())
    }

    pub(crate) fn macros(&self) -> Arc<Macros> {
        self.macros.clone()
    }

    pub(crate) fn into_channels(self) -> Channels {
        self.channels
    }

    pub fn play(self, sample_rate: u16) -> MusicPlayer {
        MusicPlayer::new(self, sample_rate)
    }
}

impl std::str::FromStr for Music {
    type Err = ParseMusicError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(text);
        match Self::parse(&mut parser) {
            None => Err(ParseMusicError::from(parser.into_parse_error())),
            Some(Err(mut e)) => {
                e.text = text.to_owned();
                Err(e)
            }
            Some(Ok(v)) => Ok(v),
        }
    }
}

pub struct ParseMusicError {
    textparse_error: Option<Box<ParseError>>,
    text: String,
    position: Position,
    reason: String,
    file_path: Option<PathBuf>,
}

impl ParseMusicError {
    pub(crate) fn new(item: impl Span, reason: &str) -> Self {
        Self {
            textparse_error: None,
            text: String::new(),
            position: item.start_position(),
            reason: reason.to_owned(),
            file_path: None,
        }
    }

    pub fn file_path<P: AsRef<Path>>(mut self, file_path: P) -> Self {
        if let Some(e) = self.textparse_error.take() {
            self.textparse_error = Some(Box::new(e.file_path(file_path)));
        } else {
            self.file_path = Some(file_path.as_ref().to_path_buf());
        }
        self
    }
}

impl std::fmt::Debug for ParseMusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(e) = &self.textparse_error {
            return e.fmt(f);
        }

        let offset = self.position.get();
        let (line, column) = self.position.line_and_column(&self.text);
        writeln!(f, "{}", self.reason)?;
        writeln!(
            f,
            "  --> {}:{line}:{column}",
            self.file_path
                .as_ref()
                .map(|s| s.to_string_lossy())
                .unwrap_or(Cow::Borrowed("<UNKNOWN>"))
        )?;

        let line_len = format!("{line}").len();
        writeln!(f, "{:line_len$} |", ' ')?;
        writeln!(
            f,
            "{line} | {}",
            self.text[offset + 1 - column..]
                .lines()
                .next()
                .unwrap_or("")
        )?;
        writeln!(f, "{:line_len$} | {:>column$} {}", ' ', '^', self.reason)?;
        Ok(())
    }
}

impl std::fmt::Display for ParseMusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ParseMusicError {}

impl From<ParseError> for ParseMusicError {
    fn from(value: ParseError) -> Self {
        Self {
            textparse_error: Some(Box::new(value)),
            text: String::new(),
            position: Position::new(0),
            reason: String::new(),
            file_path: None,
        }
    }
}
