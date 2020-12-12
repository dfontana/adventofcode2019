use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, str::FromStr};

#[derive(Debug)]
enum Action {
  N(i32),
  S(i32),
  E(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32),
}

#[derive(Clone)]
enum Direction {
  North,
  East,
  West,
  South,
}

pub struct Solve {
  actions: Vec<Action>,
}

impl FromStr for Action {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (act, amt) = (
      &s[..1],
      &s[1..]
        .parse::<i32>()
        .map_err(|_| "Can't parse amt".to_string())?,
    );
    match act {
      "N" => Ok(Action::N(*amt)),
      "S" => Ok(Action::S(*amt)),
      "E" => Ok(Action::E(*amt)),
      "W" => Ok(Action::W(*amt)),
      "L" => Ok(Action::L(*amt)),
      "R" => Ok(Action::R(*amt)),
      "F" => Ok(Action::F(*amt)),
      _ => Err("Unknown action type".to_string()),
    }
  }
}

impl Direction {
  fn turn(&self, act: &Action) -> Direction {
    let upper = match act {
      Action::R(amt) | Action::L(amt) => amt,
      _ => return self.clone(),
    };
    let mut crt = 0;
    let mut fin = self.clone();
    while crt < *upper {
      fin = match act {
        Action::L(_) => match fin {
          Direction::North => Direction::West,
          Direction::West => Direction::South,
          Direction::South => Direction::East,
          Direction::East => Direction::North,
        },
        Action::R(_) => match fin {
          Direction::North => Direction::East,
          Direction::East => Direction::South,
          Direction::South => Direction::West,
          Direction::West => Direction::North,
        },
        _ => return fin,
      };
      crt += 90;
    }
    fin
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      actions: read_input(d)?
        .lines()
        .map(Action::from_str)
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let mut dir = Direction::East;
    let (mut x, mut y) = (0, 0);
    for act in self.actions.iter() {
      match act {
        Action::L(_) | Action::R(_) => dir = dir.turn(act),
        Action::F(amt) => match dir {
          Direction::North => y += amt,
          Direction::East => x += amt,
          Direction::South => y -= amt,
          Direction::West => x -= amt,
        },
        Action::N(amt) => y += amt,
        Action::S(amt) => y -= amt,
        Action::E(amt) => x += amt,
        Action::W(amt) => x -= amt,
      }
    }
    Ok((x.abs() + y.abs()).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let (mut ship_x, mut ship_y) = (0, 0);
    let (mut wp_x, mut wp_y) = (10, 1);
    for act in self.actions.iter() {
      match act {
        Action::L(amt) => {
          let f = -to_rad(*amt as f32);
          let a_x = (((wp_x as f32) * f32::cos(f)) + ((wp_y as f32) * f32::sin(f))).round() as i32;
          let a_y = (((wp_y as f32) * f32::cos(f)) - ((wp_x as f32) * f32::sin(f))).round() as i32;
          wp_x = a_x;
          wp_y = a_y;
        }
        Action::R(amt) => {
          let f = to_rad(*amt as f32);
          let a_x = (((wp_x as f32) * f32::cos(f)) + ((wp_y as f32) * f32::sin(f))).round() as i32;
          let a_y = (((wp_y as f32) * f32::cos(f)) - ((wp_x as f32) * f32::sin(f))).round() as i32;
          wp_x = a_x;
          wp_y = a_y;
        }
        Action::F(amt) => {
          ship_x += wp_x * amt;
          ship_y += wp_y * amt;
        }
        Action::N(amt) => wp_y += amt,
        Action::S(amt) => wp_y -= amt,
        Action::E(amt) => wp_x += amt,
        Action::W(amt) => wp_x -= amt,
      }
      // println!("Ship: ({},{}). WP: ({},{})", ship_x, ship_y, wp_x, wp_y);
    }
    Ok((ship_x.abs() + ship_y.abs()).to_string())
  }
}

fn to_rad(x: f32) -> f32 {
  x * (std::f32::consts::PI / 180.0)
}