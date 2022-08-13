#[derive(Debug, Clone, Copy)]
pub enum TreeError {
    /// The entity does not exist in the tree.
    NoEntity,
    /// Parent does not exist in the tree.
    InvalidParent,
    /// Sibling does not exist in the tree.
    InvalidSibling,
    /// Entity is null.
    NullEntity,
    /// Desired sibling is already the sibling.
    AlreadySibling,
    /// Desired first child is already the first child.
    AlreadyFirstChild,
}
