use std::collections::HashSet;

#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {
    let mut sum = 0i64;

    let mut freqs = HashSet::new();

    loop {
      for freq in input.split_whitespace() {
            if freq.starts_with('+') || freq.starts_with('-') {
                let val: i64 = match freq.ends_with(',') {
                    true => freq[1..freq.len()-1].parse().unwrap(),
                    false => freq[1..freq.len()].parse().unwrap()
                };

                match &freq[0..1] {
                    "+" => sum += val,
                    "-" => sum -= val,
                    _ => {
                        panic!(format!("Unrecognized operator {}",&freq[0..1]));
                    }
                }

                if 2 == star {
                    if freqs.contains(&sum) {
                        return sum.to_string();
                    }
                    else {
                        freqs.insert(sum);
                    }
                }
            }
        }

        if 1 == star {
            break;
        }
    }

    sum.to_string()
}
