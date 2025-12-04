advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let dirs: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0), (1, 1),
    ];
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' {
                let mut count = 0;
                for (dy, dx) in dirs {

                    let ny = dy + y as i32;
                    let nx = dx + x as i32;
                    
                    if ny >= 0 && ny < height as i32 &&
                       nx >= 0 && nx < width as i32 &&
                       grid[ny as usize][nx as usize] == '@'
                    {
                        count += 1;
                    }
                }
                if count < 4 {
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

pub fn pos_to_be_removed(grid: &Vec<Vec<char>>) -> Vec<(i32, i32)>  {
    let height = grid.len();
    let width = grid[0].len();

    let dirs: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0), (1, 1),
    ];
    let mut pos_to_remove = Vec::<(i32, i32)>::new();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' {
                let mut count = 0;
                for (dy, dx) in dirs {

                    let ny = dy + y as i32;
                    let nx = dx + x as i32;
                    
                    if ny >= 0 && ny < height as i32 &&
                       nx >= 0 && nx < width as i32 &&
                       grid[ny as usize][nx as usize] == '@'
                    {
                        count += 1;

                    }
                }
                if count < 4 {
                    pos_to_remove.push((y.try_into().unwrap(), x.try_into().unwrap()));
                }
            }
        }
    }
    
    pos_to_remove
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;

    loop {
        let positions = pos_to_be_removed(&grid);
       
        if positions.is_empty() {
            break;
        }

        sum += positions.len() as u64;

        for (y, x) in positions {
            grid[y as usize][x as usize] = '.';
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
