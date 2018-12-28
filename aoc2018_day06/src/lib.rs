#[derive(Debug)]
struct Coordinate {
    idx: usize,
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance_to(&self, other: &Coordinate) -> u32 {
        self.manhattan_distance_to(other.x,other.y)
    }
    fn manhattan_distance_to(&self, other_x: i32, other_y: i32) -> u32 {
        ((self.x - other_x).abs() + (self.y - other_y).abs()) as u32
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

#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {

    let coords = parse_coordinates(input);
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();

    let area = vec![0u32; coords.len()];
    let is_inf = vec![false; coords.len()];

    for x in 0..=max_x {
        for y in 0..=max_y {

        }
    }

    "not implemented".to_string()
}
