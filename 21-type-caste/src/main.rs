use std::{num::ParseIntError, path::Display};

fn main() {
    let num1 = 100i8;
    let num2: i32 = num1 as i32;

    let num2: u32 = 2555;
    let num3: u8 = num2 as u8;

    // let float1 = 1231212312.123;
    // let num = float1 as i8;

    println!("{}", num3);
    println!("{:b}", num2);
    println!("{}", 0b11111011);

    let str1 = "true";

    let result:Result<bool,_> = str1.parse();

    match result {
        Ok(v)=> println!("{}",v),
        Err(e)=>println!("{}",e),
    }

    let age= 100;
    let is_marreed = true;
    let sal = 12312.123;
    let name = "Jiten";

   let s1= format!("age:{} name:{} married:{} sal:{}",age,name,is_marreed,sal);

   s1.to_json()
   println!("{}",s1);
   let str1= format!("{}",age);
   let str1=age.to_string();
//   let s1= age.to_string()+name+&is_marreed.to_string();

    let val= "312312".parse::<i32>();

    let val : Result<i32,ParseIntError>="312312".parse();


   let num:i32 =12313;

   let w = num.what();


   let k = (10,20);
   println!("{:?}",k);
}

//10011 1111011


trait What{
    fn what(self)->String;
}

// i32 is a predefined type 
impl What for i32{
    fn what(self)->String {
        return "i32".to_string()
    }
}


