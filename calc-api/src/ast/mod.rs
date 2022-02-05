
#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq)]
pub enum Expr {
    Const(Const),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Const(e) => e.eval(),
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Sub(left, right) => left.eval() - right.eval(),
            Expr::Mul(left, right) => left.eval() * right.eval(),
            Expr::Div(left, right) => left.eval() / right.eval(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Const(i32);

impl Const {
    pub fn new(val: i32) -> Const {
        Const(val)
    }

    pub fn eval(&self) -> i32 {
        self.0
    }
}