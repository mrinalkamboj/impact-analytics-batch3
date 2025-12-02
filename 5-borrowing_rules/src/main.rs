fn main() {
    let mut s1 = String::from("Hello Impact Analytics");

    let s2 = &s1;

    let s3 = &s1;

    let s4 = &s1;

    println!("{}", s2);

    println!("{}", s3);

    println!("{}", s4);

    {
        let s5 = &mut s1; // s5 --> 
        println!("{}", s5);
        let s5 = &mut s1;
        s5.push_str("! How are you doing !");
        // drop(s5)
    }

    {
        let s6 = &mut s1; // s6  lifetime
        s6.push_str("! How are you doing !");
        // drop(s6)
    } // s6


    append_str1(&mut s1, "Learning Rust");
    append_str2(&mut s1, "Learning Rust");

    let  s6: &mut String = &mut s1;

    let str1 = "Hello World"; // DS , raw_ptr, len
    //let str2 = "hello World".to_string(); // HP -->? raw_ptr, len, cap

    let str2 = str1.to_string();

    println!("{:p} {:p}",&str1,str1.as_ptr());

    println!("{:p} {:p}",&str2 ,str2.as_ptr());

    let  mut s1 = "hello World".to_string();

    let  s2 = " How are you doing".to_string();

    s1 = s1 +  &s2; // that happens at runtime

   //  s1.Add(&s2)

    // &str

}

//  Ownership --> Who owns the value
//  Borrowing. --> Temporarily taking the ownership of the value

// Borowing is more on variables which does not implement copy. It does not mean that rules are different for different type of variables

// There can be any number of immutable borrows, in a scope
// or
// there can be only one mutable borrow.


fn append_str1(s:&mut str,str1:&str){ // can pass either &str or &String 
    &s.to_string().push_str(str1); 
    //drop(s)
}

fn append_str2(s:&mut String,str1:&str){ // can pass only &String
    s.push_str(str1);
    //drop(s)
}

// &str -> ref of string slice 

// you can use &str or &String



// two strings 
// s1 = s1 and s2 , concatinate these twostrings +
// try with &str1 and then. &String