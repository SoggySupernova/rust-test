pub fn main() {
    let mut a = 1;
    let mut b = 1;
    while a <= 1000 {
        b = a;
        println!("\nCollatz sequence for {}: ", a);
        // std::thread::sleep(std::time::Duration::from_millis(10));
        while b != 1 {
            print!("{} ", b);
            if b % 2 == 0 {
                b /= 2;

            } else {
                b = 3 * b + 1;
            }
        };
        print!("1");
        a += 1;
    }
    print!("\n");
}