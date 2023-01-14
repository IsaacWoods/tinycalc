use chumsky::prelude::*;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        // Print a prompt
        print!("> ");
        io::stdout().flush()?;

        // Get a line of input
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
}

impl Expr {
    pub fn eval(self) -> Result<f64, String> {
        match self {
            Expr::Num(value) => Ok(value),
            Expr::Neg(a) => Ok(-a.eval()?),
            Expr::Add(a, b) => Ok(a.eval()? + b.eval()?),
        }
    }
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| Expr::Num(s.parse().unwrap()))
        .padded();

    int.then_ignore(end())
}
