//! BEWARE!
//! this is a clusterfuck of undocumented code and I already
//! spent too much time on it to consider refactoring, be safe

#[rustfmt::skip]
static ADJS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn is_number(value: &SchemaValue) -> bool {
    match value {
        SchemaValue::NumStr(_) => true,
        _ => false,
    }
}

#[derive(Copy, Clone, Debug)]
enum SchemaValue {
    NumStr(char),
    Symbol(char),
    Gear,
    Dot,
}

#[derive(Clone)]
struct Schematic {
    data: Vec<Vec<SchemaValue>>,
    row_sz: usize,
    col_sz: usize,
}

fn parse_schematics(line: &str) -> Schematic {
    let values = line.lines().map(parse_schematics_line).collect::<Vec<_>>();

    Schematic {
        row_sz: values.len(),
        col_sz: values.get(0).map_or(0, |v| v.len()),
        data: values,
    }
}

fn parse_schematics_line(line: &str) -> Vec<SchemaValue> {
    line.chars()
        .map(|c| match c {
            '.' => SchemaValue::Dot,
            '*' => SchemaValue::Gear,
            '0'..='9' => SchemaValue::NumStr(c),
            _ => SchemaValue::Symbol(c),
        })
        .collect()
}

fn parse_number(schematic: &Schematic, initial_row: usize, initial_col: usize) -> (String, usize) {
    let mut number = String::new();
    let mut idxc = initial_col;

    loop {
        if idxc >= schematic.col_sz {
            break;
        }

        let value = schematic.data[initial_row][idxc];

        match value {
            SchemaValue::NumStr(num) => {
                number.push(num);
                idxc += 1;
            }
            _ => {
                break;
            }
        }
    }

    (number, idxc - initial_col)
}

mod part1 {
    use crate::{ADJS, parse_number, SchemaValue, Schematic};

    fn is_symbol(value: &SchemaValue) -> bool {
        match value {
            SchemaValue::Symbol(_) => true,
            SchemaValue::Gear => true,
            _ => false,
        }
    }

    fn check_adjs(m: &Vec<Vec<SchemaValue>>, idxl: usize, idxc: usize) -> bool {
        let m_row_size = m.len() as isize;
        let m_col_size = m[0].len() as isize; // assuming all rows are of the same size

        for (row, col) in ADJS {
            let adj_row = idxl as isize + row as isize;
            let adj_col = idxc as isize + col as isize;

            if adj_row < 0 || adj_col < 0 || adj_row >= m_row_size - 1 || adj_col >= m_col_size - 1
            {
                continue;
            }

            if is_symbol(&m[adj_row as usize][adj_col as usize]) {
                return true;
            }
        }

        false
    }

    fn check_str_adjs(
        m: &Vec<Vec<SchemaValue>>,
        str: &str,
        initial_row: usize,
        initial_col: usize,
    ) -> bool {
        for (idx, _) in str.chars().enumerate() {
            if check_adjs(&m, initial_row, initial_col + idx) {
                return true;
            }
        }

        false
    }

    pub fn solve(schematic: &Schematic) -> u32 {
        let mut idxl = 0;
        let mut result: Vec<u32> = vec![];

        while idxl < schematic.row_sz {
            let mut idxc = 0;

            while idxc < schematic.col_sz {
                let (num_str, qty_processed_values) = parse_number(schematic, idxl, idxc);

                if num_str.is_empty() {
                    idxc += 1;
                    continue;
                }

                if check_str_adjs(&schematic.data, &num_str, idxl, idxc) {
                    result.push(num_str.parse::<u32>().unwrap());
                }

                idxc += qty_processed_values;
            }

            idxl += 1;
        }

        result.iter().fold(0u32, |acc, act| acc + act)
    }
}

mod part2 {
    use std::collections::HashSet;

    use crate::{parse_number, SchemaValue, Schematic};

    fn find_adjacent_numbers(schematic: &Schematic, row: usize, col: usize) -> Vec<(usize, usize)> {
        let m_row_size = schematic.row_sz as isize;
        let m_col_size = schematic.col_sz as isize;

        let mut results = vec![];
        for (adj_row, adj_col) in crate::ADJS {
            let adj_row = row as isize + adj_row as isize;
            let adj_col = col as isize + adj_col as isize;

            if adj_row < 0 || adj_col < 0 || adj_row >= m_row_size || adj_col >= m_col_size {
                continue;
            }

            if crate::is_number(&schematic.data[adj_row as usize][adj_col as usize]) {
                results.push((adj_row as usize, adj_col as usize));
            }
        }

        results
    }

    fn find_start_of_number(schematic: &Schematic, row: usize, col: usize) -> (usize, usize) {
        let mut start_col = col;

        loop {
            if start_col == 0 {
                break;
            }

            let value = schematic.data[row][start_col - 1];

            if !crate::is_number(&value) {
                break;
            }

            start_col -= 1;
        }

        (row, start_col)
    }

    pub fn solve(schematic: &Schematic) -> u32 {
        let mut idxl = 0;
        let mut result: Vec<u32> = vec![];

        while idxl < schematic.row_sz {
            let mut idxc = 0;

            while idxc < schematic.col_sz {
                let value = schematic.data[idxl][idxc];

                match value {
                    SchemaValue::Gear => {
                        let idx_numbers = find_adjacent_numbers(schematic, idxl, idxc);

                        let numbers = idx_numbers
                            .iter()
                            .map(|(num_row, num_col)| {
                                find_start_of_number(schematic, *num_row, *num_col)
                            })
                            // note: this results in the same number being parsed more than once
                            // if more than one digit was an adjacent of the gear.
                            .map(|(num_row, num_col)| parse_number(schematic, num_row, num_col))
                            .map(|(num_str, _)| num_str.parse::<u32>().unwrap())
                            // on the other hand, if a valid gear had the same number on two sides,
                            // this would incorrectly discard it
                            .collect::<HashSet<_>>()
                            .into_iter()
                            .collect::<Vec<_>>();

                        if numbers.len() == 2 {
                            result.push(numbers[0] * numbers[1]);
                        }

                        idxc += 1;
                    }
                    _ => {
                        idxc += 1;
                        continue;
                    }
                };
            }

            idxl += 1;
        }

        result.iter().fold(0u32, |acc, act| acc + act)
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

    let schema = parse_schematics(&contents);
    let result_1 = part1::solve(&schema);
    let result_2 = part2::solve(&schema);

    println!("part 1: {}", result_1);
    println!("part 2: {}", result_2);
}
