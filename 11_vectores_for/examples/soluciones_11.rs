// Soluciones · Vídeo 11

fn contar_mayores(v: &[i32], limite: i32) -> usize {
    let mut contador = 0;
    for n in v {
        // v es un slice (&[i32]), así que n es &i32
        if *n > limite {
            contador += 1;
        }
    }
    contador
}

fn main() {
    // 1. Con &nombres el vector solo se presta
    let nombres = vec!["Ana", "Luis", "Marta"];
    for nombre in &nombres {
        println!("Hola, {nombre}");
    }
    println!("{:?}", nombres);

    // 2. Vector mut + &mut + desreferencia
    let mut precios = vec![10.0, 25.5, 7.99];
    for precio in &mut precios {
        *precio *= 1.10;
    }
    println!("{:.2?}", precios); // {:.2?} = Debug con 2 decimales

    // 3.
    let v = vec![3, 12, 7, 20, 1];
    println!("Mayores que 5: {}", contar_mayores(&v, 5)); // 3

    // 4.
    for i in 1..=10 {
        println!("7 x {i} = {}", 7 * i);
    }
}
