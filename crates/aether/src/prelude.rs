use crate::runtime_types::RuntimeValue;
pub type Value = RuntimeValue;
pub fn print_value(value:&Value)->String{value.to_string()}
pub fn truthy(value:&Value)->bool{value.is_truthy()}
