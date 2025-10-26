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

// fn main(){
//     let mut x=5;
//     println!("The value of x is:{x}");
//     x=6;
//     println!("The value of x isa {x}");
// }
//mut provides an option to update the value of the variable.

///writing basic functions in rust

///hamesha kisi function ko varible ke through call karo
// fn main(){
   
//   let result = do_sum(7,5);
//   println!("The sum is {}",result);
// }
// fn do_sum(a:i32,b:i32)->i32{
//     return a+b;
// }

/////ownership of heap variables
// fn main(){
//     let str=String::from("KumarSaurabh");
//     let len=get_length(str);
//     println!("{}",len);

//     println!("{}",str);
// }
// fn get_length(str:String)->usize{
//    return str.len();
// }
// This code fails because the ownership of str is moved to the get_length function.
//  Once the function’s scope ends, the str variable is no longer valid.

//There are possibly two fixes of these one can be 

// fn main(){
//     let str=String::from("KumarSaurabh");
//     let (str,len)=get_length(str);
//     println!("{} {}",str,len);

// }

// fn get_length(str:String)->(String,usize){
//     let len=str.len();
//     return (str,len);
// }

fn main() {
    let str = String::from("Kumar");
    let len = get_length(str);
    println!("{}", len);

    print!("{}", str);
}

fn get_length(str: String) -> usize {
    return str.len()
}

///immutable reference and mutable referemce
fn main(){
    let mut s1:String=String:: from ("Kumar");
    s1.push_str(string:"Saurabh");
    let s2:&String=&mut s1;
    s2.push_str(string:"Saurabh 2");
    
}
/*if we use &mut then mutable reference and if only & immutable reference */

fn main(){
    let s1=String:: from ("Kumar");
    let s2=&s1;
    let s3=&s1;
    let s4=&s1;

    println!("{},{},{},{}",s1,s2,s3,s4);

}

///////Structs in Rust
struct User{
   active:bool,
   username:String,
   email:String,
   sign_in_count:u64,
}

fn main() {
    let user1=User{
        active:true,
        username:String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,
    };
    print!("User 1 username:{:?}",user1.username);
}

////only stack types in struct
// code 1)
struct User{
    active:bool,
    sign_in_count:u64,
}
fn main(){
    let mut user1=User{
        active:true,
        sign_in_count:1,
    };
    print_name(user1);
    print!("User 1 username:{}",user1.active());
}
/*error - cannot use borrowed value */
fn print_name(user1:User){
   print!("User 1 username:{}",user1.active());
}


/*copy trait */

#[derive(Copy,Clone)]
struct User{
    active:bool,
    sign_in_count:u64,
}

fn main(){
    let mut user1=User{
        active:true,
        sign_in_count:1,
    };
    print_name(user1);
    print!("User 1 username: {}",user1.active);
}
fn print_name(user1:User){
    print!("User 1 username:{}",user1.active);
}
/*error goes away as user1 is copied */
/*Strings */
struct User{
    active:bool,
    sign_in_count:u64,
    username:String,
}
fn main(){
  let mut user1:User{
    active:true,
    sign_in_count:1,
    username:"harkirat".to_string()
  };

  change_name(user1);
  print!("User 1 username:{}",user1.active);
}
fn change_name(user1:User){
   print!("User 1 username:{:?}",user1.active);
}

/*in case of strings we cannot use copy trait, use clone trait instead.*/
struct User{
  active:bool,
  sign_in_count:u64,
  username:String,
}
fn main(){
    let mut user1:User{
        active:true,
        sign_in_count:1,
        username:"harkirat".to_string()
    };
    change_name(user1.clone())
    print!("User 1 username:{}",user1.active);
}
fn change_name(user:User1){
    print!("User 1 username :{:?}",user1.active);
}
/*Implementing structs*/
 /*Passing in &self as the first argument to a function */
// struct Rect{
//   width:u32,
//   height:u32,
// }
// impl Rect{
//     fn area(&self)->u32{
//         self.width*self.height
//     }
// }
// fn main(){
//     let rect=Rect{
//         width:30,
//         height:50,
//     };
//     print!("The area of rectnagle is {}",rect.area());
// }
//  /*not passing &self as an argument*/
//  struct Rect{
//     width:u32,
//     height:u32,
//  }

//  impl Rect{
//     fn print_str(){
//         println!("Inside the rect struct");
//     }
//  }

//  fn main(){
//     Rect::print_str();
//  }

//  /*enums*/

//  enum Direction{
//     North,
//     East,
//     South,
//     West
//  }
//  fn main(){
//     let my_direction=Direction::North;
//     let new_direction=my_direction;
//     move_around(new_direction);
//  }
//  fn move_around(direction:Direction){
//     //implements logic to move a character around
//  }
// /* enums with values*/
// enum Shape(){
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64,f64)

// }

// fn calculated_area(shape:Shape)->f64{
//     return 0
// }

// fn main(){
//     let circle=Shape::Circle(5.0);
//     let square=Shape::Square(4.0);
//     let rectangle=Shape::Rectangle(3.0,6.0)
// }

// ///enum paatern matching
// enum  Shape{
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64,f64),
// }
// fn calculated_area(shape:Shape)->f64{
//     match shape{
//         Shape::Circle(radius)=>PI*radius*radius,
//         Shape::Square(side_length)=>side_length*side_length,
//         Shape::Rectangle(width,height)=>width*height,
//     }
// }
// fn main(){
//     let circle=Shape::Circle(5.0),
//     let Square=Shape::Square(4.0),
//     let rectangle=Shape::Rectangle(3.0,6.0)

//     println!("Area of Circle is:{}",calculated_area(circle));
//     println!("Area of square: {}",calaculated_area(square));
//     println!("Area of rectangle:{}",calculated_area(rectangle));
// }

// /*error handling in rust */
// use std::fs::File;

// fn main(){
//     let greeting_file_result=fs::read_to_string("hello.txt");
// }
//  /*In case you are okay with errors then you can write Unwraps*/
//  use std::fs;

//  fn main(){
//     let greeting_file_result=fs::read_to_string("hello to txt");
//     print!("{}",greeting_file_result.unwrap());
//  }

// /*option enums*/
// /*instead of null return option */
// fn main(){
//     let my_string=String::from("raman");
//     match find_first_a(my_string){
//         Some(index)=>println!("The letter 'a' is found at index:{}",index),
//         None=>println!("The letter 'a' is not found in String.");
//     }

// }
// fn find_first_a(s:String)->Option<i32>{
//     for(index,character) in s.chars().enumerate(){
//         if character=='a'{
//             return Some(index as i32);

//     }
// }
// return None;
// }

/*crates */
use chrono::{Local,Utc},
/*use chrono::prelude*/
fn main(){
    let utc_time=Utc::now();
    let local_time=Local::now();
    println!("local time is {}",utc_time);
    println!("native time is {}",local_time);
}

use dotenv::dotenv;
use std::env;

fn main(){
    dotenv().ok();
    let var=env::var("REDIS_ADDRESS").unwrap();
    println!("{}",var);


}

/*uuid*/
use uuid::Uuid;

fn main(){
    let random_uuid=Uuid::new_v4();
    println!("{}",random_uuid);
}


/*unwraps are not good at all if value exists it gets stored and if it isn't then it   provides error */

//sum_u32
fn sum_u32(a:u32,b:u32)->u32{
    return a+b;
}



//sum_f32
fn sum_f32(a:f32,b:f32)->f32{
    return a+b;
}



/*Generics*/

/*create a first_element*/

pub fn main(){
    let v =vec![1,2,3];
    let v2=vec![String::from("Harkirat"),String::from ("Singh")];
    let v3=vec![1.0,2.0,3.0];
    println!("{}",first_element(v).unwrap());
    println!("{}",first_element(v).unwrap());
    println!("{}",first_element(v).unwrap());

}

fn first_element<T>(v:Vec<T>)->Option<T>{
    return v.into_iter().nth(0);
}

fn does_exist<T:PartialEq>(v:Vec<T>,element:T)->bool{
    let mut iter=v.iter();
    while let Some(value)=iter.next(){
        if *value==element{
            return true;
        }
    }
    return false;
}

fn biggest_element<T:Ord>(a:T,b:T)->T{
    if a>b{
        return a;
    }
    return b;
}

use std::fmt::Display;

fn print_variable<T:Display>(a:T){
    println!("{}",a);
}

/*Generics over structs */
struct Rect<T>{
    width:T,
    height:T,
}
impl<T:Mul<Output=T>+Copy> Rect<T>{
    pub fn area(&self)->T{
        return self.height*self.width
    }
}

fn main(){
    let r=Rect{
        width:10,
        height:20
    };
    println!("{}",r.area());
}

impl Rect<u32>{
    pub fn area(&self)->T{
        return self.height*self.width
    }
}

// */trait*/
trait Shape{
    fn area(&self)->f32;
}

struct Rect{
    width:f32,
    height:f32
}

impl Shape for Rect{
    fn area(&self)->f32{
        return self.width*self.height
    }
}

fn get_area(shape:impl Shape)->f32{
    return shape.area();
}

fn get_area_2<T:Shape>(shape:T)->f32{
    return shape.area();
}

fn get_area_3<T>(shape:T)->f32{
    where T:Shape
    {
        return shape.area();
    }
}

/*macros*/
/*declerative macros*/
macro_rules! say_hello{
    ()=>{
        println!("Hello World!");

    };
}

fn main(){
    say_hello!();
}

macro_rules! create_function{
    ($func_name:ident)=>{
        fn $func_name(){
            println!("Hello from {}",stringify!($func_name));
        }
    };

}

create_function!(hello);

fn main(){
    hello();
}

/*procedural macros */
#[derive(Serialize, DeSerialize)]
struct User{
    username:String,
    password:String,
    age:u32
}

/*custom derived macros*/
#[derive(Serialize,DeSerialize)]
struct User{
    username:String,
    password:String,
    age:u32
}
#[route("GET")]
fn home(){
    println!("Welcome to the home page!");

}
#[route("POST")]
fn create_post(){
    println!("Creating a new post!");
}

/*function derived macros*/


use serde ::{Serialize,DeSerialize};

#[derive(Serialize,Deserialize)]

struct  User{
    // #[serde(rename="user_name")]
     username:String,
     // #[serde(rename="pass_word")]
     password:String,
     // #[serde(rename="user_age")]
     age:u32,

}
fn main(){
    let user=User{
        username:String::from("Alice"),
        password:String::from("password123"),
        age:30,
    };
     // Serializing to JSON
     let json=serde_json::to_string(&user).unwrap();
     println!("{}",json);
      // Prints: {"user_name":"Alice","pass_word":"password123","user_age":30}
}

/*Debug and Display trait*/
struct Rect{
   width:u32,
   height:u32
}
fn main(){
    let s=Rect{
      width:100,
      height:100
    };
    println!("{}",s);
}

/*Display trait*/
impl std::fmt::Display for Rect{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"({},{})",self.width,self.height)
    }
}