//! Vídeo 10 · Vectores II: eliminar y acceder a elementos
//! Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=2g2yv2hv9uM
//! Apuntes: ../apuntes/10-vectores-II-eliminar-y-acceder.md
//!
//! Idea clave: Rust evita el `null`. Cuando algo puede no existir, devuelve Option<T>:
//!   Some(valor) → hay valor      None → no hay nada

fn main() {
    eliminar_con_pop();
    acceso_con_corchetes();
    acceso_con_get();
    manejar_option();
}

fn eliminar_con_pop() {
    println!("== pop() ==");
    let mut numeros = vec![25, 10, 20, 30];
    let ultimo = numeros.pop(); // Option<i32>
    println!("{:?} → quitado: {:?}", numeros, ultimo); // [25, 10, 20] → Some(30)

    let mut vacio: Vec<i32> = Vec::new();
    println!("pop() de un vector vacío: {:?}", vacio.pop()); // None, sin error
}

fn acceso_con_corchetes() {
    println!("\n== v[i] ==");
    let numeros = vec![25, 10, 20];
    println!("posición 2: {}", numeros[2]); // un i32 sí implementa Display → {}
    // println!("{}", numeros[3]);
    // 💥 panic: index out of bounds: the len is 3 but the index is 3
}

fn acceso_con_get() {
    println!("\n== v.get(i) ==");
    let numeros = vec![25, 10, 20];
    println!("get(1): {:?}", numeros.get(1)); // Some(10)
    println!("get(5): {:?}", numeros.get(5)); // None → el programa sigue
}

// Extra (se amplía en vídeos futuros): cómo USAR el valor de un Option
fn manejar_option() {
    println!("\n== Sacar el valor de un Option ==");
    let numeros = vec![25, 10, 20];

    // 1) match: cubre los dos casos
    match numeros.get(7) {
        Some(valor) => println!("Encontrado: {valor}"),
        None => println!("No existe la posición 7"),
    }

    // 2) if let: cuando solo te interesa el caso Some
    if let Some(primero) = numeros.first() {
        println!("El primero es {primero}");
    }

    // 3) unwrap_or: valor por defecto si es None
    let valor = numeros.get(99).unwrap_or(&0);
    println!("get(99) o 0 → {valor}");
}
