// fn main()
// {
//     let vec=vec![1,2,3,4,5];
//     let ans=even_filter(&vec);
//     println!("{:?}",ans);
//     println!("{:?}",vec);

// }
// fn even_filter(vec:Vec<i32>)
// {
//     let mut new_vec=Vec::new();
//     for val in vec {
//         if val%2==0 {
//             new_vec.push(val);
//         }
//     }
// }


// use std::collections::{btree_map::Values, HashMap};

// fn main()
// {
//     let mut users=HashMap::new();
//     users.insert(String::from("kaushik"), 22);
//     users.insert(String::from("Manasvini"), 25);

//     let first_user_age=users.get("kaushik");

//     match first_user_age
//     {
//         Some(age)=>println!("Age is {}",age),
//         None=>println!("User not found in the db"),
//     }
//     println!("###########################################################################################");
//     let input_vec=vec![(String::from("Kaushik"),22),(String::from("manasvini"),25)];
//     let hm=group_values_by_keys(input_vec);

//     println!("{:?}",hm);

//     println!("#################################################################################################");
//     let mut v1=vec![1,2,3,4,6,5,7,8,9];
//     let mut v1_iter=v1.iter_mut();
//     for val in v1_iter{
//         *val =*val*2;
//     }
//     println!("{:?}",v1);
//     println!("##################################################################################################");
//     while Some(val)=v1_iter.next(){
//         print!(" {},",val);
//     }
// }
// fn group_values_by_keys(vec:Vec<(String,i32)>)->HashMap<String,i32>
// {
//     let mut hm=HashMap::new();
//     for(key,value) in vec {
//         hm.insert(key, value);
//     }
//     return hm;
// }