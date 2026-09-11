// Soluciones · Vídeo 08

fn primera_palabra(frase: &str) -> &str {
    match frase.find(' ') {
        Some(posicion) => &frase[..posicion],
        None => frase,
    }
}

fn suma(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros {
        total += n;
    }
    total
}

fn main() {
    // 1.
    let frase = String::from("Aprendiendo Rust");
    println!("{} / {}", &frase[0..11], &frase[12..]);

    // 2.
    println!("{}", primera_palabra("Hola mundo cruel")); // Hola
    println!("{}", primera_palabra("Rust")); // Rust

    // 3.
    let datos = [3, 6, 9, 12, 15, 18];
    println!("{:?}", &datos[..3]);
    println!("{:?}", &datos[datos.len() - 2..]);
    println!("{:?}", &datos[1..5]);

    // 4. &[i32] acepta un array entero o cualquier trozo de él
    println!("Suma total: {}", suma(&datos));
    println!("Suma parcial: {}", suma(&datos[..3]));
}
