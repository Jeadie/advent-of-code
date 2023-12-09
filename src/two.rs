use std::{fmt, str::FromStr, num::ParseIntError, error::Error};

/**
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the 
surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. 
It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be
 happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get 
 many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. 
Each time you play this game, he will hide a secret number of cubes of each color in the bag, and
 your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a
 handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few
  times per game.

You play several games and record the information from each game (your puzzle input). Each game is
 listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of
  subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set 
is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; 
the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12
 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with
that configuration. However, game 3 would have been impossible because at one point the Elf showed
 you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed
you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you
 get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
 */

const INPUT_FILEPATH: &'static str = "src/assets/two.txt";

struct Game {
    pub id: u64,
    pub cube_sets: Vec<CubeSet>
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(":").collect();
        let id = parts[0].split_whitespace().last().unwrap().parse()?;

        let cubes: Vec<CubeSet> = parts[1].split("; ").map(|set| {
            set.parse::<CubeSet>().expect("failed to parse cube")
        }).collect();
        Ok(Game{id: id, cube_sets: cubes})

    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {}: ", self.id)?;
        write!(f, "{}", self.cube_sets
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("; "))
    }
}

impl Game {
    pub fn could_be_from_bag(&self, bag: CubeSet) -> bool {
        let cubes_in_game = self.cube_sets.iter().fold(CubeSet{red: 0, blue: 0,green: 0}, |mut acc: CubeSet, e: &CubeSet| {
            acc.red += e.red;
            acc.blue += e.blue;
            acc.green += e.green;
            acc
        });
        cubes_in_game.green <= bag.green && cubes_in_game.blue <= bag.blue && cubes_in_game.red <= bag.red
    }
}

#[derive(Copy, Clone)]
struct CubeSet {
    pub blue: u64,
    pub green: u64,
    pub red: u64,
}

impl FromStr for CubeSet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        s.split(", ").for_each(|color_count| {
            let color_count_parts: Vec<&str> = color_count.split_whitespace().collect();
            let count: u64 = color_count_parts[0].parse().unwrap();

            match color_count_parts[1].to_lowercase().as_str() {
                "blue" => blue = count,
                "green" => green = count,
                "red" => red = count,
                _ => {}
            }
        });

        Ok(CubeSet{ blue, green, red})
    }
}

impl fmt::Display for CubeSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut components = Vec::new();
        
        if self.blue > 0 {
            components.push(format!("{} blue", self.blue))
        }
        if self.green > 0 {
            components.push(format!("{} green", self.green))
        }
        if self.red > 0 {
            components.push(format!("{} red", self.red))
        }
        write!(f, "{}", components.join(", "))
    }
}

pub fn Two(input: String) -> Result<u64, Box<dyn Error>> {
    let bag = CubeSet{red: 12, green: 13, blue: 14};

    let games: Vec<Game> = input
        .split("\n")
        .map(|l| l.parse::<Game>().unwrap())
        .filter(|g| g.could_be_from_bag(bag))
        .collect();

    Ok(games.iter().fold(0, |mut agg, g: &Game| {agg += g.id; agg}) )

}