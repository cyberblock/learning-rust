// Ejercicios · Vídeo 10
// Ejecuta con:  cargo run --example ejercicios_10
// Soluciones:   cargo run --example soluciones_10
//
// 1. Crea un vector con 5 colores. Elimina los dos últimos con pop() e imprime lo que
//    devuelve cada llamada y cómo queda el vector.
//
// 2. Llama a pop() 7 veces sobre un vector de 5 elementos. ¿Qué devuelven las dos últimas?
//
// 3. Implementa `elemento_seguro(v: &Vec<i32>, i: usize) -> String` que devuelva
//    "Posición i: valor" si existe o "La posición i no existe" si no. Usa get + match.
//
// 4. ¿Qué diferencia hay entre v[10] y v.get(10) si el vector tiene 3 elementos?
//    Pruébalo (el primero hará que el programa se detenga).

#![allow(unused_variables)]

fn elemento_seguro(v: &[i32], i: usize) -> String {
    todo!()
}

fn main() {
    let v = vec![4, 8, 15];
    println!("{}", elemento_seguro(&v, 1));
    println!("{}", elemento_seguro(&v, 9));
}
