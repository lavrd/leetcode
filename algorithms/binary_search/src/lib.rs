fn binary_search(v: Vec<u128>, target: u128) -> Option<usize> {
    let mut low_index = 0;
    let mut high_index = v.len() - 1;
    while low_index <= high_index {
        let mid_index = (low_index + high_index) / 2;
        let guess = v[mid_index];
        // println!("{:?}", (low_index, high_index, mid_index, guess));
        if guess == target {
            return Some(mid_index);
        }
        if guess < target {
            low_index = mid_index + 1
        }
        if guess > target {
            match mid_index {
                0 => return None,
                _ => high_index = mid_index - 1,
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{binary_search, gen_v};

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(Vec<u128>, u128, Option<usize>)> = vec![
            (gen_v(12), 10, Some(9)),
            (gen_v(100), 54, Some(53)),
            (gen_v(256), 54, Some(53)),
            (gen_v(256), 555, None),
            (gen_v(3), 0, None),
            (gen_v(256), 0, None),
            (gen_v(256), 257, None),
            (gen_v(256), 255, Some(254)),
        ];
        for case in cases {
            let keep = case.clone();
            let res_index = binary_search(case.0, case.1);
            if res_index != case.2 {
                return Err(format!("{:?} -> {:?}", keep, res_index).to_string().into());
            }
        }
        Ok(())
    }
}

fn gen_v(size: usize) -> Vec<u128> {
    let mut v: Vec<u128> = Vec::with_capacity(size);
    for i in 0..size {
        v.push(i as u128 + 1)
    }
    v
}
