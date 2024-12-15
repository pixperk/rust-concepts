fn main() {
    let mut book = String::from("Harry Potter"); // Our original book
    
    // Multiple people can read the book
    let reader1 = &book;  // First person reading
    let reader2 = &book;  // Second person reading
    println!("{} is being read by multiple people", reader1);
    println!("{} is being read by multiple people", reader2);
    
    // Only one person can write in the book at a time
    let note_taker = &mut book;  // One person making notes
    note_taker.push_str(" - with notes");
/*     let another_note_taker = &mut book; //cannot borrow `book` as mutable more than once at a time */
    println!("Book after notes: {}", note_taker);

    //since the first mut reference isn't in use, we can make other mut ref
    let another_note_taker = &mut book;
    another_note_taker.push_str("-more notes");
    println!("Book after notes: {}", another_note_taker);
}


