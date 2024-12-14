use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("Name", "Alice");
    map.insert("Age", "24");
    println!("{:?}", map);
}
