mod program;
mod screen;

use roman_converter_algorithm::{apply_conversion, roman_to_integer};
use crate::program::console_use_case;
use crate::screen::console::{display, get_user_input};



fn main() {
   let program = console_use_case(get_user_input, display);
   loop {
      program(apply_conversion, roman_to_integer);
   }
}


