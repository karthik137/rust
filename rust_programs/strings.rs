fn main() {

    let rand_string = "random string";

    println!("Length of the string {}", rand_string.len());

    //split strings
    let (first, second) = rand_string.split_at(6);

    println!("First : {} , Second : {}", first, second);

    //get number of characters in the string
    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();

    //prints char iterator
    println!("Printing chars = {:?}", chars);
    let mut ind_char = chars.next();

    loop {

        //this is similiar to switch case
        match ind_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        ind_char = chars.next();
    }

/**
 * Iterate by words
 */
    let mut iter = rand_string.split_whitespace();

    let mut ind_word = iter.next();

    loop {
        match ind_word {
            Some(x) => println!("{}", x),
            None => break,
        }

        ind_word = iter.next();
    }


/**
 * Iterate by lines
 */

let rand_string2 = "I am a random string\nThere are other string like it\nThis string is the best";

let mut lines = rand_string2.lines();

let mut ind_line = lines.next();

loop {
    match ind_line {
        Some(x) => println!("{}", x),
        None => break,
    }
    ind_line = lines.next();
}


//find a word 

println!("Find best : {}", rand_string2.contains("best"));





}