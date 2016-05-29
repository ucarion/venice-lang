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

    MethodCall(Box<Expr>, String, Vec<Expr>),
    Assignment(Access, Box<Expr>),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    Block(Vec<Expr>),
}

#[derive(Debug)]
pub enum Access {
    Member(Box<Expr>, String),
    Identifier(String)
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}
