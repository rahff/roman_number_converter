pub mod console {
    use std::{io, process};
    use roman_converter_algorithm::{ConverterUseCase, RomanConverter};
    pub(crate) type Display = fn(text: String) -> ();
    pub fn display(text: String) -> () {
        println!("{}", text)
    }

    pub(crate) type GetUserInput = fn() -> String;
    pub fn get_user_input() -> String {
        let mut user_input= String::new();
        display(String::from("enter a roman expression or press q for exit"));
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        match user_input.contains("q") {
            true => process::exit(0),
            false => return user_input
        }
    }
}