mod part1 {
    /// Takes the first and last digit found on `line` and returns a
    /// number of the form `(first * 10) + last`
    pub fn extract_line(line: &str) -> u8 {
        let numerics = line
            .chars()
            .filter(|c| *c >= '0' && *c <= '9')
            .map(|c| c as u8 - b'0')
            .collect::<Vec<u8>>();

        // note that in case a line has no numbers at all, it defaults to 0.
        let (fst, lst) = (
            numerics.first().unwrap_or(&0),
            numerics.last().unwrap_or(&0),
        );

        return fst * 10 + lst;
    }

    pub fn solve(input: &str) -> u32 {
        input
            .lines()
            .map(extract_line)
            .fold(0, |acc, act| acc + act as u32)
    }
}

mod part2 {
    const STR_DIGITS: [(&str, u8); 9] = [
        ("one",   1),
        ("two",   2),
        ("three", 3),
        ("four",  4),
        ("five",  5),
        ("six",   6),
        ("seven", 7),
        ("eight", 8),
        ("nine",  9),
    ];

    fn extract_line(line: &str) -> u8 {
	println!("> {}", line);

	let size = line.len();
	let line = line.chars().collect::<Vec<char>>();

	let mut idx = 0;
	let mut digits = vec![];

	'm: while idx < size {
	    let chr = line[idx];
	    println!(">> {}, {}", idx, chr);

	    // if the chr represents a number, we found a match
	    if chr >= '0' && chr <= '9' {
		digits.push(chr as u8 - b'0');
		idx += 1;
		continue;
	    }

	    // otherwise, look up for each of the digit keywords
	    // exclusively starting at `idx`
	    for (str_digit, digit) in STR_DIGITS {
		let str_digit = str_digit.chars();
		let str_digit = &str_digit.collect::<Vec<char>>();

		if line[idx..].starts_with(str_digit) {
		    digits.push(digit);
		    idx += str_digit.len() - 1;
		    continue 'm;
		}
	    }

	    idx += 1;
	}

	println!(">> {:?}", digits);

        // note that in case a line has no numbers at all, it defaults to 0.
        let (fst, lst) = (
            digits.first().unwrap_or(&0),
            digits.last().unwrap_or(&0),
        );

	println!(">> {}", fst * 10 + lst);

        return fst * 10 + lst;
    }

    pub fn solve(input: &str) -> u32 {
        input
            .lines()
            .map(extract_line)
            .fold(0, |acc, act| acc + act as u32)
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("usage: {} <filename>", args[0]);
        return;
    }

    let contents =
        std::fs::read_to_string(&args[1]).expect(&format!("could not read file {}", args[1]));
    let result_1 = part1::solve(&contents);
    let result_2 = part2::solve(&contents);

    println!("part 1: {:?}", result_1);
    println!("part 2: {:?}", result_2);
}
