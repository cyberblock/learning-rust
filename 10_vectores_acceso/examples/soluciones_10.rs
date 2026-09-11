// Soluciones · Vídeo 10

fn elemento_seguro(v: &[i32], i: usize) -> String {
    match v.get(i) {
        Some(valor) => format!("Posición {i}: {valor}"),
        None => format!("La posición {i} no existe"),
    }
}

fn main() {
    // 1.
    let mut colores = vec!["rojo", "verde", "azul", "amarillo", "negro"];
    println!("pop → {:?}", colores.pop());
    println!("pop → {:?}", colores.pop());
    println!("queda: {:?}", colores);

    // 2. Las dos últimas llamadas devuelven None (el vector ya está vacío)
    let mut v = vec![1, 2, 3, 4, 5];
    for _ in 0..7 {
        print!("{:?} ", v.pop());
    }
    println!();

    // 3.
    let v = vec![4, 8, 15];
    println!("{}", elemento_seguro(&v, 1));
    println!("{}", elemento_seguro(&v, 9));

    // 4. v.get(10) devuelve None y el programa continúa;
    //    v[10] provoca un panic (index out of bounds) y detiene el programa.
    println!("{:?}", v.get(10));
    // println!("{}", v[10]); // 💥
}
