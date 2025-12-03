advent_of_code::solution!(3);

pub fn find_biggest_digit_in_str(line: &str) -> (u32, usize) {
    let mut max_digit = 0;
    let mut pos = 0;
    let mut found = false;

    for (i, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            if d > max_digit {
                max_digit = d;
                pos = i;
                found = true;
            } else if d == max_digit && !found {
                pos = i;
                found = true;
            }
        }
    }

    (max_digit, pos)
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let (mut num1, pos1) = find_biggest_digit_in_str(line);
        let num2;

        if pos1 < line.len() - 1 {
            let after = &line[pos1 + 1..];
            num2 = find_biggest_digit_in_str(after).0;
        } else {
            let before = &line[..pos1];
            num2 = num1;
            num1 = find_biggest_digit_in_str(before).0;
        }

        sum += (num1 * 10 + num2) as u64;
    }

    return Some(sum.try_into().unwrap());
}

pub fn find_n_biggest(line: &str, n: usize) -> u64 {
    let mut nums = Vec::with_capacity(n);
    let mut start = 0;

    while nums.len() < n {
        let remaining_needed = n - nums.len();
        let mut test_digit = 0;
        let mut test_pos = 0;

        for (i, c) in line[start..].chars().enumerate() {
            let absolute_pos = start + i;
            let remaining_chars = line.len() - absolute_pos;

            if remaining_chars < remaining_needed {
                break; 
            }

            let digit = c.to_digit(10).unwrap();
            if  digit > test_digit {
                test_digit = digit;
                test_pos = absolute_pos;
            }
        }

        let digit = test_digit;
        nums.push(digit);
        start = test_pos + 1;
    }
    
    let mut value: u64 = 0;
    for d in nums {
        value = value * 10 + d as u64;
    }

    return value;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let n = 12;

    for line in input.lines() {
        sum += find_n_biggest(line, n);
    }

    return Some(sum.try_into().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
