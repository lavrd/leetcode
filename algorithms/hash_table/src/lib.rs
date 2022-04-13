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
    buckets: Vec<Vec<Value>>,
}

impl HashTable {
    fn new(hash_fn: HashFn) -> Self {
        let mut buckets: Vec<Vec<Value>> = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            buckets.push(vec![]);
        }
        HashTable { hash_fn, buckets }
    }

    fn insert(&mut self, key: &str, data: DataType) {
        let hash: usize = (self.hash_fn)(key);
        let bucket = self.buckets.get_mut(hash).unwrap();
        if !bucket.is_empty() {
            for val in bucket.iter_mut() {
                if val.key == key {
                    val.data = data;
                    return;
                }
            }
        }
        bucket.push(Value::new(key, data));
    }

    fn get(&self, key: &str) -> Option<DataType> {
        let hash: usize = (self.hash_fn)(key);
        let bucket = self.buckets.get(hash).unwrap();
        for val in bucket {
            if val.key == key {
                return Some(val.data);
            }
        }
        None
    }

    fn delete(&mut self, key: &str) {
        let hash: usize = (self.hash_fn)(key);
        let bucket = self.buckets.get_mut(hash).unwrap();
        for (i, val) in bucket.iter().enumerate() {
            if val.key == key {
                bucket.remove(i);
                return;
            }
        }
    }

    fn load_factor(&self) -> f64 {
        let mut non_empty: u64 = 0;
        for bucket in &self.buckets {
            if !bucket.is_empty() {
                non_empty += 1;
            }
        }
        let load_factor = non_empty as f64 / SIZE as f64;
        f64::trunc(load_factor * 100f64).floor() / 100f64
    }
}

fn hash_fn_mod(data: &str) -> usize {
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
        assert_eq!(hash_fn_mod("hello#1"), 6);
        assert_eq!(hash_fn_mod("#1holle"), 6);
        assert_eq!(hash_fn_mod("h#ell1o"), 6);

        let mut hash_table: HashTable = HashTable::new(Box::new(hash_fn_mod));

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

        hash_table.insert("jer_1", "lor_2");
        assert_eq!(hash_table.get("jer_1"), Some("lor_2"));
        hash_table.insert("jer_1", "lor_009");
        assert_eq!(hash_table.get("jer_1"), Some("lor_009"));

        assert_eq!(
            (hash_table.buckets.len(), hash_table.buckets.capacity()),
            (10, 10)
        );
    }
}
