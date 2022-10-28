fn main() {
    fn fizz_buzz() {
        let mut counter = 0;
        for x in 0..301 {
            if x % 3 == 0 && x % 5 == 0 {
                println!("fizz buzz");
                counter = counter + 1;
            }
            else if x % 3 == 0 {
                println!("fizz")
            }
            else if x % 5 == 0 {
                println!("buzz")
            }
            else {
                println!("{} not divisible", x)
            }
        }
        println!("fizz buzz occurred {} times", counter)
    }

    fizz_buzz()
}
