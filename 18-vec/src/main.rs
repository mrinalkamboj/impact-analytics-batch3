fn main() {
    let mut vec1 = vec![10, 20, 30, 40, 50]; // macro 
    //let mut vec2= Vec::new();

    //    vec2.push(100);
    //    Vec::push(&mut vec2, 100);
    //    vec2.push(200);
    //    vec2.push(200);

    let mut sum = 0;

    for i in &vec1 {
        sum += i;
    } // the ownership is given back to vec1
    println!("sum:{}", sum);

    println!("{:?}", vec1);

    let mut arr = [10, 20, 30, 40, 50];

    let mut ptr: *mut i32 = &mut arr[0] as *mut i32;

    // *const i32
    // *mut i32

    unsafe {
        for i in 0..= arr.len(){ // this is suppose to be array out of bounds
            println!("{}", *ptr); // print the value of a[i]
            ptr = ptr.add(1); // pinter arithmetic 
        }
        // let ptr:*mut i64 = 16161616 as *mut i64;
        // *ptr = 1231;
        // println!("{}",*ptr);
    }

    // for i in 0..=arr.len(){
    //     println!("{}",arr[i])
    // }

   
   

   let s = get_sum(&vec1);
   println!("Sum:{}",s);
//    let str1="Hello World";
//    let ptr: *mut u8 = str1.as_mut_ptr() as *mut u8;

}


fn get_sum(arr: &[i32]) -> i32 {
    // functions or methods are written for slices, not for arrays
    let mut sum = 0;
    for i in arr {
        sum += *i;
    }
    sum
}
