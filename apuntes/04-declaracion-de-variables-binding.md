# 04 · Declaración de variables. Binding

- **Vídeo:** https://www.youtube.com/watch?v=ZgrkNenxjmw
- **Duración:** 19:39

## Resumen

Cómo declarar variables con `let`, la inferencia de tipos que muestra VS Code, la inmutabilidad por
defecto y el concepto clave del vídeo: **`let` crea un *binding***, un vínculo que convierte a la
variable en **dueña** del valor. Para explicarlo compara la memoria (stack/heap) en Java y en Rust.

## Tamaño de los tipos (duda de un alumno)

El número del tipo indica **cuántos bits ocupa en memoria** y, por tanto, **qué rango de valores admite**
(`i8` = 8 bits, `i16` = 16 bits…). Más bits significa más rango, pero también más memoria y operaciones
algo más costosas. Un `5` cabe en un `i8`; un número muy grande necesitará `i32` o `i64`.

## Declarar variables con `let`

```rust
fn main() {
    let edad = 30;          // VS Code muestra:  edad: i32
    let mensaje = "Hola";   // mensaje: &'static str
    let precio = 19.99;     // precio: f64
    let activa = true;      // activa: bool
}
```

- Sintaxis: `let nombre = valor;`
- Lo que aparece en gris tras el nombre (`: i32`) son **inlay hints / type hints** de rust-analyzer:
  **no forman parte del código** y no se compilan. Solo muestran el tipo inferido. El profesor recomienda
  dejarlos activados porque con vectores y objetos el tipo no siempre es evidente.
- Aunque `30` cabría en un `i8`, Rust infiere **`i32`** por defecto. Los decimales se infieren como **`f64`**.

## Las variables son inmutables por defecto

```rust
let edad = 30;
edad = 18; // ❌ error: cannot assign twice to immutable variable `edad`
```

Es una medida de seguridad. Para poder cambiar el valor hay que declararla con `mut`
(se ve en detalle en el vídeo 5):

```rust
let mut edad = 30;
edad = 18; // ✅
```

## Binding: por qué `let` es más que "reservar memoria"

En Java o Python, una variable es "un espacio en memoria donde se guarda un valor". En Rust, `let`
además crea un **binding**, un vínculo entre el nombre y el valor que hace a la variable **dueña** del valor
(el concepto de **ownership**, que se ve más adelante).

### Stack y heap

La RAM tiene varias zonas. Aquí importan dos:

- **Stack (pila):** donde vive el *nombre* de la variable, que apunta al valor.
- **Heap (montón):** donde se guarda el *valor* de los objetos (como un `String`).

### En Java: dos variables pueden apuntar al mismo valor

```java
String nombre = "Juan";
String dato = nombre;
System.out.println(nombre); // Juan  ✅
System.out.println(dato);   // Juan  ✅
```

`nombre` y `dato` están en el stack y ambos apuntan al mismo "Juan" del heap. Puedes usar las dos.

### En Rust: el valor cambia de dueño (*move*)

```rust
fn main() {
    let nombre = String::from("Juan"); // nombre es dueña de "Juan"
    let dato = nombre;                 // la propiedad se MUEVE a dato

    println!("{}", dato);   // ✅ Juan
    println!("{}", nombre); // ❌ error: borrow of moved value: `nombre`
}
```

- En memoria ocurre lo mismo que en Java: `dato` apunta al mismo valor del heap.
- **La diferencia:** tras `let dato = nombre;` **el dueño pasa a ser `dato`**. Solo puede haber
  **un dueño por valor**, así que `nombre` ya no puede acceder a él.
- El error aparece en el editor **antes incluso de compilar**. El compilador explica que
  *`String` no implementa el trait `Copy`* y sugiere usar `.clone()` si el coste es aceptable.
- ⚠️ Esto pasa con **objetos** como `String`. Con **tipos primitivos** (`i32`, `bool`…) se copian y el
  comportamiento en la práctica es el de otros lenguajes.
- ¿Por qué importa? Porque Rust trabaja a muy bajo nivel (punteros, direcciones de memoria) y hay que
  entender cómo gestiona la memoria.

## Reglas y convenciones de nombres

- **snake_case** para nombres de varias palabras: `nombre_persona`.
- Pueden contener letras, números y guiones bajos, pero **no empezar por número**.
- Se admiten caracteres Unicode (tildes, ñ), pero el profesor recomienda **evitarlos** y usar ASCII.

## Declarar sin inicializar y tipos explícitos

No es obligatorio inicializar en la misma línea (aunque es lo habitual):

```rust
let x;      // declarada
// ...
x = 30;     // inicializada más adelante
```

Se puede indicar el tipo explícitamente en lugar de dejar que lo infiera:

```rust
let x: i32 = 30;
```

Es útil o necesario con vectores, con el *turbofish* (`::<T>`), en proyectos grandes para mejorar la
legibilidad o al diseñar APIs y librerías para otros desarrolladores.

## Próximamente

Variables mutables, diferencia entre mutable y constante, cómo definir constantes y *shadowing* (sombreado).
