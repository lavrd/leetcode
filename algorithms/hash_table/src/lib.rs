const SIZE: usize = 10;

type HashFn = Box<dyn Fn(&str) -> usize>;

type DataType = &'static str;

#[derive(Debug)]
struct Value {
    key: String,
    data: DataType,
}

impl Value {
    fn new(key: &str, data: DataType) -> Self {
        Self {
            key: key.to_string(),
            data,
        }
    }
}

struct HashTable {
    hash_fn: HashFn,
    table: Vec<Vec<Value>>,
}

impl HashTable {
    fn new(hash_fn: HashFn) -> Self {
        let mut table: Vec<Vec<Value>> = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            table.push(vec![]);
        }
        HashTable { hash_fn, table }
    }

    fn insert(&mut self, key: &str, data: DataType) {
        let hash: usize = (self.hash_fn)(key);
        let values = self.table.get_mut(hash).unwrap();
        values.push(Value::new(key, data))
    }

    fn get(&self, key: &str) -> Option<DataType> {
        let hash: usize = (self.hash_fn)(key);
        let values = self.table.get(hash).unwrap();
        for val in values {
            if val.key == key {
                return Some(val.data);
            }
        }
        None
    }

    fn delete(&mut self, key: &str) {
        let hash: usize = (self.hash_fn)(key);
        let values = self.table.get_mut(hash).unwrap();
        for (i, val) in values.iter().enumerate() {
            if val.key == key {
                values.remove(i);
                return;
            }
        }
    }

    fn load_factor(&self) -> f64 {
        let mut non_empty: u64 = 0;
        for values in &self.table {
            if !values.is_empty() {
                non_empty += 1;
            }
        }
        let load_factor = non_empty as f64 / SIZE as f64;
        f64::trunc(load_factor * 100f64).floor() / 100f64
    }
}

fn mod_ver1(data: &str) -> usize {
    let mut sum: usize = 0;
    for byte in data.as_bytes() {
        sum += *byte as usize;
    }
    sum % SIZE
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_mod_ver1() {
        // 6 if SIZE is 10;
        // 106 if SIZE is 255;
        assert_eq!(mod_ver1("hello#1"), 6);
        assert_eq!(mod_ver1("#1holle"), 6);
        assert_eq!(mod_ver1("h#ell1o"), 6);

        let mut hash_table: HashTable = HashTable::new(Box::new(mod_ver1));

        hash_table.insert("h1", "w1");
        hash_table.insert("1h", "1w");
        assert_eq!(hash_table.load_factor(), 0.1f64);
        hash_table.insert("xy", "xy");
        assert_eq!(hash_table.load_factor(), 0.2f64);

        assert_eq!(hash_table.get("h1"), Some("w1"));
        assert_eq!(hash_table.get("1h"), Some("1w"));
        assert_eq!(hash_table.get("xy"), Some("xy"));
        assert_eq!(hash_table.get("yx"), None);

        hash_table.delete("yx");
        hash_table.delete("1h");
        assert_eq!(hash_table.get("1h"), None);
        assert_eq!(hash_table.load_factor(), 0.2f64);
        hash_table.delete("h1");
        assert_eq!(hash_table.get("h1"), None);
        assert_eq!(hash_table.load_factor(), 0.1f64);

        assert_eq!(
            (hash_table.table.len(), hash_table.table.capacity()),
            (10, 10)
        );
    }
}
