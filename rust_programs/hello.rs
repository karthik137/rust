fn main() {
    println!("Hello World ");

    let number = 10;
    // declare variable mutable and datatype
    let mut age: i32 = 40;

    let let_x: char = 'x';

    println!("I am  {} years old. and char is {}", age , let_x);

    let (f_name, l_name) = ("derek", "Banas");

    let it_is_true:bool = true;

    println!("It is {0} than {1} is {0}", it_is_true, let_x);

    println!("{:.2}", 1.234);

    println!("Binary : {:b},  H: {:x}, O: {:o}", 10, 10, 10);

    println!("{ten:<ws$}, ello", ten=10, ws=5);
    println!("{ten:<0ws$}", ten =10, ws=6);

    println!(" 5+ 4 = {}", 5+4);
    println!(" 5-4 = {}", 5-4);
    println!(" 5/4 = {}", 5/4);
    println!(" 5*4 = {}", 5*4);
    println!(" 5%4 = {}", 5%4);
    println!(" 5**2 = {}", 5**4);
}