use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn new(capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64) -> Ingredient {
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }

    fn calc(&self) -> i64 {
        if self.capacity < 0 || self.durability < 0 || self.flavor < 0 || self.texture < 0 {
            return 0;
        }
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Mul<i64> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            capacity: self.capacity + rhs.capacity,
            flavor: self.flavor + rhs.flavor,
            durability: self.durability + rhs.durability,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl TryFrom<&str> for Ingredient {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(' ').collect_vec();

        let capacity = split[2].replace(',', "").parse().unwrap();
        let durability = split[4].replace(',', "").parse().unwrap();
        let flavor = split[6].replace(',', "").parse().unwrap();
        let texture = split[8].replace(',', "").parse().unwrap();
        let calories = split[10].parse().unwrap();

        Ok(Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

fn parse(input: &str) -> Vec<Ingredient> {
    input.lines().map(|l| l.try_into().unwrap()).collect_vec()
}

fn calc_ingredients(ingredients: &[Ingredient]) -> Vec<Ingredient> {
    let mut v = vec![];

    let perm: Vec<Vec<i64>> = (1..=100)
        .permutations(ingredients.len())
        .filter(|x| x.iter().sum::<i64>() == 100)
        .collect_vec();

    for i in perm {
        let mut res = Ingredient::new(0, 0, 0, 0, 0);

        for (ii, &v) in i.iter().enumerate() {
            res = res + (ingredients[ii].clone() * v)
        }

        v.push(res);
    }

    v
}

fn part1(input: &str) -> i64 {
    let ingredients = parse(input);
    let cookies = calc_ingredients(&ingredients);

    cookies.iter().map(|c| c.calc()).max().unwrap()
}

fn part2(input: &str) -> i64 {
    let ingredients = parse(input);
    let cookies = calc_ingredients(&ingredients);

    cookies
        .iter()
        .filter(|c| c.calories == 500)
        .map(|c| c.calc())
        .max()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 62842880);
    }
}
