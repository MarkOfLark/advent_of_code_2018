#[no_mangle]
pub fn aoc_solution(input: &str) -> String {
    let mut sum = 0i64;

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
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
