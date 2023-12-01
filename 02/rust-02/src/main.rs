enum CubeType {
    Red,
    Blue,
    Green,
}

impl TryFrom<&str> for CubeType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "red" => CubeType::Red,
            "blue" => CubeType::Blue,
            "green" => CubeType::Green,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: u8,
    blue: u8,
    green: u8,
}

fn parse_game(line: &str) -> Game {
    let (game, rest) = line.split_once(':').unwrap();
    let (_, game_id) = game.split_once(' ').unwrap();
    let game_id = game_id.parse::<u8>().unwrap();
    let sets = rest.split(';').collect::<Vec<_>>();

    let mut game = Game {
        id: game_id,
        sets: vec![],
    };

    for set_str in sets {
        let cubes = set_str.split(',').map(|v| v.trim()).collect::<Vec<_>>();
        let mut set = Set {
            red: 0,
            blue: 0,
            green: 0,
        };

        for cube in cubes {
            let (qty, typ) = cube.split_once(' ').unwrap();
            let qty = qty.parse::<u8>().unwrap();
            let typ = CubeType::try_from(typ).unwrap();

            match typ {
                CubeType::Red => set.red += qty,
                CubeType::Blue => set.blue += qty,
                CubeType::Green => set.green += qty,
            }
        }

        game.sets.push(set);
    }

    game
}

mod part1 {
    /// Given `game`, returns its `id` if the following condition is
    /// met, or 0 otherwise.
    /// The condition is:
    ///   + No set must have more than 12 red cubes, 13 green cubes and 14 blue cubes
    pub fn solve(game: crate::Game) -> u8 {
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                return 0;
            }
        }

        return game.id;
    }
}

mod part2 {
    /// Given `game`, returns the result of multiplying the minimum quantity
    /// of red, blue and green cubes needed in order to play the game.
    pub fn solve(game: crate::Game) -> u32 {
	let mut min_red = 0; let mut min_blue = 0; let mut min_green = 0;

	for set in game.sets {
	    if set.red > min_red { min_red = set.red };
	    if set.blue > min_blue { min_blue = set.blue };
	    if set.green > min_green { min_green = set.green };
	}

	return min_red as u32 * min_blue as u32 * min_green as u32;
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

    let result_1 = contents
        .lines()
        .map(parse_game)
        .map(part1::solve)
        .fold(0u32, |acc, act| acc + act as u32);

    let result_2 = contents
        .lines()
        .map(parse_game)
        .map(part2::solve)
        .fold(0u32, |acc, act| acc + act as u32);

    println!("part 1: {}", result_1);
    println!("part 2: {}", result_2);
}
