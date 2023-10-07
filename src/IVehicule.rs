

    pub trait Shape  {
        fn area(&mut self) -> u32;
        fn can_hold(&self, other: &Rectangle) -> bool;
    }

    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }
    impl Shape for Rectangle {
         fn area(&mut self) -> u32 {
            
            self.width=2;
            return self.width * self.height;
        }
    
         fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }


