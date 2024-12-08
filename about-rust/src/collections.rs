use std::collections::HashMap;

pub fn vec() -> Vec<i32> {
    let mut data = vec![];
    for i in 1..10 {
        data.push(i);
    }

    data
}

pub fn hash_map(kv_data: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut set = HashMap::with_capacity(kv_data.len());
    for kv in kv_data {
        set.insert(kv.0, kv.1);
    }

    set
}
