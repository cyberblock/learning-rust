# 05 · Variables inmutables vs mutables vs constantes

- **Vídeo:** https://www.youtube.com/watch?v=faj7967df4Q
- **Duración:** 19:35

## Resumen

El profesor compara los tres tipos de "contenedor" de Rust con una tabla y luego lo demuestra con código:
un contador (`mut`), una constante global usada desde varias funciones y el caso clave: una **variable
inmutable puede guardar un valor calculado en ejecución**, pero una **constante no**. Termina explicando
dónde se guardan las constantes: dentro del propio ejecutable.

## Tabla comparativa

| | `let` (inmutable) | `let mut` (mutable) | `const` |
|---|---|---|---|
| ¿Puede cambiar de valor en ejecución? | No | **Sí** | No |
| ¿Dónde vive? | Stack | Stack | En el **binario** del ejecutable |
| ¿Participa en el ownership? | Sí | Sí | **No** (no tiene dueño) |
| ¿Tipo obligatorio? | Opcional | Opcional | **Obligatorio** |
| ¿El valor debe ser constante (conocido al compilar)? | **No** (puede ser dinámico) | No | **Sí** |
| ¿Se evalúa en tiempo de compilación? | No | No | **Sí** |
| ¿Se puede declarar fuera de una función? | No | No | **Sí** (global) |

> **Idea clave:** inmutable ≠ constante. Una variable inmutable no cambia *una vez asignada*, pero su
> valor puede calcularse en tiempo de ejecución. Una constante tiene que conocerse **antes de compilar**.

## Inmutable vs mutable: un contador

```rust
fn main() {
    let contador = 0;
    contador = contador + 1; // ❌ cannot assign twice to immutable variable
    println!("Contador después de incrementar: {}", contador);
}
```

El programa ni siquiera compila. Solución: añadir `mut`.

```rust
fn main() {
    let mut contador = 0;
    contador = contador + 1;
    println!("Contador después de incrementar: {}", contador); // 1
}
```

## Constantes

```rust
const SALUDO: &str = "Hola"; // fuera de cualquier función: global

fn main() {
    // let saludo = "Hola";   // una variable NO puede declararse fuera de una función
    println!("{}", SALUDO);
    cualquiera();
}

fn cualquiera() {
    println!("Desde función cualquiera: {}", SALUDO); // también accesible aquí
}
```

Salida:

```
Hola
Desde función cualquiera: Hola
```

Reglas de las constantes:

- Palabra reservada **`const`** (no `cont`).
- Nombre en **MAYÚSCULAS** (`SALUDO`, `PI`); si no, el compilador avisa.
- **Tipo obligatorio**: `const SALUDO: &str = ...`, `const PI: f64 = 3.1416;`
- El valor debe ser **constante y conocido al compilar**.
- Pueden declararse **dentro o fuera** de funciones.
- El subrayado amarillo (*warning*) de una constante significa que está declarada pero **no se usa**.

## Diferencia clave: valores dinámicos

```rust
// const RESULTADO: i32 = obtener_numero(10);
// ❌ error: cannot call non-const fn `obtener_numero` in constants

fn main() {
    let resultado = obtener_numero(10); // ✅ una variable inmutable SÍ puede
    println!("{}", resultado);          // 10
}

fn obtener_numero(n: i32) -> i32 {
    n // devuelve n (sin punto y coma = valor de retorno)
}
```

- Una **constante no puede guardar el resultado de una función normal**, porque ese valor se calcula en
  ejecución y la constante debe conocerse al compilar.
- Una **variable inmutable sí puede**: se asigna una vez en ejecución y después ya no cambia.
- Adelanto de funciones: los parámetros llevan tipo (`n: i32`) y, si la función devuelve algo, hay que
  indicar el tipo de retorno con `-> i32`.

## ¿Dónde se guardan las constantes?

- Al compilar, Cargo genera el ejecutable en `target/debug/` (un `.exe` en Windows).
- Según el profesor, el valor de la constante acaba **dentro del propio ejecutable**, en una sección de
  datos de solo lectura: **`.rdata`** (*read-only data*) en Windows y **`.rodata`** en Linux.
- Esa sección se carga en memoria al ejecutar el programa y el sistema operativo la marca como de solo
  lectura. **No está en el stack, ni en el heap, ni tiene dueño.**

> **Nota (precisión técnica, no aparece en el vídeo):** en Rust, las `const` normalmente se **copian en
> línea (*inline*)** en cada sitio donde se usan. La que tiene una dirección fija en `.rodata` es la
> `static`. Además, una `const` declarada **dentro** de una función **sí respeta el scope**: solo se ve
> dentro de ella. Declarada en el nivel superior del archivo es visible en todo el módulo.
