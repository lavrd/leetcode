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

fn binary_search_recursive(v: Vec<u128>, target: u128, start: usize, stop: usize) -> Option<usize> {
    let mut mid: usize = 0;
    if v.len() > 1 {
        mid = stop.checked_sub(start)?.checked_div(2)? + start
    }
    if v[mid] == target {
        return Some(mid);
    }
    if v[mid] < target {
        return binary_search_recursive(v, target, mid + 1, stop);
    }
    if v[mid] > target {
        return binary_search_recursive(v.to_vec(), target, start, mid.checked_sub(1)?);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{binary_search, binary_search_recursive};

    #[test]
    fn test_binary_search() -> Result<(), Box<dyn std::error::Error>> {
        let cases = gen_cases();
        for case in cases {
            let keep = case.clone();
            let res_index = binary_search(case.0, case.1);
            if res_index != case.2 {
                return Err(format!("{:?} -> {:?}", keep, res_index).to_string().into());
            }
        }
        Ok(())
    }

    #[test]
    fn test_binary_search_recursive() -> Result<(), Box<dyn std::error::Error>> {
        let cases = gen_cases();
        for case in cases {
            let keep = case.clone();
            let length = case.0.len();
            let res_index = binary_search_recursive(case.0, case.1, 0, length - 1);
            if res_index != case.2 {
                return Err(format!("{:?} -> {:?}", keep, res_index).to_string().into());
            }
        }
        Ok(())
    }

    fn gen_cases() -> Vec<(Vec<u128>, u128, Option<usize>)> {
        vec![
            (vec![3, 6, 9, 12, 15, 19, 22, 77], 6, Some(1)),
            (vec![3, 6, 9, 12, 15, 19, 22, 77], 19, Some(5)),
            (vec![3, 6, 9, 12, 15, 19, 22, 77], 77, Some(7)),
            (vec![3, 6, 9, 12, 15, 19, 22, 77], 3, Some(0)),
            (gen_v(100), 54, Some(53)),
            (gen_v(256), 54, Some(53)),
            (gen_v(256), 555, None),
            (gen_v(3), 0, None),
            (gen_v(256), 0, None),
            (gen_v(256), 257, None),
            (gen_v(256), 255, Some(254)),
        ]
    }

    fn gen_v(size: usize) -> Vec<u128> {
        let mut v: Vec<u128> = Vec::with_capacity(size);
        for i in 0..size {
            v.push(i as u128 + 1)
        }
        v
    }
}
