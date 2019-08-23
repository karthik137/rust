fn main() {

    let mut vect1 = vec![1,2,3,4,5];

    println!("Item 2 : {}", vect1[1]);

    for i in &vect1 {
        println!("Vect : {}", i);
    }

    vect1.push(6);
    println!("Vector after push : {:?}", vect1);
    vect1.pop();
    println!("Vector after pop : {:?}", vect1);


}