// Ejercicios · Vídeo 13
// Ejecuta con:  cargo run --example ejercicios_13
// Soluciones:   cargo run --example soluciones_13
//
// Resuelve cada uno con iter() + filter/map + collect (sin bucles for):
//
// 1. Filtra los números pares de 1..=10 y elévalos al cuadrado → [4, 16, 36, 64, 100]
//    (Ejercicio 14 de la guía).
//
// 2. Dado un Vec<&str> de nombres, obtén un Vec<String> con los que tengan más de 4 letras,
//    en mayúsculas.
//
// 3. Dado un vector de precios en euros, obtén los precios en dólares (x 1.08) de los que
//    cuesten menos de 50 €.
//
// 4. Reescribe este bucle en estilo funcional:
//    `let mut r = Vec::new(); for n in &v { if *n > 0 { r.push(n * 10); } }`

#![allow(unused_variables)]

fn pares_al_cuadrado() -> Vec<i32> {
    todo!()
}

fn nombres_largos(nombres: &[&str]) -> Vec<String> {
    todo!()
}

fn main() {
    println!("{:?}", pares_al_cuadrado());
    println!("{:?}", nombres_largos(&["Ana", "Roberto", "Luis", "Marta"]));
}
