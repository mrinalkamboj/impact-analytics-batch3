fn main() {
   
   let s1 = Square::new(10.23);

   let a1 = s1.area();
   let p1 = s1.perimeter();

   let a1 =Square::area(&s1);

   println!("Area of :{} Perimeter of Square:{}",a1,p1);

}

#[derive(Debug,Copy,Clone)]
struct Square(f32);

impl Square{
    fn new(s:f32)->Self{
        // Self(s)
        return Square(s);
    }

    fn area(&self)->f64{
        return (self.0 * self.0) as f64;
    }

     fn perimeter(&self)->f64{
        return (4.0 * self.0) as f64;
    }
}







