fn main() {
    let n = 10;
    let mut a = 0;
    let mut b = 1;

    for i in 0..10 {
        println!("{}", a);
        let t = a + b;
        a = b;
        b = t;
    }

    let mut n = 1;
    let mut a = 0;
    let mut b = 1;

    while n <= 10 {
        println!("{}", a);
        let t = a + b;
        a = b;
        b = t;
        n += 1;
    }

    let mut n = 1;
    let mut a = 0;
    let mut b = 1;

    loop {
        if n > 10 {
            break;
        }
        println!("{}", a);
        let t = a + b;
        a = b;
        b = t;
        n += 1;
    }

    // let (mut i, mut j) = (0, 0);

    // let mut flag = false;

    // loop {
    //     if flag {
    //         break;
    //     }
    //     if i>=10{
    //         break;
    //     }
    //     i += 1;
    //     loop {
    //         j += 1;
    //         println!("i={} j={}", i, j);
    //         if i == 5 {
    //             flag = true;
    //             break;
    //         }
    //     }
    // }

    'outer: for i in 1..=10{
        for j in 1..=10{
            println!("{} -> {}",i,j);
            if j == 5{
                break 'outer;
            }
        }
    }
}

// There are three types of loops
// for
// while
// loop
