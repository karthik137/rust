fn main() {


let mut circle1 = Circle{
    x: 10.0, y: 10.0, radius: 10.0
};

println!("X: {}, Y: {} R : {}", circle1.x, circle1.y, circle1.radius);

println!("Circle Radius : {}", get_radius(&circle1));

println!("Circle X : {}", circle1.get_x());

println!("Circle area : {}", circle1.area());


let mut rect1 = Rectangle {
    height: 10.0, width: 10.0
};

println!("Rect area : {}", rect1.area());


let hulk = Hero::Strong(100);
let quickSilver = Hero::Fast;
let spiderman = Hero::Info {name: "Spiderman".to_owned(), secret: "Peter".to_owned()};


get_info(hulk);
get_info(quickSilver);
get_info(spiderman);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,

}

fn get_radius(Circle: &Circle) -> f64 {
    Circle.radius 
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}


struct Rectangle {
    height: f64,
    width: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
       self.height * self.width
    }
}

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero){
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons",i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        }
    }
}