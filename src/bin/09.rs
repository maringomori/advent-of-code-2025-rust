advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut points: Vec<(i64, i64)>= Vec::new();

    for rows in input.lines() {
        let nums: Vec<&str> = rows.split(",").collect();
        let x = nums[0].parse::<i64>().unwrap();
        let y = nums[1].parse::<i64>().unwrap();

        points.push((x, y));
    }
    
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let current_max: u64 = ((points[i].0 - points[j].0).abs() + 1) as u64 * ((points[i].1 - points[j].1).abs() + 1) as u64;

            if current_max > max_area {
                max_area = current_max;
            }
        }
    }
    
    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points: Vec<(i64, i64)>= Vec::new();

    for rows in input.lines() {
        let nums: Vec<&str> = rows.split(",").collect();
        let x = nums[0].parse::<i64>().unwrap();
        let y = nums[1].parse::<i64>().unwrap();

        points.push((x, y));
    }
    
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let current_max: u64 = ((points[i].0 - points[j].0).abs() + 1) as u64 * ((points[i].1 - points[j].1).abs() + 1) as u64;

            if current_max > max_area {
                let mut fits = true;

                for k in 0..points.len() {
                    let next_i = (k + 1) % points.len();

                    let (x1, y1) = points[i];
                    let (x2, y2) = points[j];

                    let max_y = y1.max(y2);
                    let min_y = y1.min(y2);
                    let max_x = x1.max(x2);
                    let min_x = x1.min(x2);

                    let (nx1, ny1) = points[k];
                    let (nx2, ny2) = points[next_i];

                    let ok =
                        ((ny1 >= max_y) && (ny2 >= max_y)) || // above
                        ((ny1 <= min_y) && (ny2 <= min_y)) || // under
                        ((nx1 >= max_x) && (nx2 >= max_x)) || // right
                        ((nx1 <= min_x) && (nx2 <= min_x));   // left

                    if !ok {
                        fits = false;
                        break;
                    }
                }

                if fits {
                    max_area = current_max;
                }
            }
        }
    }
    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
