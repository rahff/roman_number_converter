pub mod model;

use crate::model::{RomanCharacter, RomanExpression};

pub type ConverterUseCase = fn(converter: RomanConverter, input: String) -> Result<String, ()>;
pub fn apply_conversion(converter: RomanConverter, input: String) -> Result<String, ()> {
    let roman_expression = RomanExpression::from_string(input);
    let roman_number = match roman_expression {
        Some(roman) => roman,
        None => return Err(())

    };
    Ok(converter(roman_number).to_string())
}

pub type RomanConverter = fn(roman: RomanExpression) -> u32;
pub fn roman_to_integer(roman: RomanExpression) -> u32 {
    let mut integer_values = parse_roman_to_integers(roman);
    compute_pre_values(&mut integer_values);
    integer_values.iter().cloned().sum()
}

fn compute_pre_values(integer_values: &mut Vec<u32>) -> () {
    let size = integer_values.len();
    let subtracted_values: u32 = subtract_values(integer_values, size);
    for i in 0..subtracted_values  {
        subtract_values(integer_values, size);
    }
}

fn subtract_values(integer_values: &mut Vec<u32>, size: usize) -> u32 {
    let mut subtracted_values: u32 = 0;
    for i in 0..(size - 1) {
        if integer_values[i] < integer_values[i+1] {
            integer_values[i] = integer_values[i+1] - integer_values[i];
            integer_values[i+1] = 0;
            subtracted_values += 1
        }
    };
    subtracted_values
}
fn parse_roman_to_integers(roman: RomanExpression) -> Vec<u32> {
    let limit: usize = roman.value.len();
    let mut integer_values: Vec<u32> = Vec::with_capacity(limit);
    for i in 0..limit {
        let roman_character = get_nth_symbol(&roman.value, i);
        integer_values.push(convert_one_symbol(roman_character));
    };
    integer_values
}

fn get_nth_symbol(roman: &String, index: usize) -> RomanCharacter {
    RomanCharacter::new(roman.chars().nth(index).unwrap().to_string())
}

fn convert_one_symbol(roman_symbol: RomanCharacter) -> u32 {
    match roman_symbol.figure.as_str() {
        "I" => 1,
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use super::*;

    #[test]
    fn it_convert_primitive_symbol_to_corresponding_integer() {
        assert_eq!(1, roman_to_integer(RomanExpression::from_string(String::from("I")).unwrap()));
        assert_eq!(5, roman_to_integer(RomanExpression::from_string(String::from("V")).unwrap()));
        assert_eq!(10, roman_to_integer(RomanExpression::from_string(String::from("X")).unwrap()));
        assert_eq!(50, roman_to_integer(RomanExpression::from_string(String::from("L")).unwrap()));
        assert_eq!(100, roman_to_integer(RomanExpression::from_string(String::from("C")).unwrap()));
        assert_eq!(500, roman_to_integer(RomanExpression::from_string(String::from("D")).unwrap()));
        assert_eq!(1000, roman_to_integer(RomanExpression::from_string(String::from("M")).unwrap()));
    }
    #[test]
    fn convert_with_unit_primitive_addition_symbol() {
        assert_eq!(2, roman_to_integer(RomanExpression::from_string(String::from("II")).unwrap()));
        assert_eq!(3, roman_to_integer(RomanExpression::from_string(String::from("III")).unwrap()));
        assert_eq!(8, roman_to_integer(RomanExpression::from_string(String::from("VIII")).unwrap()));
    }

    #[test]
    fn convert_with_subtracted_symbol() {
        assert_eq!(4, roman_to_integer(RomanExpression::from_string(String::from("IV")).unwrap()));
        assert_eq!(9, roman_to_integer(RomanExpression::from_string(String::from("IX")).unwrap()));
        assert_eq!(90, roman_to_integer(RomanExpression::from_string(String::from("XC")).unwrap()));
        assert_eq!(40, roman_to_integer(RomanExpression::from_string(String::from("XL")).unwrap()));
    }

        #[test]
    fn convert_many_symbol() {
        assert_eq!(14, roman_to_integer(RomanExpression::from_string(String::from("XIV")).unwrap()));
        assert_eq!(13, roman_to_integer(RomanExpression::from_string(String::from("XIII")).unwrap()));
        assert_eq!(13, roman_to_integer(RomanExpression::from_string(String::from("XIII")).unwrap()));
        assert_eq!(80, roman_to_integer(RomanExpression::from_string(String::from("LXXX")).unwrap()));
    }
    #[test]
    fn convert_many_symbol_with_mix_regular_and_subtracted(){
        let expected1 = String::from("1979");
        let result1 = apply_conversion(roman_to_integer, String::from("MCMLXXIX"));
        assert_matches!(result1, Ok(expected));
        let expected2 = String::from("1949");
        let result2 = apply_conversion(roman_to_integer, String::from("MCMXLIX"));
        assert_matches!(result2, Ok(expected2));
        let expected3 = String::from("1789");
        let result3 = apply_conversion(roman_to_integer, String::from("MDCCLXXXIX"));
        assert_matches!(result3, Ok(expected3));
    }

    #[test]
    fn numerically_invalid_roman_expression_should_not_be_accepted(){
        let result1 = apply_conversion(roman_to_integer, String::from("VX"));
        assert_matches!(result1, Err(()));
        let result2 = apply_conversion(roman_to_integer, String::from("ICM"));
        assert_matches!(result2, Err(()));
        let result3 = apply_conversion(roman_to_integer, String::from("IL"));
        assert_matches!(result3, Err(()));
        let result4 = apply_conversion(roman_to_integer, String::from("IVDLXXI"));
        assert_matches!(result4, Err(()));
        let result5 = apply_conversion(roman_to_integer, String::from("VIIVII"));
        assert_matches!(result5, Err(()));
        let result6 = apply_conversion(roman_to_integer, String::from("MDCCLXXXIXVII"));
        assert_matches!(result6, Err(()));
    }
}
