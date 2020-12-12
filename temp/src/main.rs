fn main() {
    println!("Hello, world!");
    integ(55.55);
    strin(String::from("Hi"))
}

fn integ<T: std::fmt::Display>(data:T){
    println!("{}",data);
}

fn strin(data:String) {
    println!("{}",data);
}