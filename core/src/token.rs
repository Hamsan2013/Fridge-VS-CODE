  /// FRIDGE token kinds — language spec v0.2.1.
/// Official shortcut aliases collapse onto their canonical kind
/// (else -> Otherwise, loop -> Repeat, print -> Show, start -> Starts).
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Structure
    Eof,
    Newline,
    Comma,
    LParen,
    RParen,

    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Ident(String),

    // Statements
    Remember, As, Set, To, Add, Subtract, Multiply, Divide, Append, From, By,
    Show, Call, With, Return, Break, Continue, Use,

    // Control flow
    If, Otherwise, End, Repeat, Times, Forever, While, For, Each, In,

    // Declarations
    Create, Function, Called, Taking, And, Returning, When,

    // Word operators
    Or, Not, Is, Greater, Less, Than, At, Least, Most, Equal, Joined, Modulo,
    Plus, Minus, Multiplied, Divided,

    // Literal keywords
    True, False, Nothing,

    // Types and collections
    Number, Text, Boolean, Anything, List, Empty, Containing, Of, Item, Items,
    First, Last,

    // Events
    Program, Starts, Ends, Pressed, Released, Touched, Collides, Frame,

    // Structures / apps / GUI (grammar defined; runtime phases 3–8)
    Structure, Map, Game, Screen, Place, Open, Showing,

    // Shortcuts needing their own kind
    On, Define, Becomes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}
