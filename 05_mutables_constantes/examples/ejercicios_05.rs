// Ejercicios · Vídeo 05
// Ejecuta con:  cargo run --example ejercicios_05
// Soluciones:   cargo run --example soluciones_05
//
// 1. Crea un contador que empiece en 10 y réstale 3 dos veces. Imprime el resultado (4).
//
// 2. Declara una constante global IVA (21 %) y úsala en una función `precio_con_iva(precio)`
//    que devuelva el precio con IVA aplicado. Prueba con 100.0 → 121.0
//
// 3. Decide si cada dato debería ser `const`, `let` o `let mut` y declára los tres:
//    a) la velocidad de la luz (299 792 458 m/s)
//    b) la puntuación de un jugador que va subiendo
//    c) el nombre del usuario que ha iniciado sesión (se sabe en ejecución y no cambia)
//
// 4. ¿Por qué no compila `const HOY: i32 = obtener_dia();`? Escríbelo en un comentario.

#![allow(dead_code)]

const IVA: f64 = 0.0; // TODO: pon el valor correcto

fn precio_con_iva(precio: f64) -> f64 {
    todo!("aplica el IVA a {precio}")
}

fn main() {
    println!("{}", precio_con_iva(100.0));
}
