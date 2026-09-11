//! Vídeo 09 · Vectores I: creación y uso
//! Apuntes: ../apuntes/09-vectores-I-creacion.md
//!
//! Vec<T>: colección de elementos del mismo tipo que puede CRECER en ejecución.
//! En el stack guarda una cabecera (ptr, len, cap); los elementos van en el heap.

fn main() {
    con_macro_vec();
    vacio_y_push();
    inferencia();
    capacidad();
}

fn con_macro_vec() {
    println!("== vec![...] con valores iniciales ==");
    let numeros = vec![1, 2, 3]; // Vec<i32>
    let nombres = vec!["Juan", "María"]; // Vec<&str>
    // Los vectores implementan Debug, no Display → {:?}
    println!("{:?} {:?}", numeros, nombres);
}

fn vacio_y_push() {
    println!("\n== Vec::new() + push ==");
    let mut numeros: Vec<i32> = Vec::new(); // mut: va a crecer
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    println!("{:?}", numeros);
}

fn inferencia() {
    println!("\n== Inferencia desde el primer push ==");
    let mut v = Vec::new(); // el tipo se deduce del push de abajo
    v.push(3.5); // → Vec<f64>
    println!("{:?}", v);

    // let vacio = Vec::new(); // ❌ type annotations needed: Vec<{unknown}>
    let vacio: Vec<String> = Vec::new(); // ✅ con el tipo explícito
    println!("vacío: {:?}, ¿está vacío? {}", vacio, vacio.is_empty());
}

fn capacidad() {
    println!("\n== len y capacity: las recolocaciones ==");
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(i);
        // Cuando len alcanza cap, Rust reserva un espacio mayor y MUEVE los elementos
        println!("len = {:2}, cap = {:2}", v.len(), v.capacity());
    }

    // Si sabes cuántos elementos habrá, reserva de antemano y evitas recolocaciones
    let mut reservado: Vec<i32> = Vec::with_capacity(500);
    reservado.push(25);
    println!(
        "with_capacity(500): len = {}, cap = {}",
        reservado.len(),
        reservado.capacity()
    );
    // La capacidad no es un límite: si metes 501 elementos, el vector sigue creciendo.
}
