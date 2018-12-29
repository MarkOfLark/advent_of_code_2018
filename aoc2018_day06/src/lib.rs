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
                
        if dist <= nearest_measurement {
            nearest_measurement = dist;
            nearest_coord_idx = c.idx;

            if dist < nearest_measurement {
                other_nearest_coord_idx = c.idx;
            }
        }
    }

    if other_nearest_coord_idx == nearest_coord_idx {
        Some(nearest_coord_idx)
    }
    else {
        None
    }
}




#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {

    let coords = parse_coordinates(input);
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();

    let mut area = vec![0u32; coords.len()];
    let is_inf = vec![false; coords.len()];

    // Count the areas
    for x in 0..=max_x {
        for y in 0..=max_y {
            if let Some(idx) = find_nearest_coord(x,y,&coords) {
                area[idx] += 1;
            }
        }
    }
    println!("{:?}",area);

    // Loop over the edges again

    "not implemented".to_string()
}
