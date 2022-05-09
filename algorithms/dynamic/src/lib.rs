use std::{cell::RefCell, cmp::max, collections::HashMap};

type Weight = i128;
type Price = i128;

#[derive(Clone)]
struct Item {
    weight: Weight,
    price: Price,
}

#[derive(Clone)]
struct Cell {
    price: Price,
    items: Vec<String>,
}

fn run_knapsack_problem(
    knapsack: HashMap<String, HashMap<Weight, RefCell<Cell>>>,
    items: HashMap<String, Item>,
) -> (Vec<String>, Price) {
    if knapsack.is_empty() {
        return (Vec::new(), 0);
    }

    // Store last row to access later to not find higher price in every row/cell. Last row last cell contains higher price.
    // Also, store to access previous row if current item cannot fit into cell.
    let mut last_item_name: String = "".to_string();

    for (item_name, cells) in knapsack.iter() {
        let item = items.get(item_name).unwrap();
        // Iterate through row cells.
        for (cur_weight, cur_cell) in cells.iter() {
            // Get cells from previous iteration.
            let prev_cells = knapsack.get(&last_item_name);
            // Get max price from previous iteration.
            let prev_cell: Cell = if let Some(prev_cells) = prev_cells {
                prev_cells.get(cur_weight).unwrap().borrow().clone()
            } else {
                Cell {
                    price: 0,
                    items: Vec::new(),
                }
            };
            let space_left = cur_weight - &item.weight;

            // If cell fit whole item and there is no free space after item fit and current item has higher price then previous.
            if cur_weight >= &item.weight && space_left == 0 && item.price > prev_cell.price {
                *cur_cell.borrow_mut() = Cell {
                    price: item.price,
                    items: vec![item_name.clone()],
                };
                continue;
            }
            // // If cell fit whole item and there is space for another item.
            if cur_weight >= &item.weight && space_left > 0 {
                if let Some(prev_cells) = prev_cells {
                    let free_space_cell = prev_cells.get(&space_left).unwrap();
                    if item.price + free_space_cell.borrow().price > prev_cell.price {
                        let mut items: Vec<String> = vec![item_name.clone()];
                        items.extend(free_space_cell.borrow().items.clone());
                        *cur_cell.borrow_mut() = Cell {
                            price: item.price + free_space_cell.borrow().price,
                            items,
                        };
                        continue;
                    }
                }
            }
            *cur_cell.borrow_mut() = prev_cell
        }
        last_item_name = item_name.clone();
    }

    let mut items: Vec<String> = Vec::new();
    let mut price: Price = 0;
    for item in knapsack.get(&last_item_name).unwrap() {
        if price < item.1.borrow().price {
            price = item.1.borrow().price;
            items = item.1.borrow().items.clone()
        }
    }
    (items, price)
}

fn longest_common_substring(target: String, variants: Vec<String>) -> Vec<(String, i128)> {
    let mut calc_variants: Vec<(String, i128)> = Vec::new();
    let length: usize = target.len();

    for variant in variants {
        let mut cells: Vec<Vec<i128>> = Vec::with_capacity(variant.len());

        // Initialize variants.
        for i in 0..cells.capacity() {
            cells.push(Vec::with_capacity(length));
            for _ in 0..cells[i].capacity() {
                cells[i].push(0)
            }
        }

        let mut i: usize = 0;
        loop {
            let mut j: usize = 0;

            loop {
                let char_left = target.chars().nth(i).unwrap();
                let char_right = variant.chars().nth(j).unwrap();

                if char_left == char_right {
                    let prev = if i.checked_sub(1).is_none() || j.checked_sub(1).is_none() {
                        0
                    } else {
                        cells[i - 1][j - 1]
                    };
                    cells[i][j] = prev + 1
                } else {
                    let left = if i.checked_sub(1).is_none() {
                        0
                    } else {
                        cells[i - 1][j]
                    };
                    let right = if j.checked_sub(1).is_none() {
                        0
                    } else {
                        cells[i][j - 1]
                    };
                    cells[i][j] = max(left, right)
                }

                j += 1;
                if j == length {
                    break;
                }
            }

            i += 1;
            if i == length {
                break;
            }
        }

        calc_variants.push((variant, cells[length - 1][length - 1]));
    }

    calc_variants
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashMap};

    use crate::{longest_common_substring, run_knapsack_problem, Cell, Item, Weight};

    #[test]
    fn test_lcs() {
        let variants = longest_common_substring(
            "fog".to_string(),
            vec!["fish".to_string(), "fort".to_string()],
        );
        println!("{:?}", variants);
    }

    #[test]
    fn test_knapsack() {
        let mut knapsack: HashMap<String, HashMap<Weight, RefCell<Cell>>> = HashMap::from([
            (
                "guitar".to_string(),
                HashMap::from([
                    (1, gen_cell()),
                    (2, gen_cell()),
                    (3, gen_cell()),
                    (4, gen_cell()),
                ]),
            ),
            (
                "stereo".to_string(),
                HashMap::from([
                    (1, gen_cell()),
                    (2, gen_cell()),
                    (3, gen_cell()),
                    (4, gen_cell()),
                ]),
            ),
            (
                "laptop".to_string(),
                HashMap::from([
                    (1, gen_cell()),
                    (2, gen_cell()),
                    (3, gen_cell()),
                    (4, gen_cell()),
                ]),
            ),
        ]);
        let mut items: HashMap<String, Item> = HashMap::from([
            (
                "guitar".to_string(),
                Item {
                    weight: 1,
                    price: 1500,
                },
            ),
            (
                "stereo".to_string(),
                Item {
                    weight: 5,
                    price: 3000,
                },
            ),
            (
                "laptop".to_string(),
                Item {
                    weight: 3,
                    price: 2000,
                },
            ),
        ]);
        for _ in 0..10_000 {
            let res = run_knapsack_problem(knapsack.clone(), items.clone());
            assert_eq!(res.0.len(), 2);
            assert_eq!(res.0.contains(&"guitar".to_string()), true);
            assert_eq!(res.0.contains(&"laptop".to_string()), true);
            assert_eq!(res.1, 3500);
        }
        knapsack.insert(
            "iPhone".to_string(),
            HashMap::from([
                (1, gen_cell()),
                (2, gen_cell()),
                (3, gen_cell()),
                (4, gen_cell()),
            ]),
        );
        items.insert(
            "iPhone".to_string(),
            Item {
                weight: 1,
                price: 2000,
            },
        );
        for _ in 0..10_000 {
            let res = run_knapsack_problem(knapsack.clone(), items.clone());
            assert_eq!(res.0.len(), 2);
            assert_eq!(res.0.contains(&"iPhone".to_string()), true);
            assert_eq!(res.0.contains(&"laptop".to_string()), true);
            assert_eq!(res.1, 4000)
        }
    }

    fn gen_cell() -> RefCell<Cell> {
        RefCell::new(Cell {
            price: 0,
            items: Vec::new(),
        })
    }
}
