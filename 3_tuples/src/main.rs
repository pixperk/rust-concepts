//parameter as a tuple and result as tuple
fn add_and_multiply(numbers : (i32, i32))->(i32, i32){
    let sum = numbers.0 + numbers.1;
    let pdt = numbers.0 * numbers.1;
    (sum, pdt)
}

//destructuring
fn swap_tuple(str_and_int : (i32, String))->(String, i32){
    let (int, str) = str_and_int;
    (str, int)
}


fn main() {
    let sum_and_pdt = add_and_multiply((9,5));
    println!("Answer is {:?}", sum_and_pdt);
    let (str, int) = swap_tuple((2, String::from("two")));
    println!("Ans :  {str} & {int}")


}
