

pub fn type_name(name: &str) -> &str {
    match name{
        "Нат" => "usize",
        "Логич" => "bool",
        "Цел" => "i32",
        "Строка" => "String",
        _=> "",
    }
}
