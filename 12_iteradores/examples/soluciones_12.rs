// Soluciones · Vídeo 12

fn main() {
    // 1.
    let mut v = vec![1, 2, 3];
    for x in v.iter() {
        print!("{x} ");
    }
    for x in v.iter_mut() {
        *x += 1;
    }
    for x in v.into_iter() {
        print!("{x} ");
    }
    println!();

    // 2.
    let mut nombres = vec![String::from("ana"), String::from("luis")];
    for nombre in nombres.iter_mut() {
        *nombre = nombre.to_uppercase();
    }
    println!("{:?}", nombres);

    // 3. La cuarta devuelve None: ya no quedan elementos
    let datos = [10, 20, 30];
    let mut it = datos.iter();
    for _ in 0..4 {
        println!("{:?}", it.next());
    }

    // 4. into_iter() toma la PROPIEDAD del vector (move): el iterador pasa a ser el dueño
    //    y la variable original queda invalidada.
}
