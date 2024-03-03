

pub struct  RomanCharacter {
    pub figure: String
}

impl RomanCharacter {
    pub fn new(value: String) -> Self {
        RomanCharacter {figure: value}
    }
}

#[derive(Debug)]
pub struct RomanExpression {
    pub value: String
}

impl RomanExpression {

    pub fn from_string(value: String) -> Option<RomanExpression> {
        if Self::is_structurally_valid(&value) {
            Some(RomanExpression{value: value})
        }else { None }
    }

    fn is_structurally_valid(value: &String) -> bool {
         Self::is_only_composed_by_roman_figures(value)
                         &
         Self::is_numerically_valid(&value)
                         &
         Self::follow_max_of_3_consecutive_figures(&value)
    }

    fn is_only_composed_by_roman_figures(value: &String) -> bool {
        value.trim().chars().all(Self::is_roman_character)
    }

    fn is_roman_character(character: char) -> bool {
        matches!(character, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M')
    }

    fn is_numerically_valid(value: &String) -> bool {
        let invalid_combinations =
            ["VX", "IL", "VL", "LC", "IC", "VC", "ID", "VD", "XD", "LD",
            "IM", "VM", "XM", "LM", "DM", "IIV", "IXV", "IVI"];
        for &combination in &invalid_combinations {
            if value.contains(combination) {
                return false;
            }
        }
        true
    }

    fn follow_max_of_3_consecutive_figures(value: &String) -> bool {
        const MAX_REPETITION: u8 = 4;
        let mut count = 1;
        let mut prev_char = value.chars().next();
        for current_char in value.chars().skip(1) {
            if Some(current_char) == prev_char {
                count += 1;
                if count == MAX_REPETITION {
                    return false;
                }
            } else {
                count = 1;
            }
            prev_char = Some(current_char);
        }
        true
    }

    pub fn empty_expression() -> RomanExpression {
        RomanExpression{value: String::new()}
    }
}