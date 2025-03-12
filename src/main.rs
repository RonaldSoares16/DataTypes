use::std::io;

fn main() {
    loop {
        println!("Please enter a number corresponding to a test function");
        let mut choose = String::new();
        io::stdin().read_line(&mut choose).expect("Failed to read line");
        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choose {
            1 => tuple(),
            2 => while_loop(),
            _ => println!("Please type a valid number."),
        }
        break;
    }
}

fn tuple(){
    let x: (u8, u8, &str) = (1, 2, "terceiro");
    println!("x.0 = {}, x.1 = {}, x.2 = {}", x.0, x.1, x.2);
}

fn while_loop(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}