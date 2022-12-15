use textparse::{
    components::{AnyChar, Char, Either, Not, Until},
    Parse, Span,
};

#[derive(Debug, Clone, Span, Parse)]
pub enum Comment {
    Block(BlockComment),
    Line(LineComment),
}

pub type MaybeComment = Either<Comment, Not<(Char<'/'>, Char<'*'>)>>;

#[derive(Debug, Clone, Span, Parse)]
pub struct BlockComment((Char<'/'>, Char<'*'>), (AnyChar, Char<'*'>, Char<'/'>));

#[derive(Debug, Clone, Span, Parse)]
pub struct LineComment(Either<Char<';'>, Char<'/'>>, Until<Char<'\n'>>);
