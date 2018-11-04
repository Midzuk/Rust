fn main() {
    let a = fact(10);
    println!("The answer is {}.", a);
}

fn fact(x: i32) -> i32 {
    match x {
        0 => 1,
        x => x * fact(x - 1),
    }
}
