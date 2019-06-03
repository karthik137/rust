fn print_sum(a: i8, b: i8){
    println!("Sum is {}", a+b);    
}

fn main(){
    print_sum(5,5);
    println!("{}", plus_one(10));

    let test = plus_two;

    let c = test(5);
    println!("{}", c);
}


// Returning values

fn plus_one(a: i32) -> i32 {

    let test_var = 0;

    //test_var = test_var + 1;
    println!("{}", test_var);
    return a + 1;
     
}

fn plus_two (a: i32) -> i32 {
    return a + 2; //return
}


// function pointers, usage as a data type

