// Ejercicios · Vídeo 17
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=zBWrq4RYQNk
// Ejecuta con:  cargo run --example ejercicios_17
// Soluciones:   cargo run --example soluciones_17
//
// 1. Inserta "Ana" → 8.5 y después "Ana" → 6.0 guardando lo que devuelve el segundo insert.
//    Si había un valor anterior, imprime "Se ha sobrescrito la nota X".
//
// 2. Implementa `frecuencia_palabras(frase: &str) -> HashMap<String, usize>` que cuente
//    cuántas veces aparece cada palabra, sin distinguir mayúsculas/minúsculas
//    (ejercicio 13 de la guía). Pista: entry + or_insert + to_lowercase.
//
// 3. Implementa `contar_letras(texto: &str) -> HashMap<char, usize>` (ignora los espacios)
//    y muestra qué letra aparece más veces.
//
// 4. Tienes un inventario HashMap<&str, u32>. Añade 5 unidades de "tornillos":
//    si el producto no existe debe crearse con 5; si existe, sumarle 5.

#![allow(unused_variables)]

use std::collections::HashMap;

fn frecuencia_palabras(frase: &str) -> HashMap<String, usize> {
    todo!()
}

fn contar_letras(texto: &str) -> HashMap<char, usize> {
    todo!()
}

fn main() {
    println!("{:?}", frecuencia_palabras("Hola hola mundo HOLA rust"));
    println!("{:?}", contar_letras("programar en rust"));
}
