advent_of_code::solution!(8);

fn dist(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    let dz = a.2 as f64 - b.2 as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut circuits: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut boxes: Vec<(u64, u64, u64)>= Vec::new();

    for rows in input.lines() {
        let nums: Vec<&str> = rows.split(",").collect();
        let x = nums[0].parse::<u64>().unwrap();
        let y = nums[1].parse::<u64>().unwrap();
        let z = nums[2].parse::<u64>().unwrap();

        boxes.push((x, y, z));
    }
    
    let mut distances: Vec<((u64,u64,u64), (u64,u64,u64), f64)> = Vec::new();
    for i in 0..boxes.len() {
        for j in i+1..boxes.len() {
            let d = dist(boxes[i], boxes[j]);
            distances.push((boxes[i], boxes[j], d));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for i in 0..1000 { 
        let (p1, p2, _) = distances[i];
        let circuit_i = circuits.iter().position(|c| c.contains(&p1)); 
        let circuit_j = circuits.iter().position(|c| c.contains(&p2)); 
        
        match (circuit_i, circuit_j) { 
            (Some(ci), Some(cj)) if ci == cj => { 
            } 
            (Some(ci), Some(cj)) => {
                let mut merged = circuits[ci].clone();
                merged.extend(circuits[cj].iter().cloned());
                
                circuits[ci] = merged;
                circuits.remove(cj);
            }
            (Some(ci), None) => { 
                circuits[ci].push(p2); 
            }
            (None, Some(cj)) => { 
                circuits[cj].push(p1);
            } (None, None) => {
                circuits.push(vec![p1, p2]); 
            } 
        }
    }

    let mut sizes: Vec<u64> = circuits.iter().map(|c| c.len() as u64).collect();
    sizes.sort_by(|a, b| b.cmp(a)); 
    let total: u64 = sizes.iter().take(3).product();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut circuits: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut boxes: Vec<(u64, u64, u64)>= Vec::new();

    for rows in input.lines() {
        let nums: Vec<&str> = rows.split(",").collect();
        let x = nums[0].parse::<u64>().unwrap();
        let y = nums[1].parse::<u64>().unwrap();
        let z = nums[2].parse::<u64>().unwrap();

        boxes.push((x, y, z));
    }
    
    let mut distances: Vec<((u64,u64,u64), (u64,u64,u64), f64)> = Vec::new();
    for i in 0..boxes.len() {
        for j in i+1..boxes.len() {
            let d = dist(boxes[i], boxes[j]);
            distances.push((boxes[i], boxes[j], d));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());



    let p1_x;
    let p2_x;

    for b in &boxes {
        circuits.push(vec![*b]);
    }

    let mut i = 0;
    loop {

        let (p1, p2, _) = distances[i];
        let circuit_i = circuits.iter().position(|c| c.contains(&p1)); 
        let circuit_j = circuits.iter().position(|c| c.contains(&p2)); 
        
        match (circuit_i, circuit_j) { 
            (Some(ci), Some(cj)) if ci == cj => { 
            } 
            (Some(ci), Some(cj)) => {
                let mut merged = circuits[ci].clone();
                merged.extend(circuits[cj].iter().cloned());
                
                circuits[ci] = merged;
                circuits.remove(cj);
            }
            (Some(ci), None) => { 
                circuits[ci].push(p2); 
            }
            (None, Some(cj)) => { 
                circuits[cj].push(p1);
            } (None, None) => {
                circuits.push(vec![p1, p2]); 
            } 
        }

        if circuits.len() == 1 {
            p1_x = p1.0;
            p2_x = p2.0;
            break;
        }

        i+=1;
    }

    Some(p1_x*p2_x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
