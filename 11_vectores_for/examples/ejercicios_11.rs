// Ejercicios · Vídeo 11
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=VfS3EKlMvzQ
// Ejecuta con:  cargo run --example ejercicios_11
// Soluciones:   cargo run --example soluciones_11
//
// 1. Recorre un vector de nombres con for, imprime "Hola, <nombre>" para cada uno y,
//    DESPUÉS del bucle, imprime el vector completo (tiene que compilar).
//
// 2. Dado un vector de precios [10.0, 25.5, 7.99], súbelos todos un 10 % dentro de
//    un bucle for, de forma que el vector quede modificado.
//
// 3. Implementa `contar_mayores(v: &Vec<i32>, limite: i32) -> usize` que recorra el
//    vector con for y cuente cuántos elementos son mayores que `limite`.
//
// 4. Imprime la tabla de multiplicar del 7 (7 x 1 = 7 ... 7 x 10 = 70) con un for
//    sobre el rango 1..=10.

#![allow(unused_variables)]

fn contar_mayores(v: &[i32], limite: i32) -> usize {
    todo!()
}

fn main() {
    let v = vec![3, 12, 7, 20, 1];
    println!("{}", contar_mayores(&v, 5));
}
