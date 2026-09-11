// Ejercicios · Vídeo 06
// Ejecuta con:  cargo run --example ejercicios_06
// Soluciones:   cargo run --example soluciones_06
//
// 1. Implementa `contar_vocales(texto: &String) -> usize` SIN mover la propiedad:
//    después de llamarla, `main` debe poder seguir usando el String.
//
// 2. Implementa `anadir_firma(texto: &mut String)` que añada " -- Jose" al final.
//
// 3. Este código no compila. Arréglalo cambiando SOLO el orden de las líneas:
//        let mut s = String::from("hola");
//        let r = &mut s;
//        println!("{}", s);
//        r.push('!');
//
// 4. ¿Cuántas referencias inmutables a la vez permite Rust? ¿Y mutables?
//    Compruébalo escribiendo código.

// En este vídeo se usa &String (referencia al String que hemos creado). En el vídeo 07
// verás que para solo leer texto es mejor recibir &str; Clippy lo sugiere (ptr_arg).
#![allow(clippy::ptr_arg)]
#![allow(unused_variables)]

fn contar_vocales(texto: &String) -> usize {
    todo!()
}

fn anadir_firma(texto: &mut String) {
    todo!()
}

fn main() {
    let mut mensaje = String::from("Aprendiendo borrowing");
    let vocales = contar_vocales(&mensaje);
    anadir_firma(&mut mensaje);
    println!("{mensaje} ({vocales} vocales)");
}
