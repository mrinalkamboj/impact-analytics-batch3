static mut COUNTER:i32 = 100;

static mut GREET:&str = "Hello World";

const PI:f32 = 3.14;
#[allow(static_mut_refs)]

fn main() {
    let a = 100;
    let b = 200;

    let b1 = Box::new(100);
    let b2 = Box::new(200);
    {
    let r: Box<i32> = add_thru_ref(&a, &b);
    println!("r:{} ptr:{:p} ptr_in_box:{:p}",r,&r,r.as_ref());
    // drop(r)
    }
   let _ = add_thru_ref(&a, &b); // _ blank identifer

  // add_thru_ref(&a, &b);

   let _ = 100;
   let _ = "Hello World";

   let (_,_,r3)= calc(10,20);

    println!("{} {} {} ",unsafe{COUNTER},unsafe {
        GREET
    },PI);

    println!("referece of counter{:p}",unsafe{&COUNTER})

    // unsafe{

    //     let ptr:*const i32 = &COUNTER;

    // }

}

// fn add_thru_ref(a:&i32,b:&i32)->&i32{
//     let  mut r = 0; // create a variable here , what is the lifetime of the variable

//      r = *a + *b ;

//      return &r; // return the reference from the function, which the variable itself is not at all available
// }

fn add_thru_ref(a:&i32,b:&i32)->Box<i32>{
    let  mut r: Box<i32> = Box::new(0); // create a variable here , what is the lifetime of the variable
     *r = *a + *b ;
     return r; // return the reference from the function, which the variable itself is not at all available
}

fn calc(a:i32,b:i32)->(i32,i32,i32){
    (a+b,a-b,a*b)
}

// nm target/debug/smart_pointers | grep "COUNTER"
// 0000000100050238 d __ZN14smart_pointers7COUNTER17hebf9ed14d03c7ec2E
// referece of238 counter0x104248

// russt code  --> rustc -->|LLVM-> FN(IR) --> LLVM(BE)+ AS(Assembly) + LD(Linker)| --> Binary (2mb, no latency)
// C++ Code    --> c++c  -->|LLVM-> FN(IR) --> LLVM(BE)+ AS(Assembly) + LD(Linker)| --> Binary
// Zig --> zigc-->          |LLVM-> FN(IR) --> LLVM(BE)+ AS(Assembly) + LD(Linker)| --> Binary
// Go --> GoC --> Machine Code + Linker --> Binary (10mb, latency + GC)

// Javac --> Byte Code --> (JVM(JIT))--?> Run
// Javac --> GrallVM --> Bin

// rustc --> preprocessor, monomorfization, dead code elimination, optimizations --> IR