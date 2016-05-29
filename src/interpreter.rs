use ast::*;

pub struct Interpreter {
    program: Program
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        Interpreter { program: program }
    }

    pub fn execute(&self) {
        let expr = match self.program.items[0] {
            Item::Impl(ref impl_block) => {
                match impl_block.methods[0].body {
                    Expr::Block(ref exprs) => {
                        match exprs[0] {
                            Expr::MethodCall(ref lhs, ref name, ref args) => {
                                &args[0]
                            },
                            _ => panic!()
                        }
                    },
                    _ => panic!()
                }
            },
            _ => panic!()
        };

        let int = self.eval_expr(expr);

        println!("{:?}", int);
    }

    fn eval_expr(&self, expr: &Expr) -> i64 {
        match *expr {
            Expr::IntLiteral(ref int_str) => {
                int_str.parse().unwrap()
            },
            Expr::BinaryOp(ref lhs, ref op, ref rhs) => {
                match *op {
                    BinaryOp::Add => { self.eval_expr(lhs) + self.eval_expr(rhs) },
                    BinaryOp::Sub => { self.eval_expr(lhs) - self.eval_expr(rhs) },
                    BinaryOp::Mul => { self.eval_expr(lhs) * self.eval_expr(rhs) },
                    BinaryOp::Div => { self.eval_expr(lhs) / self.eval_expr(rhs) },
                }
            },
            _ => panic!()
        }
    }
}
