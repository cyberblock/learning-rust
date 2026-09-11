// Ejercicios · Vídeo 08
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=CH2fpLz39Dg
// Ejecuta con:  cargo run --example ejercicios_08
// Soluciones:   cargo run --example soluciones_08
//
// 1. Dado `let frase = String::from("Aprendiendo Rust");`, crea slices con
//    "Aprendiendo" y "Rust" e imprímelos.
//
// 2. Implementa `primera_palabra(frase: &str) -> &str` que devuelva la primera palabra
//    (todo lo que hay antes del primer espacio). Si no hay espacios, devuelve la frase entera.
//    (Ejercicio 6 de la guía). Pista: recorre `frase.char_indices()` o usa `frase.find(' ')`.
//
// 3. Dado el array [3, 6, 9, 12, 15, 18], imprime:
//    a) los tres primeros   b) los dos últimos   c) los del medio (6, 9, 12, 15)
//
// 4. Implementa `suma(numeros: &[i32]) -> i32` que sume un slice de enteros y
//    úsala con el array completo y con una parte de él.

#![allow(unused_variables)]

fn primera_palabra(frase: &str) -> &str {
    todo!()
}

fn suma(numeros: &[i32]) -> i32 {
    todo!()
}

fn main() {
    println!("{}", primera_palabra("Hola mundo cruel"));
    let datos = [3, 6, 9, 12, 15, 18];
    println!("{}", suma(&datos));
}
