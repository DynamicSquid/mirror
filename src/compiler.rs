include!("error.rs");

fn split_code(tokens: &Vec<Tok>) -> Vec<Vec<Tok>> {
    let mut code = vec![Vec::new()];

    let mut curly_count = 0;
    for tok in tokens {
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

    return code;
}

fn type_to_str(typ: TokType) -> String {
    return match typ {
        TokType::IntVal   => "int".to_string(),
        TokType::IntArr   => "vector<int>".to_string(),
        TokType::FloatVal => "float".to_string(),
        TokType::FloatArr => "vector<float>".to_string(),
        TokType::CharVal  => "char".to_string(),
        TokType::CharArr  => "vector<char>".to_string(),
        TokType::StrVal   => "string".to_string(),
        TokType::StrArr   => "vector<string>".to_string(),
        _ => "auto".to_string()
    }
}

fn check_expr(expr: &Vec<Tok>) -> TokType {
    let expr_type = expr[0].typ.clone();

    for a in 0..expr.len() {
        if expr[a].typ == TokType::Op {
            if a == 0 {
                error("expected value to the left of operator");
            }
            if a == expr.len() - 1 {
                error("expected value to the right of operator");
            }

            if (expr[a].val == "+" || expr[a].val == "-" || expr[a].val == "*" ||
                expr[a].val == "/" || expr[a].val == "%")
                &&
                (expr[a - 1].typ != TokType::IntVal &&
                expr[a - 1].typ != TokType::FloatVal &&
                expr[a + 1].typ != TokType::IntVal &&
                expr[a + 1].typ != TokType::FloatVal) {
                error("operator can only be performed on numbers");
            }
        }

        if expr[a].typ == TokType::OpenSquare {
            if a != 0 && expr[a - 1].typ != TokType::Var {
                error("arrays cannot be used in expressions");
            }
            if a > 0 && expr[a - 1].typ == TokType::Var {
                return expr_type;
            }

            let rtn_tok: TokType = match expr[a + 1].typ {
                TokType::IntVal   => TokType::IntArr,
                TokType::FloatVal => TokType::FloatArr,
                TokType::CharVal  => TokType::CharArr,
                TokType::StrVal   => TokType::StrArr,
                _ => TokType::Set
            };

            if rtn_tok == TokType::Set {
                error("invalid array element");
            }

            return rtn_tok;
        }

        if (expr[a].typ == TokType::FloatVal && expr_type == TokType::IntVal) ||
           (expr[a].typ == TokType::IntVal && expr_type == TokType::FloatVal) {
            continue;
        }

        if expr[a].typ != expr_type && expr[a].typ != TokType::Op &&
            expr[a].typ != TokType::OpenBracket && expr[a].typ != TokType::CloseBracket {
            error("invalid expression");
        }
    }

    return expr_type;
}

fn write_file_content(file: &mut String, code: &Vec<Vec<Tok>>) -> std::io::Result<()> {
    for a in 0..code.len() {
        if code[a].is_empty() {
            continue;
        }

        if code[a][0].typ == TokType::Set {
            if code[a].len() == 1 || code[a][1].typ != TokType::Var {
                error("expected variable name after 'set' keyword");
            }
            if code[a].len() == 2 || code[a][2].typ != TokType::Assign {
                error("expected assignment operator after variable name");
            }
            if code[a].len() == 3  {
                error("expected expression after assignment operator");
            }

            let var_type = check_expr(&code[a][3..code[a].len()].to_vec());
            file.push_str(format!(
                "{} {} = ",
                type_to_str(var_type),
                code[a][1].val
            ).as_str());

            for b in 3..code[a].len() {
                if code[a][b].typ == TokType::OpenSquare && code[a][b - 1].typ != TokType::Var {
                    file.push_str("{");
                }
                else if code[a][b].typ == TokType::CloseSquare && code[a][b - 3].typ != TokType::Var {
                    file.push_str("}");
                }
                else {
                    file.push_str(code[a][b].val.as_str());
                }
            }

            file.push_str(";\n\n");
        }
        else if code[a].len() >= 2 && code[a][0].typ == TokType::Var &&
            code[a][1].typ == TokType::Assign {
            if code[a].len() == 2 {
                error("expected expression after assingment operator");
            }

            for tok in &code[a] {
                file.push_str(tok.val.as_str());
            }

            file.push_str(";\n\n");
        }
        else if code[a][0].typ == TokType::If {
            if code[a].len() == 1 || (code[a].len() == 2 && code[a][1].typ == TokType::OpenCurly) {
                error("expected condition in if statement");
            }
            if code[a].len() == 2 && code[a][1].typ != TokType::OpenCurly {
                error("expected open curly after if statement condition");
            }

            file.push_str("if (");

            let mut open_curly_index = 1;
            while code[a][open_curly_index].typ != TokType::OpenCurly &&
                open_curly_index < code[a].len() {
                let con_val = code[a][open_curly_index].val.clone() + " ";
                file.push_str(con_val.as_str());

                open_curly_index += 1;
            }

            if open_curly_index == code[a].len() {
                error("missing opening curly in if statement");
            }

            file.push_str(") {\n");

            let mut scope_content = "".to_string();
            write_file_content(
                &mut scope_content,
                &split_code(&code[a][open_curly_index + 1..code[a].len() - 1].to_vec())
            )?;

            file.push_str((scope_content + "}\n\n").as_str());
        }
        else if code[a].len() >= 2 && code[a][0].typ == TokType::Else &&
            code[a][1].typ == TokType::If {
            if a == 0 || (code[a - 1][0].typ != TokType::If && code[a - 1][1].typ != TokType::If) {
                error("else if statement must come after an if or an else if statement");
            }
            if code[a].len() == 2 || code[a].len() == 3 && code[a][2].typ == TokType::OpenCurly {
                error("expected condition in else if statement");
            }
            if code[a].len() == 3 && code[a][2].typ != TokType::OpenCurly {
                error("expected open curly after else if condition");
            }

            file.push_str("else if (");

            let mut open_curly_index = 2;
            while code[a][open_curly_index].typ != TokType::OpenCurly && open_curly_index < code[a].len() {
                let con_val = code[a][open_curly_index].val.clone() + " ";
                file.push_str(con_val.as_str());

                open_curly_index += 1;
            }

            if open_curly_index == code[a].len() {
                error("missing opening curly in else if statement");
            }

            file.push_str(") {\n");

            let mut scope_content = "".to_string();
            write_file_content(
                &mut scope_content,
                &split_code(&code[a][open_curly_index + 2..code[a].len() - 1].to_vec())
            )?;

            file.push_str((scope_content + "}\n\n").as_str());
        }
        else if code[a][0].typ == TokType::Else {
            if code[a].len() == 1 || code[a][1].typ != TokType::OpenCurly {
                error("expected open curly bracket after 'else' keyword");
            }

            file.push_str("else {\n");

            let mut scope_content = "".to_string();
            write_file_content(
                &mut scope_content,
                &split_code(&code[a][2..code[a].len() - 1].to_vec())
            )?;

            file.push_str((scope_content + "}\n\n").as_str());
        }
        else if code[a][0].typ == TokType::For {
            if code[a].len() == 1 || code[a][1].typ != TokType::Var {
                error("expected iterator name after 'for' keyword");
            }
            if code[a].len() == 2 || code[a][2].typ != TokType::In {
                error("expected 'in' keyword after iterator");
            }
            if code[a].len() == 3 || code[a][3].typ != TokType::Var {
                error("expected variable name after 'in' keyword");
            }
            if code[a].len() == 4 || code[a][4].typ != TokType::OpenCurly {
                error("expected open curly after array name");
            }

            file.push_str(format!(
                "for (auto& {} : {}) {{\n",
                code[a][1].val,
                code[a][3].val
            ).as_str());

            let mut scope_content = "".to_string();
            write_file_content(
                &mut scope_content,
                &split_code(&code[a][5..code[a].len() - 1].to_vec())
            )?;

            file.push_str((scope_content + "}\n\n").as_str());
        }
        else if code[a][0].typ == TokType::Var && code[a][1].typ == TokType::OpenBracket {
            if code[a].len() == 2 {
                error("missing closing bracket in function call");
            }

            if code[a][0].val == "print" {
                file.push_str("cout << ");
                for b in 2..code[a].len() - 1 {
                    file.push_str(code[a][b].val.as_str());
                }
            }
            else {
                for tok in &code[a] {
                    file.push_str(tok.val.as_str());
                }
            }

            file.push_str(";\n\n");
        }
        else if code[a][0].typ == TokType::Var && code[a][1].typ == TokType::OpenSquare {
            for tok in &code[a] {
                file.push_str(tok.val.as_str());
            }

            file.push_str(";\n\n");
        }
        else if code[a][0].typ != TokType::Def {
            error("invalid syntax");
        }
    }

    Ok(())
}

fn compiler(code: &mut Vec<Vec<Tok>>) -> std::io::Result<()> {
    let mut out = File::create("out.cpp")?;

    out.write_all(b"#include <iostream>\n#include <string>\n#include <vector>\n")?;
    out.write_all(b"#include \"utils.h\"\nusing namespace std;\n\n")?;

    for toks in code.clone() {
        if toks.len() >= 1 && toks[0].typ == TokType::Def {
            if toks.len() == 1 || toks[1].typ != TokType::Var {
                error("expected function name after 'def' keyword");
            }
            if toks.len() == 2 || toks[2].typ != TokType::OpenBracket {
                error("expected open bracket after function name");
            }

            let mut open_bracket = 0;
            for b in 3..toks.len() {
                if toks[b].typ == TokType::OpenBracket {
                    open_bracket += 1;
                }
                else if toks[b].typ == TokType::CloseBracket {
                    open_bracket -= 1;
                }

                if toks[b].typ == TokType::CloseBracket && open_bracket == -1 {
                    if b == toks.len() - 1 || toks[b + 1].typ != TokType::OpenCurly {
                        error("missing open curly for function body");
                    }
                    
                    break;
                }

                if b == toks.len() - 1 && toks[b].typ != TokType::OpenCurly {
                    error("missing closing curly bracket in function parameters");
                }
            }

            out.write_all(format!("void {}", toks[1].val).as_bytes())?;

            let mut curly_index = 2;
            while toks[curly_index].typ != TokType::OpenCurly && curly_index < toks.len() {
                out.write_all(format!("{} ", toks[curly_index].val).as_bytes())?;
                curly_index += 1;
            }

            if curly_index == toks.len() {
                error("missing opening curly bracket");
            }

            out.write_all(b"{\n")?;

            let mut scope_content = "".to_string();
            write_file_content(
                &mut scope_content,
                &split_code(&toks[curly_index + 1..toks.len() - 1].to_vec())
            )?;

            out.write_all((scope_content + "}\n\n").as_bytes())?;
        }
    }

    out.write_all(b"int main() {\n\n")?;

    let mut content: String = "".to_string();
    write_file_content(&mut content, &code)?;

    out.write_all((content + "}").as_bytes())?;

    Ok(())
}