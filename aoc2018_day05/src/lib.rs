use std::iter::FromIterator;

fn react_polymer(poly: &mut Vec<char>, start_idx: usize) -> (bool,usize)
{
    let mut remove_idx = 0usize;
    for c in start_idx+1..poly.len() {
        let PC = poly[c-1].to_ascii_uppercase();
        let C = poly[c].to_ascii_uppercase();
        if PC == C {
            if poly[c-1] != poly[c] {
                remove_idx = c;
                break;
            }
        }
    }

    if 0 == remove_idx {
        return (false,0usize)
    }
    else {
        poly.remove(remove_idx-1);
        poly.remove(remove_idx-1);
    
        if 1 == remove_idx {
            return (true,remove_idx-1);
        }
        else {
            return (true,remove_idx-2);
        }
    }
}

fn fully_react(polymer: &str) -> String
{
    let mut fully_reacted = false;
    let mut poly = polymer.chars().collect();
    let mut start_idx = 0usize;
    while !fully_reacted {
        let (reacted, new_start_idx) = react_polymer(&mut poly,start_idx);
        
        if reacted {
            start_idx = new_start_idx;
        }
        else {
            fully_reacted = true;
        }
    }

    String::from_iter(poly)
}

#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {

    if 1 == star {
        let s = fully_react(input);
        return format!("{}",s.len());
    }
    else {
        let mut min = input.len();
        for c in ('a' as u8)..=('z' as u8) {
            let ss: String = input
                .chars()
                .filter(|x| *x != (c as char) && *x != (c as char).to_ascii_uppercase())
                .collect();

            let s = fully_react(&ss);

            if s.len() < min {
                min = s.len();
            }
        }
        return format!("{}",min);
    }

    "not implemented".to_string()
}
