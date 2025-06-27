fn main()
{
    let name=String::from("Hello Kaushik");
    let ans =first_word(name);
    println!("ans is {}",ans);

    print!("##################################################################\n");
    let mut word=String::from("Hello World");
    let word2=&word[0..5];
    println!("{}",word2);
    //word.clear();
    println!("{}",word2);
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    let big1=largest(1, 2);
    let big2=largest('a', 'b');

    println!("{} number {} char",big1,big2);

}
fn largest<T:std::cmp::PartialOrd>(a:T,b:T)->T{
    if a > b
    {
        a
    }else {
        b
    }
}

fn first_word(str:String)->String
{
    let mut ans=String::from("");
    for i in str.chars(){
        if i == ' '{
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}