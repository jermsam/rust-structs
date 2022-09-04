fn main() {
    println!("Hello, world!");
    immutable_instances();
    mutable_instances();

    let email = String::from("pauldoe@example.com");
    let username = String::from("Paul Doe");

    let login_x = build_login(email,username);
    println!("{} : {}",login_x.username, login_x.email);

    let red = Color(255,0,0);
    let origin = Point(0,0,0);

    println!("{} : y = {}",red.0, origin.1);

    let rect_a = construct_rectangle(32,64);
    let rect_b = construct_rectangle(48,15);

    let area = calculate_area(&rect_a);

    println!("Area of {:?} = {}",rect_a,area);
    dbg!(&rect_a);
    dbg!(&area);
   println!("Perimeter of {:?} = {}", rect_a, rect_a.calculate_perimeter());
   println!("Can {:?} fit in {:?}? {}", rect_b, rect_a, rect_a.can_hold(&rect_b));

    let width = 10;
    let height = 24;
    let my_box = Box::new(&width, &height);
    let square_box = Box::square(6);
    let box_area = my_box.area();
    let box_perimeter = my_box.perimeter();
    println!("Box with Width {} and Height {} has Area {} and Perimeter {}.", width,height,box_area, box_perimeter);
    println!("Can {:?} fit in {:?}? {}", square_box, my_box, my_box.can_hold(&square_box));
}


struct Login {
    email: String,
    username: String,
}

fn immutable_instances () {
    let login_a = Login {
        email: "johndoe@example.com".to_string(),
        username: "John Doe".to_string()
    };

    let login_b = Login {
        email: "janedoe@example.com".to_string(),
        username: "Jane Doe".to_string()
    };

    println!("{} : {}. {} : {}. ",login_a.username ,login_a.email, login_b.username, login_b.email);

}
fn mutable_instances () {
    /* Note that the entire instance must be mutable; Rust doesn’t allow us
    to mark only certain fields as mutable.*/
    let mut login_a = Login {
        email: "johndoe@example.com".to_string(),
        username: "John Doe".to_string(),
    };
    println!("{} : {}. ",login_a.username ,login_a.email);
    // struct update syntax
    let  login_b = Login {
        email: "johndoe@two.com".to_string(),
        ..login_a
    };

    login_a.email= String::from("xyz.example.com");
    println!("{} : {}. ",login_b.username ,login_b.email);
}

fn build_login(email: String, username: String) -> Login{
    Login {
        email,
        username
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn construct_rectangle (width: u32, height: u32) -> Rectangle {
    Rectangle {
        width,
        height
    }
}
fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn calculate_perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Box {
    x: u32,
    y: u32
}

impl Box {
    // Associated functions that aren’t methods are often used for constructors t
    fn new(x:&u32, y: &u32) -> Box {
        Self {
            x: *x,
            y: *y
        }
    }

    fn square(size: u32) -> Self {
        Self {
            x:  size,
            y: size,
        }
    }

    fn area(&self) -> u32 {
        self.x * self.y
    }
    fn perimeter(&self) -> u32 {
       2 * (self.x + self.y)
    }

    fn can_hold(&self, other: &Box) -> bool {
        self.x> other.x && self.y > other.y
    }

}