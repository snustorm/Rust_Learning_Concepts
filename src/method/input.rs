use std::io;



//从终端读取一个数，然后打印出来并判断奇数或者偶数
pub fn PrintNumber() {

    println!("Please enter your favorite number: ");
    
    let mut number_str = String::new();

    io::stdin().read_line(&mut number_str).unwrap();

    // The .trim() method in Rust is used to remove any leading and trailing whitespace from a string slice. 
    // This includes spaces, tabs, and newline characters.
    let number: i32 = number_str.trim().parse().unwrap();


    if number % 2 == 0 {
        println!("Your favorite number is {}. It is even", number);
    } else {
        println!("Your favorite number is {}. It is odd", number);
    }
}

//从终端读取一个半径(整数/浮点数)，然后计算圆的面积
pub fn calculate_area() {

    let mut radius_str = String::new();

    io::stdin().read_line(&mut radius_str).unwrap();

    let radius: f64 = radius_str.trim().parse().unwrap();

    let area = radius.powi(2) * std::f64::consts::PI;

    println!("The area of circle with radius {} is {}", radius, area);

}