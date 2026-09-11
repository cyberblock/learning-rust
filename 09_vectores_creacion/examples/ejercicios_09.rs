// Ejercicios · Vídeo 09
// Ejecuta con:  cargo run --example ejercicios_09
// Soluciones:   cargo run --example soluciones_09
//
// 1. Crea con vec! un vector con los nombres de 3 lenguajes de programación e imprímelo.
//
// 2. Crea un vector vacío de f64 y añade con push las temperaturas de una semana (7 valores).
//    Imprime el vector y cuántos elementos tiene (len).
//
// 3. Implementa `tabla_multiplicar(n: i32) -> Vec<i32>` que devuelva los 10 primeros
//    múltiplos de n. Ej.: tabla_multiplicar(3) → [3, 6, 9, ..., 30]
//    (Relacionado con el ejercicio 3 de la guía).
//
// 4. Crea un vector con `Vec::with_capacity(5)`, añade 6 elementos e imprime len y
//    capacity después de cada push. ¿Qué pasa al llegar al sexto?

#![allow(unused_variables)]

fn tabla_multiplicar(n: i32) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{:?}", tabla_multiplicar(3));
}
