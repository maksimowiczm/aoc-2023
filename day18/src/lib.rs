use std::str::FromStr;

mod aoc_tests;
mod tests;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Digging {
    direction: Direction,
    length: u64,
}

impl FromStr for Digging {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();

        let direction = match *split
            .get(0)
            .ok_or(format!("Failed to parse direction in {s}"))?
        {
            "U" => Some(Direction::Up),
            "R" => Some(Direction::Right),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            _ => None,
        }
        .ok_or(format!("Failed to parse direction in {s}"))?;

        let length = split
            .get(1)
            .ok_or(format!("Failed to parse length in {s}"))?
            .parse::<u64>()
            .ok()
            .ok_or(format!("Failed to parse length in {s}"))?;

        Ok(Digging { direction, length })
    }
}

struct DiggingPlan(Vec<Digging>);

impl FromStr for DiggingPlan {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let diggings = s
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<Digging>().unwrap())
            .collect::<Vec<_>>();

        Ok(DiggingPlan(diggings))
    }
}

struct Lagoon(Vec<Vec<bool>>);

impl Lagoon {
    fn from_digging_plan(plan: &DiggingPlan) -> Self {
        let mut start = (0, 0);
        let (mut width, mut height) = (0, 0);
        let _ = plan
            .0
            .iter()
            .filter(|digging| match digging.direction {
                Direction::Left | Direction::Right => true,
                _ => false,
            })
            .map(|digging| match digging.direction {
                Direction::Left => digging.length as i128 * -1,
                Direction::Right => digging.length as i128,
                _ => 0i128,
            })
            .reduce(|acc, v| {
                if start.0 > acc {
                    start.0 = acc
                }
                if acc > width {
                    width = acc;
                }
                acc + v
            });
        let _ = plan
            .0
            .iter()
            .filter(|digging| match digging.direction {
                Direction::Up | Direction::Down => true,
                _ => false,
            })
            .map(|digging| match digging.direction {
                Direction::Up => digging.length as i128 * -1,
                Direction::Down => digging.length as i128,
                _ => 0i128,
            })
            .reduce(|acc, v| {
                if start.1 > acc {
                    start.1 = acc
                }
                if height < acc {
                    height = acc
                }
                acc + v
            });
        start.0 *= -1;
        start.1 *= -1;

        let mut lagoon =
            vec![vec![false; (width + start.0) as usize + 1]; (height + start.1) as usize + 1];
        let mut position = (start.0 as usize, start.1 as usize);
        plan.0.iter().for_each(|digging| {
            let to_dig = Self::path(position, digging);
            to_dig.iter().for_each(|&(x, y)| lagoon[y][x] = true);
            position = *to_dig.last().unwrap();
        });

        lagoon.iter_mut().for_each(|row| {
            let mut fill = false;
            let mut prev_cell = false;
            row.iter_mut().for_each(|cell| {
                if *cell == true && prev_cell == false {
                    fill = !fill
                }

                prev_cell = *cell;
                if *cell == false && fill {
                    *cell = true;
                }
            })
        });

        Lagoon(lagoon)
    }

    fn next_position(position: (usize, usize), direction: Direction) -> (usize, usize) {
        match direction {
            Direction::Down => (position.0, position.1 + 1),
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Left => (position.0 - 1, position.1),
        }
    }

    fn path(position: (usize, usize), digging: &Digging) -> Vec<(usize, usize)> {
        let mut position = position;
        (0..digging.length).fold(vec![], |mut vec, i| {
            position = Self::next_position(position, digging.direction);
            vec.push(position);
            vec
        })
    }

    pub fn count_depth(&self) -> u64 {
        self.0.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, v| match v {
                true => acc + 1,
                false => acc,
            })
        })
    }
}

pub fn solution_01(input: &str) -> Result<u64, String> {
    let plan = input.parse::<DiggingPlan>()?;
    let lagoon = Lagoon::from_digging_plan(&plan);
    Ok(lagoon.count_depth())
}
