
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut zero_counter = 0;
    let mut starting_pos = 50;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();   
        let num: i32 = line[1..].parse().unwrap(); 

        match dir {
        'L' => {
            if starting_pos - num >= 1 {
                starting_pos -= num;
            } else {
                starting_pos = 100 - ((starting_pos - num) % 100).abs();
                if starting_pos == 100 {
                    starting_pos = 0;
                }
            }
        },
        'R' => {
            if starting_pos + num <= 99{
                starting_pos += num;
            } else {
                starting_pos = (starting_pos + num) % 100 
            }
        },
        _ => {}
        }
        if starting_pos == 0 {
            zero_counter += 1;
        }
    }

    return Some(zero_counter);
}

pub fn part_two(input: &str) -> Option<u64> {

    let mut zero_counter = 0;
    let mut starting_pos = 50;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();   
        let num: i32 = line[1..].parse().unwrap(); 
        let mut rotation = 0;

        match dir {
        'L' => {
            if starting_pos - num >= 1 {
                starting_pos -= num;
            } else {
                if starting_pos != 0 {
                   rotation = (100 + (starting_pos - num).abs()) / 100;
                } else {
                    rotation += num / 100
                }
                starting_pos = 100 - ((starting_pos - num) % 100).abs();
                if starting_pos == 100 {
                    starting_pos = 0;
                }
                
            }
        },
        'R' => {
            if starting_pos + num <= 99{
            
                starting_pos += num;
            } else {
                rotation = (starting_pos + num) / 100;
                starting_pos = (starting_pos + num) % 100 
            }
        },
        _ => {}
        }

        zero_counter += rotation;
        if rotation == 0 && starting_pos == 0 {
            zero_counter += 1;
        }
        if starting_pos == 0 {
        }
    }

    return Some(zero_counter.try_into().unwrap());
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
        assert_eq!(result, Some(6));
    }
}
