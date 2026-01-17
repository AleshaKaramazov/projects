
pub fn parse_if_expr(text: &str) -> String {
    let mut answer = String::new();
    if let Some((_, predicate)) = text.split_once(' ') {
        if predicate.starts_with("если") { 
            answer.push_str("\telse if ");
            if let Some((_, predicate)) = predicate.split_once(' ') {
                answer.push_str("\t\t");
                answer.push_str(predicate);
            }
        } else {
            answer.push_str("\tif ");
            answer.push_str(predicate);
        }
    } else {
        answer.push_str("\telse");
    }
    answer
}

