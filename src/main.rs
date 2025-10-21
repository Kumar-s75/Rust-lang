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
fn main(){
    let a=5;
   if  is_even(a){
    println!("{} is even",a);

   } else{
    println!("{} is odd",a  );
}
}

fn is_even(a:u32)->bool{
    return a%2==0;
}
///
