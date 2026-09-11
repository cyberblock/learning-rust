// Ejercicios · Vídeo 16
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=ThGAOWOZIM8
// Ejecuta con:  cargo run --example ejercicios_16
// Soluciones:   cargo run --example soluciones_16
//
// 1. Crea un HashMap con la configuración de una app: modo → "producción",
//    idioma → "español", tema → "oscuro". Imprímelo con {:#?}.
//
// 2. Con un HashMap de precios (producto → f64), implementa
//    `precio_de(precios: &HashMap<&str, f64>, producto: &str) -> String` que devuelva
//    "El <producto> cuesta <precio> €" o "No tenemos <producto>".
//
// 3. Recorre un HashMap de alumnos → nota e imprime SOLO los aprobados.
//
// 4. Simula una caja registradora (ejercicio extra de la guía): un HashMap<String, f32>
//    con los productos del carrito y su precio. Calcula el total recorriendo el mapa.

#![allow(unused_variables)]

use std::collections::HashMap;

fn precio_de(precios: &HashMap<&str, f64>, producto: &str) -> String {
    todo!()
}

fn main() {
    let mut precios = HashMap::new();
    precios.insert("pan", 1.20);
    precios.insert("leche", 0.95);
    println!("{}", precio_de(&precios, "pan"));
    println!("{}", precio_de(&precios, "queso"));
}
