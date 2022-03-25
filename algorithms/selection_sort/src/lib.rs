type Type = u128;

fn selection_sort(mut v: Vec<Type>, asc: bool) -> Vec<Type> {
    if v.len() == 0 || v.len() == 1 {
        return v;
    }
    for i in 0..v.len() {
        let extremum_index = find_extremum(v[i..].to_vec(), asc);
        let tmp_val = v[i];
        v[i] = v[extremum_index + i];
        v[extremum_index + i] = tmp_val;
    }
    v
}

fn find_extremum(v: Vec<Type>, asc: bool) -> usize {
    let mut tmp_extremum: Type = v[0];
    let mut tmp_index: usize = 0;
    for (i, el) in v.iter().enumerate() {
        if asc && *el < tmp_extremum {
            tmp_extremum = *el;
            tmp_index = i
        }
        if !asc && *el > tmp_extremum {
            tmp_extremum = *el;
            tmp_index = i
        }
    }
    tmp_index as usize
}

#[cfg(test)]
mod tests {
    use crate::{selection_sort, Type};

    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(Vec<Type>, Vec<Type>, bool)> = vec![
            (vec![], vec![], true),
            (vec![1], vec![1], false),
            (
                vec![5, 1, 6, 2, 7, 10, 2, 4, 4, 10],
                vec![1, 2, 2, 4, 4, 5, 6, 7, 10, 10],
                true,
            ),
            (
                vec![0, 11, 62, 2, 77, 2, 40],
                vec![77, 62, 40, 11, 2, 2, 0],
                false,
            ),
        ];
        for case in cases {
            let sorted = selection_sort(case.0.clone(), case.2);
            if sorted != case.1 {
                return Err(
                    format!("({:?}, {:?}, {}) -> {:?}", case.0, case.1, case.2, sorted)
                        .to_string()
                        .into(),
                );
            }
        }
        Ok(())
    }
}
