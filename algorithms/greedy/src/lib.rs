use chrono::TimeZone;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Class {
    name: String,
    start: chrono::DateTime<chrono::Utc>,
    stop: chrono::DateTime<chrono::Utc>,
}

impl Class {
    fn new(name: &str, start: &str, stop: &str) -> Self {
        Self {
            name: name.to_string(),
            start: chrono::Utc
                .datetime_from_str(format!("2014-11-28 {}:00", start).as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            stop: chrono::Utc
                .datetime_from_str(format!("2014-11-28 {}:00", stop).as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
    }
}

fn schedule_problem(classes: Vec<Class>) -> Vec<Class> {
    let mut can_be_visited: Vec<Class> = Vec::new();
    let mut next_classes: Vec<Class> = classes.clone();
    loop {
        let ends_soonest = next_classes.iter().fold(next_classes[0].clone(), |ends_soonest, c| {
            if ends_soonest.stop < c.stop {
                ends_soonest
            } else {
                c.clone()
            }
        });
        can_be_visited.push(ends_soonest.clone());
        next_classes = classes.iter().filter(|c| c.start >= ends_soonest.stop).cloned().collect();
        if next_classes.is_empty() {
            break;
        }
    }
    can_be_visited
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{schedule_problem, Class};

    #[test]
    fn test_schedule_problem() {
        let art = Class::new("Art", "09:00", "10:00");
        let eng = Class::new("Eng", "09:30", "10:30");
        let math = Class::new("Math", "10:00", "11:00");
        let cs = Class::new("CS", "10:30", "11:30");
        let music = Class::new("Music", "11:00", "12:00");
        let classes: Vec<Class> = Vec::from([art.clone(), eng, math.clone(), cs, music.clone()]);
        let can_be_visited = schedule_problem(classes);
        assert_eq!(can_be_visited, vec![art, math, music]);
    }
}
