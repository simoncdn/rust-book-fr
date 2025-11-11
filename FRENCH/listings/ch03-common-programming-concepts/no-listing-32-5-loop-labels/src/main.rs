fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("compteur = {}", count);
        let mut remaining = 10;

        loop {
            println!("restant = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Fin du compteur = {}", count);
}
