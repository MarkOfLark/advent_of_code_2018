#[macro_use]
extern crate nom;

use nom::digit;
use nom::types::CompleteStr;
use std::str::FromStr;

#[derive (Eq, PartialEq, Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

//named!(claimi(CompleteStr) -> i32, map_res!(recognize!(nom::digit), i32::from_str));

named!(parse_claim <CompleteStr,Claim>, do_parse!(
    ws!(tag!("#")) >>
    id: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    ws!(tag!("@")) >>
    x: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    ws!(tag!(",")) >>
    y: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    ws!(tag!(":")) >>
    w: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    ws!(tag!("x")) >>
    h: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    (Claim {id,x,y,w,h})
));



#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    let mut claim: Vec<Claim> = Vec::new();

    for line in input.lines() {

        let c = parse_claim(CompleteStr(line)).unwrap().1;

        if c.x + c.w - 1 > max_x {
            max_x = c.x + c.w - 1;
        }
        if c.y + c.h - 1 > max_y {
            max_y = c.y + c.h - 1;
        }

        claim.push(c);
    }

    let width = (max_x+1) as usize;
    let height = (max_y+1) as usize;
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let fabric: &mut [&mut [_]] = grid_base.as_mut_slice();

    for c in claim.iter() {
        for y in c.y..=(c.y+c.h-1) {
            for x in c.x..=(c.x+c.w-1) {
                //println!("{:?}",c);
                fabric[y as usize][x as usize] += 1;
            }
        }
    }

    let mut star1_count = 0u64;
    for y in 0..height {
        for x in 0..width {
            if fabric[y as usize][x as usize] >= 2 {
                star1_count += 1;
            }
        }
    }

    let mut good_claim_id = 0u32;
    for c in claim.iter() {
        let mut claim_pure = true;
        for y in c.y..=(c.y+c.h-1) {
            for x in c.x..=(c.x+c.w-1) {
                if fabric[y as usize][x as usize] > 1 {
                    claim_pure = false;
                }
            }
        }

        if claim_pure {
            good_claim_id = c.id;
            break;
        }
    }


    if 1 == star {
        format!("{}",star1_count)
    }
    else {
        format!("{}",good_claim_id)
    }
}
