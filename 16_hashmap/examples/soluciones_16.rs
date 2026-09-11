// Soluciones · Vídeo 16

use std::collections::HashMap;

fn precio_de(precios: &HashMap<&str, f64>, producto: &str) -> String {
    match precios.get(producto) {
        Some(precio) => format!("El {producto} cuesta {precio} €"),
        None => format!("No tenemos {producto}"),
    }
}

fn main() {
    // 1.
    let mut config = HashMap::new();
    config.insert("modo", "producción");
    config.insert("idioma", "español");
    config.insert("tema", "oscuro");
    println!("{:#?}", config);

    // 2.
    let mut precios = HashMap::new();
    precios.insert("pan", 1.20);
    precios.insert("leche", 0.95);
    println!("{}", precio_de(&precios, "pan"));
    println!("{}", precio_de(&precios, "queso"));

    // 3.
    let mut notas = HashMap::new();
    notas.insert("Ana", 8.5);
    notas.insert("Luis", 4.2);
    notas.insert("Marta", 9.1);
    for (alumno, nota) in &notas {
        if *nota >= 5.0 {
            println!("{alumno} aprobado con {nota}");
        }
    }

    // 4.
    let mut carrito: HashMap<String, f32> = HashMap::new();
    carrito.insert(String::from("manzanas"), 2.50);
    carrito.insert(String::from("café"), 4.75);
    carrito.insert(String::from("arroz"), 1.10);
    let mut total = 0.0;
    for (producto, precio) in &carrito {
        println!("{producto:10} {precio:>6.2} €");
        total += precio;
    }
    println!("TOTAL      {total:>6.2} €");
    // Versión funcional: let total: f32 = carrito.values().sum();
}
