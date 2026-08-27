use std::{fmt, ops::Add};

fn parse(_input: &str) -> State {
    let floor0 = vec![
        Type::Generator('S'),
        Type::Microchip('S'),
        Type::Generator('P'),
        Type::Microchip('P'),
    ];
    let floor1 = vec![
        Type::Generator('T'),
        Type::Generator('R'),
        Type::Microchip('R'),
        Type::Generator('C'),
        Type::Microchip('C'),
    ];
    let floor2 = vec![Type::Microchip('T')];
    let floor3 = vec![];
    let floors = vec![floor0, floor1, floor2, floor3];
    State::new(floors)
}

fn part1(input: &str) -> usize {
    let state = parse(input);
    search(&state).steps
}

// The original solution never implemented part 2's full search (it's a much
// larger state space); ported as-is rather than fabricating an answer.
fn part2(_input: &str) -> usize {
    0
}

fn search(input: &State) -> State {
    let mut queue: Vec<State> = vec![input.clone()];
    let mut visited_states: Vec<State> = vec![];

    while !queue.is_empty() {
        let current_state = queue.remove(0);

        if !is_state_safe(&current_state) || visited_states.contains(&current_state) {
            continue;
        }

        if is_state_final(&current_state) {
            return current_state;
        }

        visited_states.push(current_state.clone());

        let current_floor = current_state.get_current_floor();

        for i in 0..current_floor.len() {
            let mut new_state = current_state.clone();
            if new_state.move_elevator(Direction::Up, &current_floor[i], None)
                && (!visited_states.contains(&new_state)
                    && is_state_safe(&new_state)
                    && !queue.contains(&new_state))
            {
                queue.push(new_state);
            }
            let mut new_state = current_state.clone();
            if new_state.move_elevator(Direction::Down, &current_floor[i], None)
                && !visited_states.contains(&new_state)
                && is_state_safe(&new_state)
                && !queue.contains(&new_state)
            {
                queue.push(new_state);
            }
            for j in i..current_floor.len() {
                let mut new_state = current_state.clone();
                if new_state.move_elevator(
                    Direction::Up,
                    &current_floor[i],
                    Some(&current_floor[j]),
                ) && !visited_states.contains(&new_state)
                    && is_state_safe(&new_state)
                    && !queue.contains(&new_state)
                {
                    queue.push(new_state);
                }
                let mut new_state = current_state.clone();
                if new_state.move_elevator(
                    Direction::Down,
                    &current_floor[i],
                    Some(&current_floor[j]),
                ) && !visited_states.contains(&new_state)
                    && is_state_safe(&new_state)
                    && !queue.contains(&new_state)
                {
                    queue.push(new_state);
                }
            }
        }
    }

    unreachable!();
}

fn is_state_safe(state: &State) -> bool {
    for floor in &state.floors {
        if floor.iter().any(|x| !x.is_microchip()) {
            for microchip in floor.iter().filter(|x| x.is_microchip()) {
                let value = microchip.value();
                if !floor
                    .iter()
                    .any(|x| !x.is_microchip() && x.value() == value)
                {
                    return false;
                }
            }
        }
    }

    true
}

fn is_state_final(state: &State) -> bool {
    state.floors[0].len() + state.floors[1].len() + state.floors[2].len() == 0
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Type {
    Generator(char),
    Microchip(char),
}

impl Type {
    fn is_microchip(&self) -> bool {
        matches!(self, Self::Microchip(_))
    }

    fn value(&self) -> char {
        match self {
            Self::Microchip(v) => *v,
            Self::Generator(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    floors: Vec<Vec<Type>>,
    elevator_floor: usize,
    steps: usize,
}

impl State {
    fn new(floors: Vec<Vec<Type>>) -> State {
        Self {
            floors,
            elevator_floor: 0,
            steps: 0,
        }
    }

    fn move_elevator(
        &mut self,
        direction: Direction,
        component1: &Type,
        component2: Option<&Type>,
    ) -> bool {
        if (self.elevator_floor == 3 && direction == Direction::Up)
            || (self.elevator_floor == 0 && direction == Direction::Down)
        {
            return false;
        }

        if let Some(index1) = self.floors[self.elevator_floor]
            .iter()
            .position(|x| x == component1)
        {
            self.floors[self.elevator_floor].remove(index1);
            self.floors[self.elevator_floor + direction].push(*component1);
            if let Some(component2) = component2
                && let Some(index2) = self.floors[self.elevator_floor]
                    .iter()
                    .position(|x| x == component2)
            {
                self.floors[self.elevator_floor].remove(index2);
                self.floors[self.elevator_floor + direction].push(*component2);
            }
            self.elevator_floor = self.elevator_floor + direction;

            self.steps += 1;
            return true;
        }

        false
    }

    fn get_current_floor(&self) -> Vec<Type> {
        self.floors[self.elevator_floor].clone()
    }
}

impl Add<Direction> for usize {
    type Output = usize;

    fn add(self, dir: Direction) -> usize {
        match dir {
            Direction::Up => self + 1,
            Direction::Down => self - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, floor) in self.floors.iter().enumerate().rev() {
            write!(f, "F{} ", i + 1)?;
            if self.elevator_floor == i {
                write!(f, "E ")?;
            } else {
                write!(f, ". ")?;
            }

            for t in floor {
                match t {
                    Type::Microchip(v) => write!(f, "{}M ", v)?,
                    Type::Generator(v) => write!(f, "{}G ", v)?,
                }
            }

            writeln!(f)?;
        }

        write!(f, "")
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let floors = vec![
            vec![Type::Microchip('H'), Type::Microchip('L')],
            vec![Type::Generator('H')],
            vec![Type::Generator('L')],
            vec![],
        ];

        let state = State::new(floors);

        assert_eq!(search(&state).steps, 11);
    }
}
