use std::fs::File;
use std::io::prelude::*;

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct GameDraw {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

#[derive(Debug)]
struct Game {
    id: i32,
    draw: Vec<GameDraw>,
}

impl Game {
    fn check(&self, red: i32, green: i32, blue: i32) -> bool {
        for color in vec![Color::Red, Color::Green, Color::Blue] {
            if let Some(max_color) = self.get_max_color(&color) {
                if max_color
                    > match color {
                        Color::Red => red,
                        Color::Green => green,
                        Color::Blue => blue,
                    }
                {
                    return false;
                }
            }
        }

        true
    }

    fn get_max_color(&self, color: &Color) -> Option<i32> {
        let mut max_color = 0;
        for game_draw in &self.draw {
            if let Some(current_color) = match color {
                Color::Red => game_draw.red,
                Color::Green => game_draw.green,
                Color::Blue => game_draw.blue,
            } {
                if current_color > max_color {
                    max_color = current_color;
                }
            }
        }
        if max_color == 0 {
            None
        } else {
            Some(max_color)
        }
    }
}

fn main() {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let mut games: Vec<Game> = Vec::new();
    for line in contents.lines() {
        let game = Game {
            draw: line
                .split(":")
                .nth(1)
                .unwrap()
                .split(";")
                .map(|x| {
                    let rgb = x.trim().split(',');
                    let mut game_draw = GameDraw {
                        red: None,
                        green: None,
                        blue: None,
                    };
                    for item in rgb {
                        let item = item.trim();
                        let mut items = item.split(" ");
                        let (value, color) = (items.next().unwrap(), items.next().unwrap());
                        match color {
                            "red" => game_draw.red = Some(value.parse::<i32>().unwrap()),
                            "green" => game_draw.green = Some(value.parse::<i32>().unwrap()),
                            "blue" => game_draw.blue = Some(value.parse::<i32>().unwrap()),
                            _ => (),
                        }
                    }
                    game_draw
                })
                .collect(),
            id: line
                .split(":")
                .nth(0)
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap(),
        };
        games.push(game);
    }
    let mut sum = 0;
    for game in games {
        let result = game.check(12, 13, 14);
        if result {
            sum += game.id;
        }
        println!("Game {} result: {}", game.id, result);
        println!("Sum: {}", sum);
    }
}
