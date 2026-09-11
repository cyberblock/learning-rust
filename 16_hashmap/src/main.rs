//! Vídeo 16 · HashMap I: guardar datos con clave y valor
//! Apuntes: ../apuntes/16-hashmap-I.md
//!
//! Vec     → lista ordenada, acceso por POSICIÓN
//! HashMap → sin orden, acceso por CLAVE única (DNI, id, usuario, token...)

use std::collections::HashMap; // a diferencia de Vec, hay que importarlo

fn main() {
    let mut notas = HashMap::new(); // HashMap<&str, f64> (inferido del primer insert)
    notas.insert("Ana", 8.5); //        insert(clave, valor)
    notas.insert("Luis", 4.2);
    notas.insert("Marta", 9.1);
    notas.insert("Pedro", 6.4);

    // Imprimir: implementa Debug, no Display
    println!("{:?}", notas);
    println!("{:#?}", notas); // uno por línea

    // get(clave) → Option<&V>
    println!("Marta: {:?}", notas.get("Marta")); // Some(9.1)
    println!("Juan:  {:?}", notas.get("Juan")); // None

    // Recorrer: dos variables (clave, valor). &notas para no consumir el mapa.
    // ⚠️ El orden NO está garantizado: puede cambiar entre ejecuciones.
    for (alumno, nota) in &notas {
        println!("{alumno} tiene una nota de {nota}");
    }

    // Extras útiles
    println!("\n== Extras ==");
    println!("¿Existe Luis? {}", notas.contains_key("Luis"));
    println!("Número de alumnos: {}", notas.len());
    if let Some(nota) = notas.remove("Pedro") {
        println!("Eliminado Pedro, que tenía {nota}");
    }
    let mut nombres: Vec<&&str> = notas.keys().collect();
    nombres.sort(); // para mostrarlos ordenados hay que ordenarlos aparte
    println!("Alumnos (ordenados): {:?}", nombres);

    // Una clave puede tener una lista como valor (agrupar por categorías)
    let mut cursos: HashMap<&str, Vec<&str>> = HashMap::new();
    cursos.insert("programación", vec!["Rust", "Java", "Python"]);
    cursos.insert("ofimática", vec!["Excel", "Word"]);
    println!("Programación: {:?}", cursos["programación"]);
}
