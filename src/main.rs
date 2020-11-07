include!("./lexer.rs");
include!("./compiler.rs");

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file = File::open("source.mir")?;
    let reader = BufReader::new(file);

    let mut toks = Vec::new();

    for line in reader.lines() {
        let mut get_tokens = lex(&line.unwrap());
        if !get_tokens.is_empty() {
            toks.append(&mut get_tokens);
            toks.push(Tok{
                typ: TokType::Eol,
                val: String::from("eol")
            });
        }
    }

    if toks.len() == 0 {
        let mut out = File::create("out.cpp")?;
        out.write_all(b"int main() {}")?;

        return Ok(());
    }

    let mut code = vec![Vec::new()];
    
    let mut curly_count = 0;
    for tok in toks {
        if tok.typ == TokType::Eol && curly_count == 0 {
            code.push(Vec::new());
            continue;
        }

        if tok.typ == TokType::OpenCurly {
            curly_count += 1;
        }
        else if tok.typ == TokType::CloseCurly {
            curly_count -= 1;
        }

        let len = code.len() - 1;
        code[len].push(tok.clone());
    }

    compiler(&mut code)?;

    Ok(())
}