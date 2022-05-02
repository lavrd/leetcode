use std::collections::{HashMap, HashSet};

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

fn set_covering_problem<'a>(
    mut states_needed: HashSet<&'a str>,
    stations: HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let mut final_stations: HashSet<&str> = HashSet::new();
    while !states_needed.is_empty() {
        let mut best_station = stations.iter().next().unwrap();
        let mut best_station_covered: HashSet<&str> =
            best_station.1.intersection(&states_needed).cloned().collect();
        for station in &stations {
            let station_covered: HashSet<&str> =
                station.1.intersection(&states_needed).cloned().collect();
            if station_covered.len() > best_station_covered.len() {
                best_station = station.clone();
                best_station_covered = station_covered;
            }
        }
        final_stations.insert(best_station.0.clone());
        states_needed = states_needed.difference(&best_station_covered).cloned().collect();
    }
    final_stations
}

fn salesperson_problem(
    points: HashMap<String, HashMap<String, u8>>,
    start: String,
) -> (u8, Vec<String>) {
    let mut max_variants: u8 = u8::MAX;
    let mut variants: HashMap<u8, Vec<String>> = HashMap::with_capacity(max_variants as usize);
    // TODO: Add a comment.
    let mut visited: HashMap<String, HashSet<String>> = HashMap::with_capacity(points.len());

    while max_variants != 0 {
        let mut cur_distance: u8 = 0;
        let mut cur_path: Vec<String> = Vec::with_capacity(points.len());
        let mut cur_visited: HashSet<String> = HashSet::with_capacity(points.len());

        let mut cur_point_name = start.clone();
        let mut cur_point = points.get(&cur_point_name).unwrap();
        cur_path.push(start.clone());

        while cur_visited.len() != points.len() {
            let _empty_str: &String = &"".to_string();
            let closest_point: (&String, &u8) =
                cur_point.iter().fold((_empty_str, &u8::MAX), |closest, cur| {
                    if cur.1 < &closest.1 {
                        if !cur_visited.contains(cur.0) {
                            if visited.get(&cur_point_name).is_some() {
                                if visited.get(&cur_point_name).unwrap().contains(cur.0) {
                                    return closest;
                                }
                            }
                            return cur.clone();
                        }
                    }
                    closest
                });
            if closest_point.0.is_empty() {
                break;
            }

            if visited.get_mut(&cur_point_name).is_none() {
                visited.insert(cur_point_name.clone(), HashSet::from([closest_point.0.clone()]));
            } else {
                visited.get_mut(&cur_point_name).unwrap().insert(closest_point.0.clone());
            }

            cur_distance += closest_point.1;
            cur_point_name = closest_point.0.clone();
            cur_path.push(cur_point_name.clone());
            cur_visited.insert(cur_point_name.clone());
            cur_point = points.get(&cur_point_name).unwrap();
        }

        variants.insert(cur_distance, cur_path);
        max_variants -= 1;
    }

    let mut path: (u8, Vec<String>) = (u8::MAX, Vec::new());
    for v in &variants {
        if v.1.len() >= points.len() && v.0 < &path.0 {
            path = (*v.0, v.1.clone())
        }
    }
    path
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        vec,
    };

    use crate::{salesperson_problem, schedule_problem, set_covering_problem, Class};

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

    #[test]
    fn test_covering_problem() {
        let states_needed: HashSet<&str> =
            HashSet::from(["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]);
        let stations: HashMap<&str, HashSet<&str>> = HashMap::from([
            ("kone", HashSet::from(["id", "nv", "ut"])),
            ("ktwo", HashSet::from(["wa", "id", "mt"])),
            ("kthree", HashSet::from(["or", "nv", "ca"])),
            ("kfour", HashSet::from(["nv", "ut"])),
            ("kfive", HashSet::from(["ca", "az"])),
        ]);
        let final_stations = set_covering_problem(states_needed, stations);
        let diff: HashSet<_> = final_stations
            .difference(&HashSet::from(["ktwo", "kthree", "kone", "kfive"]))
            .cloned()
            .collect();
        assert_eq!(diff, HashSet::new());
    }

    #[test]
    fn test_salesperson_problem() {
        let points: HashMap<String, HashMap<String, u8>> = HashMap::from([
            (
                "A".to_string(),
                HashMap::from([
                    ("A".to_string(), 0),
                    ("B".to_string(), 1),
                    ("C".to_string(), 2),
                    ("D".to_string(), 5),
                    ("E".to_string(), 3),
                ]),
            ),
            (
                "B".to_string(),
                HashMap::from([
                    ("A".to_string(), 6),
                    ("B".to_string(), 0),
                    ("C".to_string(), 8),
                    ("D".to_string(), 3),
                    ("E".to_string(), 9),
                ]),
            ),
            (
                "C".to_string(),
                HashMap::from([
                    ("A".to_string(), 3),
                    ("B".to_string(), 1),
                    ("C".to_string(), 0),
                    ("D".to_string(), 1),
                    ("E".to_string(), 1),
                ]),
            ),
            (
                "D".to_string(),
                HashMap::from([
                    ("A".to_string(), 4),
                    ("B".to_string(), 9),
                    ("C".to_string(), 4),
                    ("D".to_string(), 0),
                    ("E".to_string(), 4),
                ]),
            ),
            (
                "E".to_string(),
                HashMap::from([
                    ("A".to_string(), 5),
                    ("B".to_string(), 4),
                    ("C".to_string(), 8),
                    ("D".to_string(), 6),
                    ("E".to_string(), 4),
                ]),
            ),
        ]);
        let path = salesperson_problem(points, "A".to_string());
        println!("{:?}", path)
    }
}
