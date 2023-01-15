use chumsky::prelude::*;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        // Print a prompt
        print!("> ");
        io::stdout().flush()?;

        // Get a line of input
        // TODO: handle up and down arrows for input history
        // TODO: handle empty input for spacing
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let ast = parser().parse(input);
        println!("AST: {:?}", ast);

        if let Ok(expr) = ast {
            println!("Result: {:?}", expr.eval());
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Num(f64),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(self) -> Result<f64, String> {
        match self {
            Expr::Num(value) => Ok(value),
            Expr::Neg(a) => Ok(-a.eval()?),
            Expr::Add(a, b) => Ok(a.eval()? + b.eval()?),
            Expr::Sub(a, b) => Ok(a.eval()? - b.eval()?),
            Expr::Mul(a, b) => Ok(a.eval()? * b.eval()?),
            Expr::Div(a, b) => Ok(a.eval()? / b.eval()?),
        }
    }
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        // TODO: support scientific notation
        let num = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .padded();

        let atom = num.or(expr.delimited_by(just('('), just(')'))).padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    })
    .then_ignore(end())
}
