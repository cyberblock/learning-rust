// Soluciones · Vídeo 03
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=roQHqh4lqM4

fn main() {
    // 1. Inferencia: VS Code muestra i32, f64, bool y char
    let entero = 42;
    let decimal = 2.5;
    let booleano = false;
    let caracter = 'z';
    println!("{entero} {decimal} {booleano} {caracter}");

    // 2. Elegir el tipo más adecuado
    let edad: u8 = 35; // 0..=255 es suficiente y nunca es negativa
    let poblacion: u64 = 8_000_000_000; // no cabe en u32 (máx. ~4 300 millones). `_` separa miles
    let temperatura: i8 = -12; // necesita signo
    let pixel: u8 = 255; // exactamente el rango de u8
    println!("{edad} {poblacion} {temperatura} {pixel}");

    // 3. Rangos
    println!("i16: {} .. {}", i16::MIN, i16::MAX);
    println!("u16: {} .. {}", u16::MIN, u16::MAX);

    // 4. Tupla
    let alumno = ("Luis", 7.5, true);
    println!("Nombre: {}", alumno.0);
    println!("Nota: {}", alumno.1);
    println!("Aprobado: {}", alumno.2);

    // 5. Array
    let estaciones = ["primavera", "verano", "otoño", "invierno"];
    println!("La tercera estación es {}", estaciones[2]); // los índices empiezan en 0
}
