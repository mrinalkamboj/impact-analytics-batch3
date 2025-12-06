fn main() {
    let shape = shapes::shape_1::Shape::new_circle(100.34);

    let a1 = shape.area();

    let p1 = shape.perimeter();

    let shape = shapes::shape_1::Shape::default();

    let mut s1 = String::new();
    let s2 = String::new();

    // ptr of s1 and s2
    // 0x10 or 0x11

    println!("{:p} {:p}", s1.as_ptr(), s2.as_ptr());

    let ptr1 = s1.as_mut_ptr();

    //   unsafe{
    //     *ptr1 = 0b111;
    //   }

    let mut s1 = "hello world".to_string();

    let ptr1 = s1.as_mut_ptr();

    unsafe {
        *ptr1 = 65;
    }

    println!("{}", s1);
}

pub mod shapes{

    pub mod shape_1{
pub enum Shape {
    Circle(f32), // Touple structre
    Rectangle(f32, f32), // Touple structure // 8 bytes
                 // Cuboid(f32,f32,f32) // 12 bytes
}

// impl --> either the developer or the compiler implementes the code, genegerates the code

// self,&self ot &mut self
// Self --> The current Type.. the type on which the functions are being implemented
impl Shape {
   pub fn new_circle(r: f32) -> Self {
        return Shape::Circle(r);
    }

    pub fn new_rect(l: f32, b: f32) -> Shape {
        return Shape::Rectangle(l, b);
    }

    pub fn default() -> Shape {
        return Shape::Rectangle(1.1, 1.1);
    }

    pub fn area(&self) -> f32 {
        match self {
            Shape::Circle(r) => {
                return 3.14 * r * r;
            }
            Shape::Rectangle(l, b) => {
                return l * b;
            }
        }
    }
    pub fn perimeter(&self) -> f32 {
        match self {
            Shape::Circle(r) => {
                return 2.0 * 3.14 * r;
            }
            Shape::Rectangle(l, b) => {
                return 2.0 * (l + b);
            }
        }
    }
}
    }

}