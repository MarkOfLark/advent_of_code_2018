use std::collections::HashMap;

#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {

    let mut count2 = 0i64;
    let mut count3 = 0i64;

    let lines: Vec<&str> = input.lines().collect();

    if 1 == star {
        for line in lines.iter() {
            let mut hist = HashMap::new();
            for c in line.chars() {

                let current_count = match hist.get(&c) {
                    Some(count) => *count,
                    None => 0
                };
                hist.insert(c,current_count+1);
            }

            let mut found2 = 0i64;
            let mut found3 = 0i64;
            for count in hist.values() {
                if 2 == *count {
                    found2 = 1;
                }
                if 3 == *count {
                    found3 = 1;
                }
            }

            count2 += found2;
            count3 += found3;
        }   

        (count3*count2).to_string()
    }
    else {
        for src_line in 0..lines.len() {

            for (id1,id2) in lines[src_line..lines.len()].iter().zip(lines.iter()) {

                let mut diff_count = 0i32;
                let mut last_diff_pos = 0usize;
                for (i,(c1,c2)) in id1.chars().zip(id2.chars()).enumerate() {
                    if c1 != c2 {
                        last_diff_pos = i;
                        diff_count += 1;
                    }
                }

                if 1 == diff_count {
                    return id1[0..last_diff_pos].to_owned() + &id1[last_diff_pos+1..id1.len()];
                }
            }
        }

        "No star 2 solution".to_string()
    }
}
