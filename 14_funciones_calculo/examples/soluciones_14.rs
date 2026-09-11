// Soluciones · Vídeo 14

fn main() {
    let edades = vec![15, 22, 17, 34, 41, 19, 12];

    let suma: i32 = edades.iter().sum();
    println!("1. Suma: {suma}");

    println!("2. Primera > 30: {:?}", edades.iter().find(|e| **e > 30));

    println!(
        "3. Menores de edad: {}",
        edades.iter().filter(|e| **e < 18).count()
    );

    println!("4. ¿Alguien > 40? {}", edades.iter().any(|e| *e > 40));

    println!("5. ¿Todos > 10? {}", edades.iter().all(|e| *e > 10));

    // 6. Sacar el valor del Option
    if let Some(maxima) = edades.iter().max() {
        println!("6. Máxima: {maxima}");
    }
    let minima = edades.iter().min().unwrap(); // seguro: el vector no está vacío
    println!("6. Mínima: {minima}");
}
