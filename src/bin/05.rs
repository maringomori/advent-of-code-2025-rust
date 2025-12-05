use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // Can be optimized the same way as prt2 tho
    let mut fresh_ingredients: HashSet<(u64, u64)> = HashSet::new();
    let mut count = 0;

    for i in input.lines() {
        if i.contains("-") {
            let mut fresh_ranges = i.split("-");
            let start: u64 = fresh_ranges.next().unwrap().parse().unwrap();
            let end: u64 = fresh_ranges.next().unwrap().parse().unwrap();
           
            fresh_ingredients.insert((start, end)); 
        } else if i.is_empty() {
            continue;
        } else {


            let digit = i.parse::<u64>().unwrap();
            for j in &fresh_ingredients {
                if j.0 <= digit && digit <= j.1 {
                    count += 1;
                    break;
                } 
            }
        }
    }

    Some(count)
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut fresh_ingredients: Vec<(u64, u64)> = Vec::new();

    for i in input.lines() {
        if i.contains("-") {
            let mut fresh_ranges = i.split("-");
            let start: u64 = fresh_ranges.next().unwrap().parse().unwrap();
            let end: u64 = fresh_ranges.next().unwrap().parse().unwrap();
           
           fresh_ingredients.push((start, end));

        } else if i.is_empty() {
            break;
        } 
    }

    fresh_ingredients.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in fresh_ingredients {
        if let Some(i) = merged.last_mut() {
            if start <= i.1 + 1 {
                i.1 = i.1.max(end);
                continue;
            }
        }

        merged.push((start, end));
    }

    let mut total = 0;
    for (start, end) in merged {
        total += end - start + 1;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
