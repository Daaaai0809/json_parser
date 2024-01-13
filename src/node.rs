#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SyntaxTree {
    NullLiteral,
    TrueLiteral,
    FalseLiteral,
    NumberLiteral(f64),
    StringLiteral(String),
    Identifier(String),
    PropertyAssignment,
    ObjectLiteral,
    ArrayLiteral,
    End,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Node {
    pub kind: SyntaxTree,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(kind: SyntaxTree, children: Vec<Node>) -> Node {
        Node {
            kind,
            children: children,
        }
    }
}
