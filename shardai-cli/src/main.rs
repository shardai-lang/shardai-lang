use shardai_syntax::lexer::Lexer;
use shardai_syntax::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // the first argument is always the executable name
    match args.len() {
        1 => run_repl(), // no arguments were passed
        2 => run_file(&args[1]), // (what we can assume to be) a file was passed

        _ => { println!("usage: {} file_path", args[0]); Ok(()) }
    }
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

fn run_file(file_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(file_path)?;
    let mut lexer = Lexer::new(file_contents);
    let tokens = lexer.lex()?;

    for tok in &tokens {
        println!("{:?}", tok)
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    for node in &ast {
        println!("{:?}", node)
    }
    
    Ok(())
}
