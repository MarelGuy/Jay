use super::identifier::IdentifierNode;
use either::Either;

#[derive(Debug, PartialEq)]
pub struct ImportNode<'a> {
    pub import: Either<IdentifierNode<'a>, Vec<IdentifierNode<'a>>>,
    pub from: IdentifierNode<'a>,
}

impl<'a> ImportNode<'a> {
    pub fn new(
        import: Either<IdentifierNode<'a>, Vec<IdentifierNode<'a>>>,
        from: IdentifierNode<'a>,
    ) -> Self {
        Self { import, from }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportNode<'a> {
    pub items: Either<IdentifierNode<'a>, Vec<IdentifierNode<'a>>>,
}

impl<'a> ExportNode<'a> {
    pub fn new(items: Either<IdentifierNode<'a>, Vec<IdentifierNode<'a>>>) -> Self {
        Self { items }
    }
}
