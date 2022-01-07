use frost::parser::parse;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    let mut stdout = io::stdout();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;

        let parse = parse(&input);

        println!("{}", parse.debug_tree());
        input.clear();
    }
}
