// Ejercicios · Vídeo 07
// Ejecuta con:  cargo run --example ejercicios_07
// Soluciones:   cargo run --example soluciones_07
//
// 1. Este código no compila. Explica por qué en un comentario y arréglalo:
//        let r;
//        {
//            let x = 5;
//            r = &x;
//        }
//        println!("{}", r);
//
// 2. Implementa `presentar(nombre: &str, edad: u32) -> String` que devuelva
//    "Me llamo <nombre> y tengo <edad> años". Pista: la macro format!
//
// 3. Implementa `mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str` que devuelva
//    la cadena más larga (ejercicio 12 de la guía). El `'a` indica que el resultado
//    vive tanto como las dos entradas.
//
// 4. Crea un String con tu frase favorita, añádele texto con push_str y conviértelo
//    a mayúsculas con to_uppercase().

#![allow(unused_variables)]

fn presentar(nombre: &str, edad: u32) -> String {
    todo!()
}

fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

fn main() {
    println!("{}", presentar("Ana", 30));
    println!("{}", mas_larga("Rust", "Ferris el cangrejo"));
}
