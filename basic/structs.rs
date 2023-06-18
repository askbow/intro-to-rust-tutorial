use std::fmt;

// Display cannot be derived
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
struct Point2d {
    x: i32,
    y: i32,
}

// need to implement method to display:
impl fmt::Display for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn main() {
   let point = Point2d {x: 32, y: 42};
   println!("Display: {}", point);
   println!("raw fields: {},{}", point.x, point.y);

   //inline struct:
   struct RGB(u8, u8, u8);
   let color = RGB(127, 127, 127);
   println!("Red: {}, Green: {}, Blue: {}", color.0, color.1, color.2);


   //tuple:
   let something: (u8, f32, f32) = (42, 1.1, -23.9);
   println!("Something: {}/{}/{}", something.0, something.1, something.2);
   // unpack tuple
   let (a,b,c) = something;
   println!("A:{}/B:{}/C:{}", a, b, c);

   // unpack tuple returned by method
   let things: [i32; 3] = [10, 20, 30];
   for (i, thing) in things.iter().enumerate() {
       println!("{}: {}", i, thing);
   }



    //enums
    enum Planet {
        Mercury,
        Venus,
        Mars,
        Pluto
    }
    let _planet = Planet::Mars;

    enum Action {
        MoveForward(usize),
        MoveBack(usize),
        Turn(Angle),
    }
    enum Angle {
        Radians(f32),
        Degrees(f32),
    }

}