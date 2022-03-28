fn quicksort(mut v: Vec<u128>, asc: bool) -> Vec<u128> {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((0, v.len() - 1));

    while let Some(ss) = stack.pop() {
        let pivot_comparison_index: usize = partition(&mut v, ss.0, ss.1, asc);

        if pivot_comparison_index != 0 && pivot_comparison_index - 1 > ss.0 {
            stack.push((ss.0, pivot_comparison_index - 1))
        }
        if pivot_comparison_index + 1 < ss.1 {
            stack.push((pivot_comparison_index + 1, ss.1))
        }
    }

    v
}

// TODO: This is Lomuto partitioning. Add Hoare and pass partitioning like a argument.
fn partition(v: &mut Vec<u128>, start: usize, stop: usize, asc: bool) -> usize {
    let pivot = v[stop];
    // Numbers that are smaller than pivot go before (left) of this index or vice versa if decent.
    let mut pivot_comparison_index = start;

    for i in start..stop {
        if v[i] <= pivot && asc {
            swap_force(v, i, pivot_comparison_index);
            pivot_comparison_index += 1
        }
        if v[i] >= pivot && !asc {
            swap_force(v, i, pivot_comparison_index);
            pivot_comparison_index += 1
        }
    }

    swap_force(v, pivot_comparison_index, stop);

    pivot_comparison_index
}

fn quicksort_recursive(mut v: Vec<u128>, asc: bool) -> Vec<u128> {
    if v.len() <= 1 {
        return v;
    }

    if v.len() == 2 {
        swap(&mut v, 0, 1, asc);
        return v;
    }

    let pivot_index = (v.len() - 1) / 2;
    let pivot = v[pivot_index];
    v.swap_remove(pivot_index);
    let mut smaller: Vec<u128> = Vec::new();
    let mut larger: Vec<u128> = Vec::new();
    for x in v {
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

fn swap_force(v: &mut Vec<u128>, i1: usize, i2: usize) {
    let tmp = v[i1];
    v[i1] = v[i2];
    v[i2] = tmp;
}

fn swap(v: &mut Vec<u128>, i1: usize, i2: usize, asc: bool) {
    println!("{:?}", (v.clone(), i1, i2));
    if v[i1] > v[i2] && asc {
        let tmp = v[i1];
        v[i1] = v[i2];
        v[i2] = tmp;
    }
    if v[i1] < v[i2] && !asc {
        let tmp = v[i1];
        v[i1] = v[i2];
        v[i2] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use crate::{quicksort, quicksort_recursive};

    #[test]
    fn test_quicksort() -> Result<(), Box<dyn std::error::Error>> {
        let cases = gen_cases();
        for case in cases {
            let keep = case.clone();
            let res = quicksort(case.0, case.2);
            if res != case.1 {
                return Err(format!("{:?} -> {:?}", keep, res).to_string().into());
            }
        }
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
