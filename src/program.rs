use roman_converter_algorithm::{ConverterUseCase, RomanConverter};
use crate::screen::console::{Display, GetUserInput};

pub type Program = Box<dyn Fn(ConverterUseCase, RomanConverter)>;
pub fn console_use_case(user_input_request: GetUserInput, display: Display) -> Program {
    Box::new(move |apply_conversion: ConverterUseCase, converter: RomanConverter| {
        let user_input = user_input_request();
        let result = apply_conversion(converter, user_input);
        match result {
            Ok(integer_value) => display(integer_value),
            Err(_) => display(String::from("invalid roman expression"))
        }
    })
}