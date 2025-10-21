//////Numbers

// fn main() {
//     println!("Hello, world!");
// }

// fn main(){
//     let x:u32=1;
//     println!("{}",x);

// }

// fn main(){
//     let x:i32=-1;
//     println!("{}",x);
// }

//Booleans
//simple boolean function
// fn main(){
//     let a=5;
//    if  is_even(a){
//     println!("{} is even",a);

//    } else{
//     println!("{} is odd",a );
// }
// }

// fn is_even(a:u32)->bool{
//     return a%2==0;
// }
///
// fn main(){
//     let is_male=true;
//     let is_above_18=true;

//     if is_male{
//         println!("You are a male");

//     } else{
//         println!("You are not a male");
//     }

//     if is_male && is_above_18{
//         println!("You are a legal male");
//     }
// }
//////  strings
// fn main(){
//     let name=String::from("KumarSaurabh");
//     println!("{}",name);

// }
// fn main(){
//    let name:String=String::from ("KumarSaurabh");
//     println!("{}",name);
// }
/////i faced case sensitivity issue here.i.e. string=String(wrong)


///////Arrays
// fn main(){
//     let arr:[i32;5]=[1,2,3,4,5];
//     println!("{}",arr.len());
// }
// fn main(){
//     let arr:[i32;6]=[1,2,3,-4,5,6];
//     println!("{}",arr.len());
// }

////vectors

// fn main(){
//     let mut xs=vec![1,2,3];
    
//     print!("{}",xs.len());

//     xs.push(4);

//     print!("{}",xs.len());
// }

// fn main(){
//     let mut xs=vec![1,2,3];

//     println!("Before:{}",xs.len());

//     xs.push(4);

//     println!("After:{}",xs.len());
// }

/////Conditionals,loops
///we dont use parantheses after if statement in case of Rust
/// 1st example
// pub fn main(){
//     let x=99;
//     let is_even=is_even(x);
//     if is_even {
//         println!("{} is even",x);
//     }
//     else{
//         println!("{} is odd",x);
//     }
// }

// pub fn is_even(x:i32)->bool{
//     return x%2==0;
// }

///2nd example
// pub fn main(){
//     let str=String::from("kumar saurabh");
//     println!("First Name {}",get_first_name(str));
// }

// pub fn get_first_name(str:String)->String{

//     let mut first_name=String::from("");
//     for c in str.chars(){
//         if c == ' ' {
//             break
//     }
//     first_name.push(c);
// }
//   return first_name;
// }


/////mutable vs immutable
// fn main(){
//     let x=5;
//     println!("The value of x is:{x}");
//     x=6;
//     println!("The value of x isa {x}");
// }

fn main(){
    let mut x=5;
    println!("The value of x is:{x}");
    x=6;
    println!("The value of x isa {x}");
}