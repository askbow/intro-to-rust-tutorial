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

struct Agent {
    x: f32,
    y: f32,
    heading: f32,
}

// traits are like abstract classes
trait Move {
    fn forward(&mut self, speed: f32) {}
    fn turn(&mut self, degrees: f32) {}
}

//method definitions are separate
impl Move for Agent {
    fn forward(&mut self, speed: f32) {
        self.x += self.heading.cos() * speed;
        self.y += self.heading.sin() * speed;
    }

    fn turn(&mut self, degrees: f32) {
        self.heading += degrees;
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
    let _planet = Planet::Mercury;
    let _planet = Planet::Venus;
    let _planet = Planet::Mars;
    let _planet = Planet::Pluto;

    enum Action {
        MoveForward(usize),
        MoveBack(usize),
        Turn(Angle),
    }
    enum Angle {
        Radians(f32),
        Degrees(f32),
    }


    // Collections
    let mut v: Vec<u32> = Vec::new();
    v.push(1_u32);
    v.push(2_u32);
    v.push(3_u32);
    //println!("vector: {}", v); // no default formatter for Vector
    //println!("vector pop: {}", v.pop()); // no default formatter for u32 or i32
    println!("vector len: {}, cap:{}", v.len(), v.capacity());

    enum UserInput {
        Number(f32),
        Message(Box<str>),
    }
    let mut v: Vec<UserInput> = Vec::new();
    v.push(UserInput::Number(123.45));
    v.push(
       UserInput::Message( "string??".into() )
    );
    v.pop();
    println!("vector: {}", v.capacity());

    let mut agent = Agent {x: 5.0, y: 32.0, heading: 12.0,};
    agent.turn(50.0);
}