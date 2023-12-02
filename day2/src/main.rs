use anyhow::{anyhow, Result};
use std::io;

static MAX_RED: usize = 12;
static MAX_GREEN: usize = 13;
static MAX_BLUE: usize = 14;

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    cube_reveals: Vec<CubeReveal>,
}

#[derive(Clone, Debug)]
struct CubeReveal {
    red_count: usize,
    blue_count: usize,
    green_count: usize,
}

impl Default for CubeReveal {
    fn default() -> Self {
        Self {
            red_count: 0,
            blue_count: 0,
            green_count: 0,
        }
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

fn string_to_color(s: &str) -> Result<Color, anyhow::Error> {
    match s {
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "blue" => Ok(Color::Blue),
        _ => Err(anyhow!("invalid color string: ".to_owned() + s)),
    }
}

fn game_is_possible(g: Game) -> bool {
    g.cube_reveals.iter().fold(true, |acc, g| {
        g.red_count <= MAX_RED && g.blue_count <= MAX_BLUE && g.green_count <= MAX_GREEN && acc
    })
}

fn update_cube_reveal(orig_reveal: CubeReveal, new_reveal_string: &str) -> CubeReveal {
    let count = new_reveal_string
        .trim()
        .split(" ")
        .nth(0)
        .unwrap()
        .parse::<usize>()
        .expect("couldn't parse count");
    let color_string = new_reveal_string.trim().split(" ").nth(1).unwrap();
    match string_to_color(color_string).unwrap() {
        Color::Red => CubeReveal {
            red_count: orig_reveal.red_count + count,
            ..orig_reveal
        },
        Color::Blue => CubeReveal {
            blue_count: orig_reveal.blue_count + count,
            ..orig_reveal
        },
        Color::Green => CubeReveal {
            green_count: orig_reveal.green_count + count,
            ..orig_reveal
        },
    }
}

fn parse_game(s: &str) -> Game {
    let first_space_idx = s.find(" ").expect("found no space");
    let first_colon_idx = s.find(":").expect("found no colon");
    let game_id_string = &s[first_space_idx + 1..first_colon_idx];
    let id = game_id_string
        .parse::<usize>()
        .expect(&("failed to parse gameid from:".to_owned() + game_id_string));

    let reveal_strings = &s[first_colon_idx + 1..].split(";");
    let cube_reveals: Vec<CubeReveal> = reveal_strings
        .clone()
        .into_iter()
        .map(|reveal_string| {
            reveal_string
                .split(",")
                .fold(CubeReveal::default(), update_cube_reveal)
        })
        .collect();
    Game { id, cube_reveals }
}

fn part_one(games: Vec<Game>) {
    let result = games
        .into_iter()
        .filter(|g| game_is_possible(g.clone()))
        .fold(0, |acc, g| acc + g.id);
    println!("{}", result);
}

fn minimum_power(g: Game) -> usize {
    let mut max_red = None;
    let mut max_green = None;
    let mut max_blue = None;

    for r in g.cube_reveals {
        if max_red.is_none() || r.red_count > max_red.unwrap() {
            max_red = Some(r.red_count);
        }
        if max_blue.is_none() || r.blue_count > max_blue.unwrap() {
            max_blue = Some(r.blue_count);
        }
        if max_green.is_none() || r.green_count > max_green.unwrap() {
            max_green = Some(r.green_count);
        }
    }

    max_blue.unwrap() * max_green.unwrap() * max_red.unwrap()
}

fn part_two(games: Vec<Game>) {
    let result = games
        .into_iter()
        .map(|g| minimum_power(g))
        .fold(0, |acc, power| acc + power);
    println!("{}", result);
}

fn main() {
    let mut games: Vec<Game> = Vec::new();

    let mut lines = io::stdin().lines();
    while let Some(l) = lines.next() {
        let line = l.unwrap();
        let game = parse_game(&line);
        games.push(game);
    }

    part_one(games.clone());
    part_two(games);
}
