// Soluciones · Vídeo 09

fn tabla_multiplicar(n: i32) -> Vec<i32> {
    let mut tabla = Vec::with_capacity(10); // sabemos que serán 10
    for i in 1..=10 {
        tabla.push(n * i);
    }
    tabla
}

fn main() {
    // 1.
    let lenguajes = vec!["Rust", "Python", "Java"];
    println!("{:?}", lenguajes);

    // 2.
    let mut temperaturas: Vec<f64> = Vec::new();
    temperaturas.push(21.5);
    temperaturas.push(23.0);
    temperaturas.push(19.8);
    temperaturas.push(25.1);
    temperaturas.push(24.4);
    temperaturas.push(18.9);
    temperaturas.push(20.0);
    println!("{:?} → {} días", temperaturas, temperaturas.len());

    // 3.
    println!("{:?}", tabla_multiplicar(3));

    // 4. Al superar la capacidad, Rust reserva más espacio (normalmente el doble)
    let mut v = Vec::with_capacity(5);
    for i in 1..=6 {
        v.push(i);
        println!("len = {}, cap = {}", v.len(), v.capacity());
    }
}
