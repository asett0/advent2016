use std::fs;

enum Direction {
    North,
    East,
    West,
    South,
}

enum Move {
    L(u32),
    R(u32),
}

struct Position {
    d: Direction,
    x: i32,
    y: i32,
}

fn main() {
    let final_pos = fs::read_to_string("input.txt").unwrap().split(", ").fold(
        Position {
            d: Direction::North,
            x: 0,
            y: 0,
        },
        |pos, instruction| make_move(parse_instruction(instruction), pos),
    );

    let part_a = final_pos.x.abs() + final_pos.y.abs();

    println!("The answer to part A is {}", part_a);
}

fn parse_instruction(instruction: &str) -> Move {
    let mut cs = instruction.chars();

    match cs.next().unwrap() {
        'R' => Move::R(cs.collect::<String>().parse::<u32>().unwrap()),
        'L' => Move::L(cs.collect::<String>().parse::<u32>().unwrap()),
        _ => {
            panic!("Invalid move provided in input!");
        }
    }
}

fn make_move(m: Move, p: Position) -> Position {
    match m {
        Move::L(n) => match p.d {
            Direction::North => Position {
                d: Direction::West,
                x: p.x - (n as i32),
                y: p.y,
            },
            Direction::East => Position {
                d: Direction::North,
                x: p.x,
                y: p.y + (n as i32),
            },
            Direction::West => Position {
                d: Direction::South,
                x: p.x,
                y: p.y - (n as i32),
            },
            Direction::South => Position {
                d: Direction::East,
                x: p.x + (n as i32),
                y: p.y,
            },
        },
        Move::R(n) => match p.d {
            Direction::North => Position {
                d: Direction::East,
                x: p.x + (n as i32),
                y: p.y,
            },
            Direction::East => Position {
                d: Direction::South,
                x: p.x,
                y: p.y - (n as i32),
            },
            Direction::West => Position {
                d: Direction::North,
                x: p.x,
                y: p.y + (n as i32),
            },
            Direction::South => Position {
                d: Direction::West,
                x: p.x - (n as i32),
                y: p.y,
            },
        },
    }
}
