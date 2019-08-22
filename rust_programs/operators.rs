fn main() {

let age_old = 6;

if (age_old == 5) {
    println!("Go to kindergarden");
}else {
    if (age_old > 5 && age_old <=18){
        println!("Go to grade {}", age_old - 5);
    }else {
        if (age_old <= 25 && age_old > 18){
            println!("Go to kindergarten");
        }else {
            println!("Do waht you want!");
        }
    }
}
    
    println!("!true = {}", !true );
    println!("true || false = {}", true || false);
    println!(" true != false = { }", (true != false));

    let can_vote = if (age_old >= 18) {true} else 
    {false};

    println!(" can_vote = {}", can_vote);
    //can_vote = true; throws error because it is not mutable
}