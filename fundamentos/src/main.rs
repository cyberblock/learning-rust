fn main() {

    // con let creamos variables inmutables, se puede usar mut para hacerlas mutables
    // las variables en Rust son de tipo estatico, es decir, su tipo no puede cambiar
    // estas pueden estar tipadas de forma explicita o implicita
    // let edad = 25;

    // let mensaje= "Hola";

    // let precio= 19.99;

    // let activa=true;


    // Binding en Rust
    let nomnbre = String::from("Jose");
    let dato= nomnbre;
    // explicacion de porque no se puede usar nomnbre despues del binding
    // En Rust, cuando asignas una variable a otra, la propiedad del valor se transfiere (mueve)
    // y la variable original ya no es válida. 
    //Esto se hace para garantizar la seguridad de la memoria y 
    // evitar problemas como el doble liberación de memoria.S
    println!("Nombre: {}", dato);
}
