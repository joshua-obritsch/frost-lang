use std::io::{self, Write};

fn main() -> io::Result<()> {
  let stdin = io::stdin();

  let mut env = frost::Env::default();
  let mut input = String::new();
  let mut stderr = io::stderr();
  let mut stdout = io::stdout();

  loop {
    write!(stdout, "> ")?;
    stdout.flush()?;
    stdin.read_line(&mut input)?;

    match run(input.trim(), &mut env) {
      Ok(Some(val)) => writeln!(stdout, "{}", val)?,
      Ok(None) => {}
      Err(msg) => writeln!(stderr, "{}", msg)?,
    }

    input.clear();
  }
}

fn run(input: &str, env: &mut frost::Env) -> Result<Option<frost::Val>, String> {
  let parse = frost::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;
  let evaluated = parse
    .eval(env)
    .map_err(|msg| format!("Evaluation error: {}", msg))?;

  if evaluated == frost::Val::Unit {
    Ok(None)
  } else {
    Ok(Some(evaluated))
  }
}
