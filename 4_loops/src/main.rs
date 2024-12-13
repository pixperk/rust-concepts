fn main() {
    for multiplicand in 1..11{ //exclusive of 11
        println!("Table of {multiplicand}");
        for multiplier in 1..11{
            let pdt: i32 = multiplicand*multiplier;
            println!("{multiplicand}*{multiplier}={pdt}")
        }
    }

    let mut i = 0;
    while i <= 5 {
        println!("{i}");
        i+=1;
    }

    let mut count = 9;
    loop{
        println!("{count}");
        count-=1;

        if count==0{
            break;
        }
    }

    //continue
    for number in 1..=10 {
      if number%2!=0{
        continue;
      }
      println!("{number}")
    }
}
