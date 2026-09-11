// Soluciones · Vídeo 05
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=faj7967df4Q

const IVA: f64 = 0.21;
const VELOCIDAD_LUZ: u64 = 299_792_458; // 3a: fija y conocida al compilar → const

fn precio_con_iva(precio: f64) -> f64 {
    precio * (1.0 + IVA)
}

fn main() {
    // 1.
    let mut contador = 10;
    contador -= 3;
    contador -= 3;
    println!("contador: {contador}");

    // 2.
    println!("100 € con IVA: {} €", precio_con_iva(100.0));

    // 3b: cambia durante la ejecución → let mut
    let mut puntuacion = 0;
    puntuacion += 50;
    // 3c: se conoce en ejecución pero no cambia → let (inmutable)
    let usuario = String::from("jose");
    println!("{VELOCIDAD_LUZ} m/s · {usuario} tiene {puntuacion} puntos");

    // 4. No compila porque una constante debe evaluarse en tiempo de COMPILACIÓN,
    //    y una función normal solo se ejecuta en tiempo de EJECUCIÓN.
    //    Solución: `let hoy = obtener_dia();` (o que la función sea `const fn`).
}
