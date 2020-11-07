include!("./token.rs");

use std::process;
use std::collections::HashMap;

fn add_token(tokens: &mut Vec<Tok>, token: &mut String, token_type: TokType) {
    tokens.push(Tok{
        typ: token_type,
        val: token.clone()
    });

    token.clear();
}

fn add_symbol(tokens: &mut Vec<Tok>, token: &str, token_type: TokType) {
    tokens.push(Tok{
        typ: token_type,
        val: token.to_string()
    });
}

fn is_integer(token: &String) -> bool {
    let num = token.parse::<i64>();
    return match num {
        Ok(_) => true,
        Err(_) => false
    }
}

fn is_decimal(token: &String) -> bool {
    let num = token.parse::<f64>();
    return match num {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_variable(token: &String) -> bool {
	if token.chars().nth(0).unwrap() != '_' && !token.chars().nth(0).unwrap().is_alphabetic() {
		return false;
    }

	for ch in token.chars() {
		if ch != '_' && !ch.is_alphabetic() && !ch.is_digit(10) {
			return false;
        }
	}

	return true;
}

fn is_char(token: &String) -> bool {
    if token.len() == 3 && token.chars().nth(0).unwrap() == '\'' && token.chars().nth(2).unwrap() == '\'' {
        return true;
    }

    return false;
}

fn check_keywords(tokens: &mut Vec<Tok>, token: &mut String) {
    if token.is_empty() {
        return;
    }

    if token == "set" {
        add_token(tokens, token, TokType::Set)
    }
    else if token == "if" {
        add_token(tokens, token, TokType::If);
    }
    else if token == "else" {
        add_token(tokens, token, TokType::Else);
    }
    else if token == "for" {
        add_token(tokens, token, TokType::For);
    }
    else if token == "in" {
        add_token(tokens, token, TokType::In);
    }
    else if token == "def" {
        add_token(tokens, token, TokType::Def);
    }
    else if is_char(&token) {
        add_token(tokens, token, TokType::CharVal);
    }
    else if is_integer(&token) {
        add_token(tokens, token, TokType::IntVal);
    }
    else if is_decimal(&token) {
        add_token(tokens, token, TokType::FloatVal);
    }
    else if is_variable(&token) {
        add_token(tokens, token, TokType::Var);
    }
    else {
        error(format!("undefined token '{}'", token).as_str());
    }
}

fn lex(line: &String) -> Vec<Tok> {
    let mut token = String::new();
    let mut tokens = Vec::new();

    let mut symbols = HashMap::new();
    symbols.insert('+', TokType::Op);
    symbols.insert('-', TokType::Op);
    symbols.insert('*', TokType::Op);
    symbols.insert('/', TokType::Op);
    symbols.insert('%', TokType::Op);
    symbols.insert('>', TokType::Op);
    symbols.insert('<', TokType::Op);
    symbols.insert('!', TokType::Op);
    symbols.insert('(', TokType::OpenBracket);
    symbols.insert(')', TokType::CloseBracket);
    symbols.insert('[', TokType::OpenSquare);
    symbols.insert(']', TokType::CloseSquare);
    symbols.insert('{', TokType::OpenCurly);
    symbols.insert('}', TokType::CloseCurly);
    symbols.insert('=', TokType::Assign);
    symbols.insert(',', TokType::Comma);

    let mut skip = false;
    let mut is_str = false;
    'outer: for a in 0..line.len() {
        if skip {
            skip = false;
            continue;
        }

        let ch = line.chars().nth(a).unwrap();

        if ch == '"' && !is_str {
            token.push(ch);
            is_str = true;
            continue;
        }
        else if ch != '"' && is_str {
            token.push(ch);
            continue;
        }
        else if ch == '"' && is_str {
            token.push(ch);
            add_token(&mut tokens, &mut token, TokType::StrVal);

            is_str = false;
            continue;
        }

        if a < line.len() - 1 && ch == '=' && line.chars().nth(a + 1).unwrap() == '=' {
            check_keywords(&mut tokens, &mut token);
            add_symbol(&mut tokens, "==", TokType::Op);

            skip = true;
            continue;
        }
        if a < line.len() - 1 && ch == '!' && line.chars().nth(a + 1).unwrap() == '=' {
            check_keywords(&mut tokens, &mut token);
            add_symbol(&mut tokens, "!=", TokType::Op);

            skip = true;
            continue;
        }
        if a < line.len() - 1 && ch == '<' && line.chars().nth(a + 1).unwrap() == '=' {
            check_keywords(&mut tokens, &mut token);
            add_symbol(&mut tokens, "<=", TokType::Op);

            skip = true;
            continue;
        }
        if a < line.len() - 1 && ch == '>' && line.chars().nth(a + 1).unwrap() == '=' {
            check_keywords(&mut tokens, &mut token);
            add_symbol(&mut tokens, ">=", TokType::Op);

            skip = true;
            continue;
        }

        for (key, value) in &symbols {
            if ch == *key {
                check_keywords(&mut tokens, &mut token);
                add_symbol(&mut tokens, &key.to_string(), value.clone());
                continue 'outer;
            }
        }

        if ch == ' ' && (token.len() != 1 || token != "'") {
            check_keywords(&mut tokens, &mut token);
        }
        else {
            token.push(ch);
        }
    }

    check_keywords(&mut tokens, &mut token);
    
    return tokens;
}