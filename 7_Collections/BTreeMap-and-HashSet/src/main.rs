use std::collections::{BTreeMap, HashSet};

fn main() {
    let mut grades = BTreeMap::new();

    // Insert values (they'll be stored in sorted order of keys)
    grades.insert("Carol", 93);
    grades.insert("Alice", 95);
    grades.insert("Bob", 87);

    // Iterate over sorted keys
    for (name, grade) in &grades {
        println!("{}: {}", name, grade);
    }

       // Create a new HashSet
       let mut languages = HashSet::new();
    
       // Insert elements
       languages.insert("Rust");
       languages.insert("Python");
       languages.insert("JavaScript");
       
       // Duplicates are automatically ignored
       languages.insert("Rust"); // Won't add a duplicate
       
       // Check if an element exists
       println!("Contains Rust? {}", languages.contains("Rust"));
       
       // Remove an element
       languages.remove("Python");
       
       // Iterate over elements
       for lang in &languages {
           println!("{}", lang);
       }
       
       // Set operations
       let set1 = HashSet::from([1, 2, 3]);
       let set2 = HashSet::from([3, 4, 5]);
       
       // Union
       let union: HashSet<_> = set1.union(&set2).copied().collect();
       
       // Intersection
       let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
       println!("Union : {:?},", union);
       println!(" Intersection: {:?}",intersection);
}
