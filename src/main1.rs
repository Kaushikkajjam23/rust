// fn main() {
//     println!("Hello, world!");

//     let ans = is_even(25);
//     println!("{}", ans);

//     let fib5 = fib(10);
//     println!("{}", fib5);

//     let mystring=String::from("Hello World !!");
//     let length=get_string_length(&mystring);
//     println!("length os the string {} is {}",mystring,length);

//     let user=User
//     {
//         name:String::from("Kaushik"),
//         last_name:String::from("Kajjam"),
//         age:22,
//     };
//     println!("{} {} {} ",user.name,user.last_name,user.age)
// }

// fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }

// fn fib(num: i32) -> i32 {
//     let mut a = 0;
//     let mut b = 1;

//     if num == 0 {
//         return a;
//     }
//     if num == 1 {
//         return b;
//     }

//     for _i in 2..=num {
//         let temp = b;
//         b = b + a;
//         a = temp;
//     }
//     return  b;
// }

// fn get_string_length(s:&str)->usize
// {
//     s.chars().count()
// }

// struct User
// {
//     name:String,
//     last_name:String,
//     age:i32,
// }

// struct Rect {
//     width: i32,
//     height: i32,
// }

// impl Rect {
//     fn area(&self) -> i32 {
//         self.width * self.height
//     }

//     fn perimeter(&self, _num: i32) -> i32 {
//         // `_num` used to silence warning (or remove it entirely if not needed)
//         2 * (self.width + self.height)
//     }

//     fn debug() -> i32 {
//         1
//     }
// }

// fn main() {
//     let rect1 = Rect {
//         width: 10,
//         height: 20,
//     };

//     println!("Area of Rectangle is {}", rect1.area());
//     println!("Perimeter of Rectangle is: {}", rect1.perimeter(1));
//     println!("Debug the Rectangle: {}", Rect::debug());
// }
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

enum CustomOption {
    Some(i32),
    None,
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    println!("Area of rectangle is {}", calculate_area(rect));
    
    let circle = Shape::Circle(1.0);
    println!("Area of circle is {}", calculate_area(circle));

    println!("#########################################################################################");

    // let index=find_first_a(String::from("preethiaa"));
    // match index
    // {
    //     Some(value)=>println!("index is {}",value),
    //     None=> println!(" a not found"),
    // }

    let index: CustomOption = find_first_a(String::from("preethiaa"));
    match index {
        CustomOption::Some(value) => println!("Index is {}", value),
        CustomOption::None => println!("'a' not found"),
    }
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    area
}

fn find_first_a(s: String) -> CustomOption {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None;
}


