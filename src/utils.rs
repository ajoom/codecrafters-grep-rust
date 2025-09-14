use crate::pattern::RegPattern;

pub fn patterns_to_vec(pattern: &str) -> Vec<RegPattern> {
    let mut pattern_ind = 0;
    let mut result = vec![];

    while pattern_ind < pattern.len() {
        if (pattern_ind + 1) < pattern.len() {

            // look for digit pattern
            if &pattern[pattern_ind..pattern_ind + 2] == r"\d" {
                result.push(RegPattern::Digit);
                pattern_ind += 2;
                continue;
            }

            // look for word pattern
            if &pattern[pattern_ind..pattern_ind + 2] == r"\w" {
                result.push(RegPattern::Word);
                pattern_ind += 2;
                continue;
            }

            // look for positive/ negative group pattern
            if pattern.chars().nth(pattern_ind).unwrap() == '[' {
                let closing_bracket_ind = pattern[pattern_ind..].find(']');

                if closing_bracket_ind.is_none() {
                    panic!("Unhandled pattern: {pattern}");
                }

                let closing_bracket_ind = closing_bracket_ind.unwrap() + pattern_ind;

                if pattern.chars().nth(pattern_ind + 1).unwrap() == '^' {
                    result.push(RegPattern::NegativeGroup(
                        pattern[pattern_ind + 2..closing_bracket_ind].to_string(),
                    ));
                } else {
                    result.push(RegPattern::PositiveGroup(
                        pattern[pattern_ind + 1..closing_bracket_ind].to_string(),
                    ));
                }

                pattern_ind = closing_bracket_ind + 1;
                continue;
            }
        }

        // literal pattern :)
        result.push(RegPattern::Literal(
            pattern.chars().nth(pattern_ind).unwrap(),
        ));
        pattern_ind += 1;
    }

    result
}

pub fn match_pattern_with_char(pattern: &RegPattern, c: char) -> bool {
    match pattern {
        RegPattern::Digit => c.is_ascii_digit(),

        RegPattern::Word => c.is_ascii_alphanumeric(),

        RegPattern::PositiveGroup(group) => group.chars().any(|gc| gc == c),

        RegPattern::NegativeGroup(group) => group.chars().all(|gc| gc != c),

        RegPattern::Literal(l) => c == *l,
    }
}
