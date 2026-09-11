//! Vídeo 08 · Slices
//! Apuntes: ../apuntes/08-slices.md
//!
//! Un slice es una REFERENCIA a una parte de una colección que ya existe:
//! no copia datos y no es dueño. Internamente guarda puntero + longitud.
//! Sintaxis: &coleccion[inicio..fin]  (inicio incluido, fin excluido)

fn main() {
    slice_de_string();
    slice_de_array();
    variantes_de_rango();
    display_vs_debug();
}

fn slice_de_string() {
    println!("== Slice de un String ==");
    let s = String::from("Hola alumnos");
    let saludo = &s[0..4]; // posiciones 0,1,2,3 → "Hola"
    let resto = &s[5..12]; // "alumnos"
    println!("{saludo} | {resto}");
    // `s` sigue siendo la dueña; `saludo` apunta directamente a su parte del heap.
}

fn slice_de_array() {
    println!("\n== Slice de un array ==");
    let numeros = [1, 2, 3, 4, 5]; // [i32; 5]
    let parte = &numeros[1..4]; // &[i32] → [2, 3, 4]
    println!("parte: {:?} (longitud {})", parte, parte.len());
}

fn variantes_de_rango() {
    println!("\n== Variantes de rango ==");
    let numeros = [10, 20, 30, 40, 50];
    println!("[..2]  -> {:?}", &numeros[..2]); // desde el principio
    println!("[3..]  -> {:?}", &numeros[3..]); // hasta el final
    println!("[..]   -> {:?}", &numeros[..]); // todo
    println!("[1..=3]-> {:?}", &numeros[1..=3]); // ..= incluye el final

    // let fuera = &numeros[2..10]; // 💥 panic en ejecución: range end index 10 out of range
}

fn display_vs_debug() {
    println!("\n== Display {{}} vs Debug {{:?}} ==");
    let numeros = [1, 2, 3];
    let parte = &numeros[0..2];
    // println!("{}", parte);
    // ❌ `[i32]` doesn't implement `std::fmt::Display`
    // Los arrays/slices no saben mostrarse "bonito"; sí implementan el trait Debug:
    println!("{:?}", parte);
    println!("{:#?}", parte); // Debug con saltos de línea
}
