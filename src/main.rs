
mod IVehicule;
use IVehicule::{Rectangle,Shape};
use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
 let mut r = Rectangle{
    height:10,
    width:12,
 };


 let a = r.area();
 println!("{a}");

//  let br:Box<dyn Shape> = Box::new(r);
//  let a = br.area();
//  println!("{a}");

 let brr:&mut  dyn   Shape = &mut r as &mut dyn Shape;
  let a = brr.area();
  println!("brr {a}");

}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
