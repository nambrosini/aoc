use itertools::Itertools;
use std::ops::Add;

#[derive(Clone)]
struct Character {
    hitpoints: usize,
    damage: usize,
    armor: usize,
}

impl Character {
    fn rounds(&self, opponent: &Character) -> u16 {
        let round_damage: f64 = if opponent.armor >= self.damage {
            1
        } else {
            self.damage - opponent.armor
        } as f64;

        ((opponent.hitpoints as f64) / round_damage).ceil() as u16
    }

    fn beats(&self, opponent: &Character) -> bool {
        let a = self.rounds(opponent);
        let b = opponent.rounds(self);

        a <= b
    }
}

#[derive(Debug, Clone)]
struct Inventory {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Add<&Inventory> for &Inventory {
    type Output = Inventory;

    fn add(self, rhs: &Inventory) -> Self::Output {
        Inventory {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

fn inv(cost: usize, damage: usize, armor: usize) -> Inventory {
    Inventory {
        cost,
        damage,
        armor,
    }
}

fn weapons() -> [Inventory; 5] {
    [
        inv(8, 4, 0),
        inv(10, 5, 0),
        inv(25, 6, 0),
        inv(40, 7, 0),
        inv(74, 8, 0),
    ]
}

fn armor_options() -> [Inventory; 5] {
    [
        inv(13, 0, 1),
        inv(31, 0, 2),
        inv(53, 0, 3),
        inv(75, 0, 4),
        inv(102, 0, 5),
    ]
}

fn rings() -> [Inventory; 6] {
    [
        inv(25, 1, 0),
        inv(50, 2, 0),
        inv(100, 3, 0),
        inv(20, 0, 1),
        inv(40, 0, 2),
        inv(80, 0, 3),
    ]
}

fn all_inventories() -> Vec<Inventory> {
    let weapons = weapons();
    let armor = armor_options();
    let rings = rings();

    let mut inventories = vec![];

    for weapon in &weapons {
        let mut candidates = vec![weapon.clone()];

        for a in &armor {
            candidates.push(weapon + a);
        }

        let mut additional = vec![];
        for rings in rings.iter().combinations(2) {
            for candidate in candidates.iter() {
                additional.push(candidate + rings[0]);
                additional.push(candidate + rings[1]);
                additional.push(candidate + &(rings[0] + rings[1]));
            }
        }

        candidates.extend(additional);
        inventories.extend(candidates);
    }

    inventories
}

fn parse(input: &str) -> Character {
    let val = input
        .lines()
        .map(|x| x.split(' ').next_back().unwrap())
        .map(|x| x.parse().unwrap())
        .collect_vec();

    Character {
        hitpoints: val[0],
        damage: val[1],
        armor: val[2],
    }
}

fn part1(input: &str) -> usize {
    let boss = parse(input);
    let mut minimal_cost = usize::MAX;

    for inventory in all_inventories() {
        let player = Character {
            hitpoints: 100,
            damage: inventory.damage,
            armor: inventory.armor,
        };

        if player.beats(&boss) && inventory.cost < minimal_cost {
            minimal_cost = inventory.cost;
        }
    }

    minimal_cost
}

fn part2(input: &str) -> usize {
    let boss = parse(input);
    let mut maximal_cost = 0;

    for inventory in all_inventories() {
        let player = Character {
            hitpoints: 100,
            damage: inventory.damage,
            armor: inventory.armor,
        };

        if !player.beats(&boss) && inventory.cost >= maximal_cost {
            maximal_cost = inventory.cost;
        }
    }

    maximal_cost
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/21.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
