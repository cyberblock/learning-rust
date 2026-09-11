// Soluciones · Vídeo 13

fn pares_al_cuadrado() -> Vec<i32> {
    // Los rangos también son iteradores: no hace falta crear un vector antes
    (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).collect()
}

fn nombres_largos(nombres: &[&str]) -> Vec<String> {
    nombres
        .iter()
        .filter(|nombre| nombre.len() > 4)
        .map(|nombre| nombre.to_uppercase())
        .collect()
}

fn main() {
    // 1.
    println!("{:?}", pares_al_cuadrado());

    // 2.
    println!("{:?}", nombres_largos(&["Ana", "Roberto", "Luis", "Marta"]));

    // 3.
    let precios = vec![20.0, 75.5, 49.99, 120.0];
    let en_dolares: Vec<f64> = precios
        .iter()
        .filter(|p| **p < 50.0)
        .map(|p| p * 1.08)
        .collect();
    println!("{:.2?}", en_dolares); // {:.2?} = Debug con 2 decimales

    // 4.
    let v = vec![-2, 3, 0, 7];
    let r: Vec<i32> = v.iter().filter(|n| **n > 0).map(|n| n * 10).collect();
    println!("{:?}", r); // [30, 70]
}
