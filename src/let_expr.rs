use crate::type_expr::type_name;


pub fn parse_let_expr(text: &str) -> String {
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
    let mut answer = String::from("\tlet mut ");
    if let Some((name, expr)) = text.split_once(':') {
        let mut per_name = "";
        if let Some(name) = name.split_whitespace().nth(1) {
            per_name = name;
            answer.push_str(name);
            answer.push(' ');
        }
        if let Some((type_n, expr)) = expr.split_once('=') {
            let typed = type_name(type_n.trim());
            if !typed.is_empty() {
                answer.push(':');
                answer.push_str(typed);
            }
            if let Some((command, expr)) 
                    = expr.trim().split_once('!') 
                    && command.starts_with("считать") {
                answer.push_str(";");
                answer.push_str(&format!("\n\t{{\n\
                        print!{};\n\
                        io::stdout().flush()?;\n\
                        let mut for_read = String::new();\n\
                        io::stdin().read_line(&mut for_read)?;\n", expr));
                if typed != "String" && !typed.is_empty() {
                    answer.push_str(&format!("\n\
                        {} = match for_read.trim().parse::<{}>() {{\n\
                        Ok(res) => res,\n\
                        Err(e) => {{\n\
                        eprintln!(\"ошибка перевода в {}\");\n\
                        return Ok(());\n\
                        }},\n\
                        }}\n\
                    }}", per_name, typed, typed));
                } else {
                    answer.push_str(
                        &format!("\t\t{} = for_read;\n\t}}\n", per_name));
                }
            } else {
                answer.push_str("= ");
                answer.push_str(expr);
            }
        }
    }
    answer
}

