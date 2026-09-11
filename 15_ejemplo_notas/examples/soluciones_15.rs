// Soluciones · Vídeo 15
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=dq-ifVDamss

fn calificacion(nota: f64) -> &'static str {
    if nota < 5.0 {
        "Suspenso"
    } else if nota < 7.0 {
        "Aprobado"
    } else if nota < 9.0 {
        "Notable"
    } else {
        "Sobresaliente"
    }
}

fn main() {
    let notas: Vec<f64> = vec![7.5, 4.1, 8.2, 9.1, 3.8, 6.4];

    // 1. copied() convierte &f64 → f64, así el resultado es Vec<f64>
    let suspensos: Vec<f64> = notas.iter().copied().filter(|n| *n < 5.0).collect();
    println!("1. Suspensos: {:?}", suspensos);

    // 2.
    let minima = notas
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("2. Mínima: {minima}");

    // 3.
    let sobresalientes = notas.iter().filter(|n| **n >= 9.0).count();
    println!("3. Sobresalientes: {sobresalientes}");

    // 4.
    let calificaciones: Vec<&str> = notas.iter().map(|n| calificacion(*n)).collect();
    println!("4. {:?}", calificaciones);

    // 5.
    let media = notas.iter().sum::<f64>() / notas.len() as f64;
    println!("5. Media: {:.1}", media);
}
