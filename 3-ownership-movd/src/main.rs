fn main() {

    let a = 100;

    let b = true;

    let str1 = "Hello Impact Analytics";

    let c: i32 = a ; // This is move , or ownership transfer but for this variable it does copy
    
   // The ownership is not transffered but it does Copy(Traits)
   println!("{}",a);

   let  s1 = "Hello World".to_string();


   let mut s2 = s1; // ownership transferred(move) , the previous owner become obsolete
   // drop(&s1)
   //println!("{}",s1);

   let mut s3 = s2.clone(); // deep copy

   s2= get_length2(s2); 

   let l: usize =get_length1(s2);
   println!("length:{}",l);
   //println!("{}",s2)

   let mut l:usize = 0;

    (s3,l) = get_length3(s3);

   let s1: &str = "Hello World";
   let s2 = s1;

   println!("{}",s1);


   let m: &mut i32 = &mut 10;
   println!("{}",m);// though it is a reference ,unless told , it always prints the value
   *m = 20; // derefenrece 

   let mut a = 10;
   let b = &mut a;
   *b = 20;

   println!("{}",a);


}

// 1. Ownersip
// 2. Borrowing 
// 3. Lifetimes

fn get_length1(s:String)->usize{
    return s.len() // uisize 
    // drop(s)
}

fn get_length2(s:String)->String{
    println!("{}",s.len());
    return s;
}

fn get_length3(s:String)->(String,usize){
   let l = s.len();
    return (s,l);
}