fn main() {
    let x = plus_one(5);

    println!("La valeur de x est : {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
