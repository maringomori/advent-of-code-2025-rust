use std::u64;

advent_of_code::solution!(6);

pub fn transpose(input: &str) -> Vec<Vec<String>> {
    let rows: Vec<Vec<&str>> =
        input.lines()
             .map(|line| line.split_whitespace().collect())
             .collect();

    let max_cols = rows.iter().map(|r| r.len()).max().unwrap();

    let mut cols: Vec<Vec<String>> = vec![Vec::new(); max_cols];

    for row in rows {
        for (i, field) in row.iter().enumerate() {
            cols[i].push((*field).to_string());
        }
    }

    cols
}

pub fn part_one(input: &str) -> Option<u64> {
    let cols = transpose(input);

    let mut sum = 0;
    for col in cols {
        let mut nums = Vec::<u64>::new();

        for (i, val) in col.iter().enumerate() {
            if i == col.len() - 1 {
                if val == "+" {
                    sum += nums.iter().sum::<u64>();
                } else if val == "*" {
                    sum += nums.iter().product::<u64>();
                
                }

            } else {
                nums.push(val.parse::<u64>().unwrap());
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows: Vec<String> = input.lines().map(String::from).collect();

    let num_cols = rows[0].len();
    let mut result = String::new();
    let mut sum = 0;

    for i in (0..num_cols).rev() {

        for row in rows.iter() {
            let char_at_col = row.chars().nth(i);
            result.push(char_at_col.unwrap());
        }

        if result.contains("+") || result.contains("*") {
            let before_op = &result[0..result.len()-1]; 
            let nums: Vec<u64> = before_op.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect(); 
            if result.contains("+") {
                sum += nums.iter().sum::<u64>();
            } else if result.contains("*") {
                sum += nums.iter().product::<u64>();
            }
            result.clear();
        }
    }

    Some(sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
