use std::fmt;
use std::fmt::Display;

fn main() {
    
    let e1 = Empty::new();

    e1.greet();

    let e2 = e1;

    println!("{}",e1);


}


struct Empty; // what is the size -- 0 

impl Empty{
    fn new()->Self{
        Self
    }
}

impl Display for Empty{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           write!(f, "Empty")
       }
}

impl Copy for Empty{}

impl Clone for Empty{
     fn clone(&self) -> Self {
        *self
    }
}

impl Empty{
    fn greet(&self){
        println!("Hello Empty structure")
    }
}




// MarkerStruct or Empty Struct
// UnitStruct
// Touple Struct
// Structure
