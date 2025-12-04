fn main() {
   
   let s1: String = String::new();
   let s1 = String::from("Hello World");

   let mut str1 = "Hello World".to_string();

    let mut ptr = str1.as_mut_ptr();
    
    let l = str1.len();
    let c =str1.capacity();
    std::mem::forget(str1); // says dont drop it here
        unsafe{
            let mut s1 = String::from_raw_parts(ptr, l,c);
            println!("{}",s1);
        }
    
   let mut arr1 = [10,20,30,40,50]; // stack allocation, it might have implemented Copy trait

   let mut vec1 = Vec::from(arr1);

   vec1.push(100);


   let vec2 = vec1; // Copy has not been implemented
   let mut arr2 = arr1;

   arr1[0] = 999;

   println!("{:?}",arr2); // Display or Debug traits
   println!("{}",arr2[0]);
   println!("{:?}",vec2);
  
}
