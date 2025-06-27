// trait Summary {
//     fn summarise(&self)->String;
// }
// struct User
// {
//     name:String,
//     age:u32,
// }
// impl Summary for User{
//     fn summarise(&self)->String {
//         println!("{} is {} aged",self.name,self.age);
//     }
// }
// fn main()
// {
//     let user=User{
//         name:String::from("Kaushik Kajjam"),
//         age:22,
//     };
//     user.summarise();
// }

// fn notify(u:impl Summary)
// {
//     println!("{}",u.summarise());
// }