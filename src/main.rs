use std::{env, fs::{self, File}, io::{self, Write}};
mod let_expr;
mod type_expr;
mod if_expr;
mod write_expr;
use write_expr::parse_write_expr;
use if_expr::parse_if_expr;
use let_expr::parse_let_expr;

pub enum ExprType {
    Let,
    If,
    Loop,
    Func
}

fn parse_expr(line: &str, mut end_depth: i32, output: &mut File) -> io::Result<i32> {
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
           return Ok(end_depth);
        } else {
            end_depth-=1;
            writeln!(output, "}}")?;
        }
    }
    else if line.starts_with("пусть") {
        writeln!(output, "{}", parse_let_expr(line))?;
    } else if line.starts_with("если") || line.starts_with("иначе") {
        writeln!(output, "{}", parse_if_expr(line))?;
    } else if line.starts_with("написать") {
        writeln!(output, "{}", parse_write_expr(line))?;
    }
    Ok(end_depth) 
}

fn parse_file(text: String, mut output: File) -> io::Result<()> {
    let mut end_depth = 0;
    for line in 
            text.lines()
            .filter(|line| !line.is_empty() && !line.starts_with(['/', '!']))
            .map(&str::trim) {
        end_depth = parse_expr(line, end_depth, &mut output)?; 
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("использование: cargo run -- [ИМЯ ФАЙЛА].абв");
        return Ok(())
    }
    let filename = &args[1];
    let new_filename = 
        format!("{}.rs", filename.strip_suffix(".абв").unwrap());

    let text = fs::read_to_string(filename)?;
    let mut file = File::create(new_filename)?;
    writeln!(file, "\
    #![allow(unused)]\n\
    #![allow(warnings)]\n\
    use std::io::{{self, Write}};\n")?; 
    parse_file(text, file)
}
