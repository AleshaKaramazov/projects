use std::{
    fmt, io::Write
};

use crate::type_expr::type_name;


pub fn parse_let_func(text: &str) -> String {
    /*
    it can be like (i want to realize 3 words only) 
    1) "let name:= read_"
    2) "let name := read_"
    3) "let name :=read_"
    4) "let name: String = read_"
    5) "let name:String, = read_"
    6) "let name :String = read_"
    7) "let name : String = read_"
    8) "let nmame :String=read_"
    */    
    let mut answer = String::new();
    if let Some((name, expr)) = text.split_once(':') {
        answer.push_str("    let mut ");
        let mut per_name = "";
        if let Some(name) = name.split_whitespace().nth(1) {
            per_name = name;
            answer.push_str(name);
            answer.push(' ');
        }
        if let Some((type_n, expr)) = expr.split_once('=') {
            let typed = type_name(type_n.trim());
            answer.push(':');
            answer.push_str(typed);
            if let Some((command, expr)) 
                    = expr.trim().split_once('!') 
                    && command.starts_with("считать") {
                answer.push_str(";");
                answer.push_str(&format!("\n\
                    \t{{\n\
                        \t\tprint!{};\n\
                        \t\tio::stdout().flush()?;\n\
                        \t\tlet mut for_read = String::new();\n\
                        \t\tio::stdin().read_line(&mut for_read)?;\n\
                        \t\t{} = match for_read.trim().parse::<{}>() {{\n\
                        \t\t\tOk(res) => res,\n\
                        \t\t\tErr(e) => {{\n\
                        \t\t\t\teprintln!(\"ошибка перевода в {}\");\n\
                        \t\t\t\treturn Ok(());\n\
                        \t\t\t}},\n\
                        \t\t}}\n\
                    \t}}", expr, per_name, typed, typed));
            } else {
                answer.push_str("= ");
                answer.push_str(expr);
            }
        }
    }
    answer
}

