

pub fn parse_write_expr(text: &str) -> String {
    let mut answer = String::from("println!");
    if let Some((_, write)) = text.split_once('!') {
        answer.push_str("\t");
        answer.push_str(write);
    }
    answer
}
