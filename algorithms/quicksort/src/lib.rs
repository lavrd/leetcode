fn quicksort_recursive(mut v: Vec<u128>, asc: bool) -> Vec<u128> {
    if v.len() <= 1 {
        return v;
    }

    if v.len() == 2 {
        return swap(v, asc);
    }

    let pivot_index = (v.len() - 1) / 2;
    let pivot = v[pivot_index];
    v.swap_remove(pivot_index);
    let mut smaller: Vec<u128> = Vec::new();
    let mut larger: Vec<u128> = Vec::new();
    for x in v.clone() {
        choose_sub_array(x, pivot, &mut smaller, &mut larger, asc)
    }

    let sorted_smaller = quicksort_recursive(smaller, asc);
    let sorted_larger = quicksort_recursive(larger, asc);

    let mut result: Vec<u128> = Vec::new();
    result.extend(sorted_smaller);
    result.push(pivot);
    result.extend(sorted_larger);

    result
}

fn choose_sub_array(
    x: u128,
    pivot: u128,
    smaller: &mut Vec<u128>,
    larger: &mut Vec<u128>,
    asc: bool,
) {
    if asc {
        if x > pivot {
            larger.push(x);
        } else {
            smaller.push(x);
        }
    } else {
        if x > pivot {
            smaller.push(x);
        } else {
            larger.push(x);
        }
    }
}

fn swap(mut v: Vec<u128>, asc: bool) -> Vec<u128> {
    if v[0] > v[1] && asc {
        let tmp = v[0];
        v[0] = v[1];
        v[1] = tmp;
        return v;
    }
    if v[0] < v[1] && !asc {
        let tmp = v[0];
        v[0] = v[1];
        v[1] = tmp;
        return v;
    }
    v
}

#[cfg(test)]
mod tests {
    use crate::quicksort_recursive;

    #[test]
    fn test_quicksort() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[test]
    fn test_quicksort_recursive() -> Result<(), Box<dyn std::error::Error>> {
        let cases = gen_cases();
        for case in cases {
            let keep = case.clone();
            let res = quicksort_recursive(case.0, case.2);
            if res != case.1 {
                return Err(format!("{:?} -> {:?}", keep, res).to_string().into());
            }
        }
        Ok(())
    }

    fn gen_cases() -> Vec<(Vec<u128>, Vec<u128>, bool)> {
        vec![
            (
                vec![55, 8, 12, 34, 5, 7, 122, 34, 0],
                vec![0, 5, 7, 8, 12, 34, 34, 55, 122],
                true,
            ),
            (
                vec![55, 8, 12, 5, 7, 122, 34, 0],
                vec![122, 55, 34, 12, 8, 7, 5, 0],
                false,
            ),
        ]
    }
}
