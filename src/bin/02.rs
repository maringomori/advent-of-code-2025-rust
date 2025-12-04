advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let records = input.split(",");
    
    let mut sum: u64 = 0;

    for ids in records {
        let ids: Vec::<&str> = ids.split("-").collect();
        let id1: u64 = ids[0].parse().unwrap();
        let id2: u64 = ids[1].parse().unwrap();

        for i in id1..id2+1 {
            let s = i.to_string();
            let chars: Vec<char> = s.chars().collect();

            let mid = chars.len()/2;

            if chars.len() % 2 != 0 {
                continue;
            }

            let part1: String = chars[..mid].iter().collect();
            let part2: String = chars[mid..].iter().collect();
            
            if part1 == part2 {
                sum += i;
            }
        }   
    }

    return Some(sum.try_into().unwrap());
}

pub fn part_two(input: &str) -> Option<u64> {
    let records = input.split(",");
    
    let mut sum: u64 = 0;

    for ids in records {
        let ids: Vec::<&str> = ids.split("-").collect();
        let id1: u64 = ids[0].parse().unwrap();
        let id2: u64 = ids[1].parse().unwrap();

        for i in id1..id2+1 {
            let s = i.to_string();
            
            if is_repetition(&s) {
                sum += i
            }
        }
    }

    return Some(sum.try_into().unwrap());
}

fn is_repetition(s: &str) -> bool {
    let len = s.len();

    for sub_len in 1..len/2+1 {
        if len % sub_len != 0 {
            continue;
        }

        let mut valid = true;
        let sub = &s[0..sub_len];

        for chunk_start in (0..len).step_by(sub_len) {
            if &s[chunk_start..chunk_start + sub_len] != sub {
                valid = false;
                break;
            }
        }

        if valid {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
