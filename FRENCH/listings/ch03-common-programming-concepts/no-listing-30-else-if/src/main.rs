fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Le nombre est divisible par 4");
    } else if number % 3 == 0 {
        println!("Le nombre est divisible par 3");
    } else if number % 2 == 0 {
        println!("Le nombre est divisible par 2");
    } else {
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }
}
