fn main() {
   
 
 let mut r1 = Rect::new(12.234,43.34);

 let a1 =r1.area();
 let p1 =r1.perimeter();

println!("Area of :{} Perimeter of Square:{}",a1,p1);


let mut r1_box = Box::new(r1);

let a1 = r1.area();

// let mut v = vec![10,20];
let mut a = 10;


}

#[derive(Debug,Copy,Clone)]
//struct Rectangle(f32,f32);// tuple structure
struct Rect{
    l:f32,
    b:f32,
    a:f64,
    p:f64,
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        // Self(s)
        return Rect { l: l, b:b,a:0.0,p:0.0 }
    }

    fn area(&mut self)->f64{
        self.a = (self.l * self.b) as f64;
        return self.a
    }

     fn perimeter(&self)->f64{
        // self.p= (2.0 * (self.l+self.b)) as f64;
        // return self.p;
        (2.0 * (self.l+self.b)) as f64;
    }
}







