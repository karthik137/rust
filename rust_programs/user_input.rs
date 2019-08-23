use std::io::stdin;

fn main(){

    'outer: loop{
        let number: i32 = 10;
        println!("Pick a number");

        loop {
            let mut line = String::new();
            let input = stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(None, 
            |_|line.trim().parse().ok());

            match guess {
                None => println!("Enter a number"),
                Some(n) if n == number => {
                        println!("You guessed it");
                        break 'outer;
                }
                Some(n) if n < number =>
                println!("Too low"),
                Some(n) if n > number =>
                println!("Two High"),
                Some(_) => println!("Error")
            }            
        }
    }
}