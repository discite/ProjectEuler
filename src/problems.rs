pub fn problem_1() {
    let mut counter:u32 = 0;
    for i in 0u32..1000 {
        if (i % 3 == 0) || (i % 5 == 0) {
            counter += i;
        }
    }
    println!("Problem 1 is {}",  counter);
}