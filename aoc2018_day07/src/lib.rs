use std::collections::HashMap;
use std::iter::FromIterator;


#[derive(Debug)]
struct Coordinate {
    idx: usize,
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance_to(&self, other: &Coordinate) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

fn parse_coordinates(input: &str) -> Vec<Coordinate> {
    let mut c = Vec::new();

    for (i,l) in input.lines().enumerate() {
        let coords : Vec<i32> = l
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        c.push(Coordinate{idx: i, x:coords[0], y:coords[1]});
    }
    return c;
}



fn find_nearest_coord(x: i32, y: i32, coords: &[Coordinate]) -> Option<usize> {
    let mut nearest_measurement = std::u32::MAX;
    let mut nearest_coord_idx = 0usize;
    let mut other_nearest_coord_idx = 0usize;

    for c in coords.iter() {
        let dist = c.manhattan_distance_to(&Coordinate{idx:0usize,x:x,y:y});

        //println!("  {}: ({},{}) to ({},{}) is {}",c.idx,c.x,c.y,x,y,dist);
        if dist <= nearest_measurement {
            //println!("  {}: nearest_measurement was {}, now it is {}",c.idx,nearest_measurement,dist);
            if dist < nearest_measurement {
                other_nearest_coord_idx = c.idx;
            }

            nearest_measurement = dist;
            nearest_coord_idx = c.idx;

        }
    }

    //println!("  idx {}, other idx {}",nearest_coord_idx,other_nearest_coord_idx);

    if other_nearest_coord_idx == nearest_coord_idx {
        Some(nearest_coord_idx)
    }
    else {
        None
    }
}

fn sum_distances(x: i32, y: i32, coords: &[Coordinate]) -> u32 {
    let mut dist = 0u32;
    for c in coords.iter() {
        dist += c.manhattan_distance_to(&Coordinate{idx:0usize,x:x,y:y});
    }
    dist
}




#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {

    let coords = parse_coordinates(input);
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();

    if 1 == star {
        let mut area = HashMap::new();
        for i in 0..coords.len() {
            area.insert(i,0usize);
        }
    
        // Count the areas
        for x in 0..=max_x {
            for y in 0..=max_y {
                if let Some(idx) = find_nearest_coord(x,y,&coords) {
                    *area.get_mut(&idx).unwrap() += 1;
                }
            }
        }

        // Loop over the edges to remove any infinite regions
        for x in 0..=max_x {
            if let Some(idx) = find_nearest_coord(x,0,&coords) {
                area.remove(&idx);
            }
            if let Some(idx) = find_nearest_coord(x,max_y,&coords) {
                area.remove(&idx);
            }
        }
        for y in 0..=max_y {
            if let Some(idx) = find_nearest_coord(0,y,&coords) {
                area.remove(&idx);
            }
            if let Some(idx) = find_nearest_coord(max_x,y,&coords) {
                area.remove(&idx);
            }
        }
        format!("{}",area.values().max().unwrap())
    }
    else {
        let mut area = 0u32;

        for x in 0..=max_x {
            for y in 0..=max_y {
                if 10000 > sum_distances(x,y,&coords) {
                    area += 1;
                }
            }
        }
        area.to_string()
    }
}
