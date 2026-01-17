

pub fn parse_if_expr(text: &str) -> String {
    let mut answer = String::from("\tif ");
    if let Some((_, if_expr)) = text.split_once(' ') {
        answer.push_str(if_expr);
    } 
    answer
}

