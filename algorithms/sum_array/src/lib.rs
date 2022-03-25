type Type = i128;

// Recursive and D&C.
fn sum_recursive_dc(v: Vec<Type>) -> Type {
    if v.len() == 1 {
        return v[0];
    }
    sum_recursive_dc(v[..1].to_vec()) + sum_recursive_dc(v[1..].to_vec())
}

fn sum(v: Vec<Type>) -> Type {
    let mut count: Type = 0;
    for el in v {
        count += el;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{sum, sum_recursive_dc, Type};

    #[test]
    fn test_sum_recursive_dc() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(Vec<Type>, Type)> = vec![(vec![0, -1, 2, 3], 4)];
        for case in cases {
            let sum = sum_recursive_dc(case.0.clone());
            if sum != case.1 {
                return Err(format!("{:?} -> {:?}", case, sum).to_string().into());
            }
        }
        Ok(())
    }

    #[test]
    fn test_sum() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(Vec<Type>, Type)> = vec![(vec![0, -1, 2, 3], 4)];
        for case in cases {
            let sum = sum(case.0.clone());
            if sum != case.1 {
                return Err(format!("{:?} -> {:?}", case, sum).to_string().into());
            }
        }
        Ok(())
    }
}
