use std::{fs::{self, File}, io::{self, Write}};
mod let_expr;
mod type_expr;
use let_expr::parse_let_func;

pub enum ExprType {
    Let,
    Loop,
    Func
}

fn expr_type(command: &str) -> ExprType {
    if command.starts_with("пусть") {
        ExprType::Let
    } else if command.starts_with("для") { 
        ExprType::Loop
    } else{
        ExprType::Func
    }
}

fn parse_file(text: String, mut output: File) -> io::Result<()> {
    let mut end_depth = 0;
    for line in 
            text.lines()
            .filter(|line| !line.is_empty() && !line.starts_with(['/', '!']))
            .map(&str::trim) {
        if line.starts_with("Алгоритм") {
            write!(output, "fn ")?;
            if let Some(name) = line.split_whitespace().nth(1) {
                if name == "Главная" {
                    writeln!(output, "main() -> io::Result<()> ")?;
                } else {
                    writeln!(output, "{} {{", name)?;
                }
            }
        } else if line.starts_with("Начало") {
            end_depth+=1; 
            writeln!(output, "{{")?; 
        } else if line.starts_with("Конец") {
            if end_depth == 1 {
               writeln!(output, "\tOk(())\n}}")?; 
               return Ok(());
            } else {
                end_depth-=1;
                writeln!(output, "}}")?;
            }
        }
        else {
            match expr_type(line) {
                ExprType::Let =>{
                    writeln!(output, "{}", parse_let_func(line))?;
                }
                _=> println!("unknown: {}", line),
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let filename = "alg.абв";
    let new_filename = 
        format!("{}.rs", filename.strip_suffix(".абв").unwrap());

    let text = fs::read_to_string(filename)?;
    let mut file = File::create(new_filename)?;
    writeln!(file, "\
    #![allow(unused)]\n\
    use std::io::{{self, Write}};\n")?; 
    parse_file(text, file)
}
