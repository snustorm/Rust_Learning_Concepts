use std::fmt::{Debug, Display};



//打印任意类型的值
pub fn print_value<T: Display>(value: T){

    println!("{}", value);
}

//通用结构体
pub struct Point<T: Into<f64>>{
    pub x: T,
    pub y: T,
}

impl<T: Into<f64> + Copy> Point<T> {

    pub fn distance_from_origin(&self) -> f64 {
        
        // The .into() method in Rust is part of the Into trait, 
        // which is used for type conversion. When you call value.into(),
        // it converts value from its current type to the type specified by the target context.
        let x = self.x.into();
        let y = self.y.into();
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

