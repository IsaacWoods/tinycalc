use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        // Print a prompt
        print!("> ");
        io::stdout().flush()?;

        // Get a line of input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        println!("Got line: {}", input);
    }
}
