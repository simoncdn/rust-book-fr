use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Veuillez entrer un index de tableau.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let index: usize = index
        .trim()
        .parse()
        .expect("L'index entré n'est pas un nombre");

    let element = a[index];

    println!(
        "La valeur de l'élément d'index {} est : {}",
        index, element
    );
}
