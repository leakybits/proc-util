use itertools::Itertools;

fn head(str: impl ToString) -> String {
    str.to_string().chars().take(1).collect()
}

pub fn to_snake(str: impl ToString) -> String {
    str.to_string()
        .chars()
        .tuple_windows()
        .fold(head(str).to_lowercase(), |res, (lhs, rhs)| {
            if lhs.is_lowercase() && rhs.is_uppercase() {
                res + "_" + &rhs.to_lowercase().to_string()
            } else {
                res + &rhs.to_lowercase().to_string()
            }
        })
}

pub fn to_camel(str: impl ToString) -> String {
    str.to_string().chars().tuple_windows().fold(
        head(str).to_lowercase(),
        |res, (lhs, rhs)| match (lhs, rhs) {
            (_, '_') => res + "",
            ('_', _) => res + &rhs.to_uppercase().to_string(),
            (_, _) => res + &rhs.to_string(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake() {
        let case = [
            // --- empty ---
            ("", ""),
            // --- single character ---
            ("a", "a"),
            ("A", "a"),
            ("_", "_"),
            // --- all lowercase ---
            ("foo", "foo"),
            ("föö", "föö"),
            ("foobar", "foobar"),
            ("fööbar", "fööbar"),
            // --- already snake case ---
            ("foo_bar", "foo_bar"),
            ("föö_bar", "föö_bar"),
            ("foo_bar_baz", "foo_bar_baz"),
            ("föö_bar_baz", "föö_bar_baz"),
            // --- camel case ---
            ("fooBar", "foo_bar"),
            ("fööBär", "föö_bär"),
            ("fooBarBaz", "foo_bar_baz"),
            ("fööBärBaz", "föö_bär_baz"),
            // --- pascal case ---
            ("FooBar", "foo_bar"),
            ("FööBär", "föö_bär"),
            ("FooBarBaz", "foo_bar_baz"),
            ("FööBärBaz", "föö_bär_baz"),
        ];

        for (input, expected) in &case {
            assert_eq!(&to_snake(input), expected);
        }
    }

    #[test]
    fn test_to_camel() {
        let case = [
            // // --- empty ---
            ("", ""),
            // // --- single character ---
            ("a", "a"),
            ("A", "a"),
            ("_", "_"),
            // // --- all lowercase ---
            ("foo", "foo"),
            ("föö", "föö"),
            ("foobar", "foobar"),
            ("fööbar", "fööbar"),
            // // --- already camel case ---
            ("fooBar", "fooBar"),
            ("fööBär", "fööBär"),
            ("fooBarBaz", "fooBarBaz"),
            ("fööBärBaz", "fööBärBaz"),
            // // --- snake case ---
            ("foo_bar", "fooBar"),
            ("föö_bar", "fööBar"),
            ("foo_bar_baz", "fooBarBaz"),
            ("föö_bar_baz", "fööBarBaz"),
            // // --- pascal case ---
            ("FooBar", "fooBar"),
            ("FööBär", "fööBär"),
            ("FooBarBaz", "fooBarBaz"),
            ("FööBärBaz", "fööBärBaz"),
        ];

        for (input, expected) in &case {
            assert_eq!(&to_camel(input), expected);
        }
    }
}
