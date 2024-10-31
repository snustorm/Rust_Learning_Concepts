

pub fn create_vector_and_display() {

    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);


    let v2 = vec![1,2,3];

    //一个安全的方法从vector里面取数
    match v.get(20) {
        Some(x) => println!("The value at index 2 is {}", x),
        None => println!("There is no such a element"),
    }
}