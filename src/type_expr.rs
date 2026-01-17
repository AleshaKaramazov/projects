

pub fn type_name(name: &str) -> &str {
    match name{
        "Логич" => "bool",
        "Цел" | "Нат" => "i32",
        "Строка" => "String",
        _=> "",
    }
}
