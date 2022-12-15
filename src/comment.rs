use textparse::{
    components::{AnyChar, Char, Either, Not, Until, While, Whitespace},
    Parse, Span,
};

#[derive(Debug, Clone, Span, Parse)]
pub enum Comment {
    Block(BlockComment),
    Line(LineComment),
}

pub type MaybeComment = Either<Comment, Not<(Char<'/'>, Char<'*'>)>>;

pub type CommentsOrWhitespaces = While<Either<Whitespace, Comment>>;

#[derive(Debug, Clone, Span, Parse)]
pub struct BlockComment(
    (Char<'/', false>, Char<'*'>),
    (AnyChar, Char<'*'>, Char<'/'>),
);

#[derive(Debug, Clone, Span, Parse)]
pub struct LineComment(
    Either<Char<';', false>, Char<'/', false>>,
    Until<Char<'\n'>>,
);
