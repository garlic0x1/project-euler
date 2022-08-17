/*

For a number written in Roman numerals to be considered valid there are basic
rules which must be followed. Even though the rules allow some numbers to be
expressed in more than one way there is always a "best" way of writing a particular number.

For example, it would appear that there are at least six ways of writing the number sixteen:

IIIIIIIIIIIIIIII
VIIIIIIIIIII
VVIIIIII
XIIIIII
VVVI
XVI

However, according to the rules only XIIIIII and XVI are valid, and the last
example is considered to be the most efficient, as it uses the least number of numerals.

The 11K text file, roman.txt (right click and 'Save Link/Target As...'), contains
one thousand numbers written in valid, but not necessarily minimal, Roman numerals;
see About... Roman Numerals for the definitive rules for this problem.

Find the number of characters saved by writing each of these in their minimal form.

Note: You can assume that all the Roman numerals in the file contain no more than four consecutive identical units.

*/

pub fn pe89() {
    let romans = std::fs::read_to_string("p089_roman.txt").unwrap();
    let sol = romans
        .lines()
        .map(|roman| roman.len() - to_roman(parse_roman(roman)).len())
        .sum::<usize>();

    println!("{}", sol);
    println!("{}", parse_roman(&to_roman(122)));
}

pub fn to_roman(n: u64) -> String {
    if n >= 1000 {
        format!("M{}", to_roman(n - 1000))
    } else if n >= 900 {
        format!("CM{}", to_roman(n - 900))
    } else if n >= 500 {
        format!("D{}", to_roman(n - 500))
    } else if n >= 400 {
        format!("CD{}", to_roman(n - 400))
    } else if n >= 100 {
        format!("C{}", to_roman(n - 100))
    } else if n >= 90 {
        format!("XC{}", to_roman(n - 90))
    } else if n >= 50 {
        format!("L{}", to_roman(n - 50))
    } else if n >= 40 {
        format!("XL{}", to_roman(n - 40))
    } else if n >= 10 {
        format!("X{}", to_roman(n - 10))
    } else if n >= 9 {
        format!("IX{}", to_roman(n - 9))
    } else if n >= 5 {
        format!("V{}", to_roman(n - 5))
    } else if n >= 4 {
        format!("IV{}", to_roman(n - 4))
    } else if n >= 1 {
        format!("I{}", to_roman(n - 1))
    } else {
        String::new()
    }
}

pub fn parse_roman(roman: &str) -> u64 {
    let chars = roman.chars().peekable();
    chars
        .enumerate()
        .map(|(i, char)| match char {
            'I' => {
                if let Some(next) = roman.as_bytes().get(i + 1) {
                    match *next as char {
                        'V' | 'X' => -1,
                        _ => 1,
                    }
                } else {
                    1
                }
            }
            'V' => 5,
            'X' => {
                if let Some(next) = roman.as_bytes().get(i + 1) {
                    match *next as char {
                        'L' | 'C' => -10,
                        _ => 10,
                    }
                } else {
                    10
                }
            }
            'L' => 50,
            'C' => {
                if let Some(next) = roman.as_bytes().get(i + 1) {
                    match *next as char {
                        'D' | 'M' => -100,
                        _ => 100,
                    }
                } else {
                    100
                }
            }
            'D' => 500,
            'M' => 1000,
            _ => 0,
        })
        .sum::<i64>() as u64
}
