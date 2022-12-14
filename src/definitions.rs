use textparse::{Position, Span};

#[derive(Debug, Clone, Span)]
pub struct LineString {
    start: Position,
    value: String,
    end: Position,
}

impl LineString {
    pub fn get(&self) -> &str {
        &self.value
    }
}
