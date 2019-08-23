fn main(){

say_hello("Kirito");

}

fn say_hello(name: &str){
    println!("Hello {}", name);

    println!("5+4 = {}", get_sum(5,4));
}


fn get_sum(num1: i32, num2: i32) -> i32 {
    num1+num2
}