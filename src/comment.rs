use textparse::{
    components::{AnyChar, Char, Either, Not, Str, While, Whitespace},
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
    #[allow(dead_code)] While<(Not<Str<'*', '/'>>, AnyChar)>,
    Str<'*', '/'>,
);

#[derive(Debug, Clone, Span, Parse)]
pub struct LineComment(
    Either<Char<';', false>, Char<'/', false>>,
    While<(Not<Char<'\n'>>, AnyChar)>,
);
