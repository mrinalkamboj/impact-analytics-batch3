fn main() {
    let (a, b) = (10, 20);

    let r = a + b * (b / a) + (a * b) % 10 ; // operator precedence
    // 10 + 20  * 2 +(10 * 20) % 10;
    println!("r:{}", r);

    // expression , evaluated with then scope
    let r1 = {
        let (a, b) = (10, 20);
        let r = a + b * (b / a) + (a * b) % 10; // operator precedence
        r
    };
    println!("r1:{}", r1);

    let num = 100;

    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("odd");
    }

    let r2 = if num % 2 == 0 { "even" } else { "odd" };

    println!("r2:{}", r2);

    let _ = a+b; 

    let mut num=1;
    let mut sum = 0;

    // loop expression
    let r3 = loop{ // by design this is a unconditional loop
        sum = sum +num;
        num+=1;
        if num ==10{
            break (num,sum);
        }
    };

      println!("r3:{:#?}", r3); // debug print, becasuse r3 is a tuple which has not implemented Display trait 

    
    // let t= ("hello",'A',100,1231.1231,10i8,true);
}

// arthimetic
// compara
// logical
// bitwise
