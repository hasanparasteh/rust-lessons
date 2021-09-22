fn main() {
    println!("Hello, world!");

    let x = {
        let y = 3;
        y + 1
    };
    println!("x is: {}", x);

    x_looper();
    x_while();
    for_looper();
    for_looper_nums();
}

fn x_looper(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn x_while(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        
        number -= 1;
    }
}

fn for_looper(){
    let a = [10, 20, 30, 40];

    for item in a.iter() {
        println!("the value is: {}", item);
    }
}

fn for_looper_nums(){
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}