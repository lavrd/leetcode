fn find(height: u128, width: u128) -> u128 {
    // We found
    if height == width * 2 {
        return width;
    }

    // First number in tuple is bigger than second.
    let measurements: (u128, u128) = if height > width {
        (height, width)
    } else {
        (width, height)
    };

    // Find left space.
    let left_space = measurements.0 % measurements.1;

    if left_space == 0 {
        return width;
    }

    // Try to find smaller plots in left space.
    find(measurements.1, left_space)
}

#[cfg(test)]
mod tests {
    use crate::find;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(u128, u128, u128)> =
            vec![(1680, 640, 80), (80, 80, 80), (160, 80, 80), (80, 90, 10)];
        for case in cases {
            let res = find(case.0, case.1);
            if res != case.2 {
                return Err(format!("{:?} -> {}", case, res).to_string().into());
            }
        }
        Ok(())
    }
}
