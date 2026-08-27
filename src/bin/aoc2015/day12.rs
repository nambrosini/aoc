use serde_json::Value;

fn part1(input: &str) -> i64 {
    let input: Value = serde_json::from_str(input).unwrap();
    calc1(&input)
}

fn calc1(input: &Value) -> i64 {
    let mut sum: i64 = 0;
    match input {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(calc1).sum::<i64>(),
        Value::Object(object) => sum += object.values().map(calc1).sum::<i64>(),
        _ => (),
    }

    sum
}

fn part2(input: &str) -> i64 {
    let input: Value = serde_json::from_str(input).unwrap();
    calc2(&input)
}

fn calc2(input: &Value) -> i64 {
    let mut sum: i64 = 0;

    match input {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(calc2).sum::<i64>(),
        Value::Object(object) => {
            let red_param_detected = object.values().any(|value| {
                if let Value::String(s) = value {
                    return s == "red";
                }
                false
            });
            if !red_param_detected {
                sum += object.values().map(calc2).sum::<i64>();
            }
        }
        _ => (),
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("[1,2,3]"), 6);
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(part2(r#"[1,"red",5]"#), 6);
    }
}
