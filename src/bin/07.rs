use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let rows: Vec<String> = input.lines().map(String::from).collect();
    let mut paths = HashSet::<usize>::new();
    let mut counter: usize = 0;
    let start_index = rows[0].find('S').unwrap();

    paths.insert(start_index);

    let mut visited_splitters = HashSet::<(usize, usize)>::new();

    for (i, row) in rows.iter().enumerate().skip(1) {
        let mut new_paths = HashSet::<usize>::new();

        for path in &paths {
            let p = *path;
            let ch = row.chars().nth(p).unwrap();

            if ch == '^' {
                if visited_splitters.insert((i, p)) {
                    counter += 1;
                }

                let left = p - 1;
                if !new_paths.contains(&left) {
                    new_paths.insert(left);
                }

                let right = p + 1;
                if !new_paths.contains(&right) {
                    new_paths.insert(right);
                }

            } else if ch == '.' {
                new_paths.insert(p);
            }
        }

        paths = new_paths;
    }

    Some(counter as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows: Vec<String> = input.lines().map(String::from).collect();
    let mut paths = HashMap::<usize, usize>::new();
    
    let start_index = rows[0].find('S').unwrap();

    paths.insert(start_index, 1);

   for row in &rows[1..] {
        let mut new_paths = HashMap::<usize, usize>::new();

        for (path, &n) in &paths {

            let p = *path;
            let ch = row.chars().nth(p).unwrap();

            if ch == '^' {
                *new_paths.entry(p - 1).or_insert(0) += n;
                *new_paths.entry(p + 1).or_insert(0) += n;
            } else if ch == '.' {
                *new_paths.entry(p).or_insert(0) += n;
            }
        }

        paths = new_paths;
    }

   Some(paths.values().sum::<usize>() as u64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
