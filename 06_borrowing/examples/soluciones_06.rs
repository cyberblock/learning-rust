// Soluciones · Vídeo 06
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=bPVWshg1tC4

// En este vídeo se usa &String (referencia al String que hemos creado). En el vídeo 07
// verás que para solo leer texto es mejor recibir &str; Clippy lo sugiere (ptr_arg).
#![allow(clippy::ptr_arg)]

// 1. Recibe una referencia (&): la función toma prestado el String, no se queda con él
fn contar_vocales(texto: &String) -> usize {
    texto
        .chars()
        .filter(|c| "aeiouáéíóúAEIOUÁÉÍÓÚ".contains(*c))
        .count()
}

// 2. Referencia mutable: puede modificar el String del que llama
fn anadir_firma(texto: &mut String) {
    texto.push_str(" -- Jose");
}

fn main() {
    let mut mensaje = String::from("Aprendiendo borrowing");
    let vocales = contar_vocales(&mensaje);
    anadir_firma(&mut mensaje);
    println!("{mensaje} ({vocales} vocales)");

    // 3. El préstamo mutable debe terminar (último uso) antes de volver a usar `s`
    let mut s = String::from("hola");
    let r = &mut s;
    r.push('!');
    println!("{}", s); // hola!

    // 4. Inmutables: todas las que quieras
    let a = &s;
    let b = &s;
    let c = &s;
    println!("{a} {b} {c}");
    // Mutables: solo UNA a la vez
    let m1 = &mut s;
    // let m2 = &mut s; // ❌ cannot borrow `s` as mutable more than once at a time
    m1.push('?');
    println!("{s}");
}
