use std::fmt::Display;

pub fn build_string<T: Display>(arg: T) -> String {
    format!("{}", arg)
}

