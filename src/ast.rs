#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>
}

#[derive(Debug)]
pub enum Item {
    Struct(Struct),
    Impl(Impl)
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub attrs: Vec<String>
}

#[derive(Debug)]
pub struct Impl {
    pub name: String,
    pub methods: Vec<Method>
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expr
}

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    IntLiteral(String),
    StringLiteral(String),
    VecLiteral(Vec<Expr>),
    MapLiteral(Vec<(String, Expr)>),

    MethodCall(Box<Expr>, String, Vec<Expr>),
    Assignment(Access, Box<Expr>),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
}

#[derive(Debug)]
pub enum Access {
    Identifier(String),
    Member(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}
