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

struct Rect
{
    width:i32,
    height:i32,
};
impl Rect{
    fn area(&self)->i32{
        self.width*self.height
    }
}
fn main()
{
    let rect1=Rect{
        width:10,
        height:20,
    };
    println!("Area of Rectangle is {}",rect1.area());
}