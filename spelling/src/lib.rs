pub fn spell(n: u64) -> String {
    match n {
        0..=9 => ones(n).to_string(),
        10..=19 => teens(n).to_string(),
        20..=99 => {
            let word = tens_word(n).to_string();
            match n % 10 {
                0 => word,
                _ => format!("{}-{}", word, spell(n % 10)),
            }
        }
        100..=999 => {
            let hundreds = format!("{} hundred", ones(n / 100));
            match n % 100 {
                0 => hundreds,
                _ => format!("{} {}", hundreds, spell(n % 100)),
            }
        }
        1000..=999_999 => {
            let thousands = format!("{} thousand", spell(n / 1000));
            match n % 1000 {
                0 => thousands,
                _ => format!("{} {}", thousands, spell(n % 1000)),
            }
        }
        1_000_000 => "one million".to_string(),
        _ => unreachable!(),
    }
}

fn ones(n: u64) -> &'static str {
    match n {
        0 => "zero", 1 => "one", 2 => "two", 3 => "three", 4 => "four", 5 => "five",
        6 => "six", 7 => "seven", 8 => "eight", 9 => "nine",
        _ => unreachable!(),
    }
}

fn teens(n: u64) -> &'static str {
    match n {
        10 => "ten", 11 => "eleven", 12 => "twelve", 13 => "thirteen", 14 => "fourteen",
        15 => "fifteen", 16 => "sixteen", 17 => "seventeen", 18 => "eighteen", 19 => "nineteen",
        _ => unreachable!(),
    }
}

fn tens_word(n: u64) -> &'static str {
    match n / 10 {
        2 => "twenty", 3 => "thirty", 4 => "forty", 5 => "fifty",
        6 => "sixty", 7 => "seventy", 8 => "eighty", 9 => "ninety",
        _ => unreachable!(),
    }
}