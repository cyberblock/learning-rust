# 16 · HashMap I: guardar datos con clave y valor

- **Vídeo:** https://www.youtube.com/watch?v=ThGAOWOZIM8
- **Duración:** 20:11

## Resumen

Se empieza una nueva colección del módulo 4: el **`HashMap`**, que guarda pares **clave → valor**.
El vídeo explica en qué se diferencia de un vector, cuándo usar cada uno y casos reales (caché,
configuración, agrupar por categorías, sesiones). Después, la práctica: importarlo, crearlo, `insert`,
imprimirlo (`{:?}` y `{:#?}`), `get` (que devuelve un `Option`) y recorrerlo con `for`.

## ¿Qué es un HashMap?

- Una colección de **pares clave-valor**. Existe en muchos lenguajes (en Java son los *maps*).
- En un **vector** cada valor va asociado a un **índice** (0, 1, 2…). En un **HashMap** va asociado a una
  **clave** identificativa, y se accede al valor **por su clave**.

Ejemplo (notas de alumnos):

| Clave | Valor |
|-------|-------|
| `"Ana"` | 8.5 |
| `"Luis"` | 4.2 |
| `"Marta"` | 9.1 |

- **El orden no importa**: lo importante es que cada clave tenga su valor.
- **Las claves deben ser únicas** (como un DNI). Si insertas una clave repetida, **se queda el último
  valor** y el anterior se elimina. Si tus datos van a tener claves repetidas, el HashMap no es la
  estructura adecuada.

## ¿Vector o HashMap?

| Usa un **vector** (`Vec`) cuando… | Usa un **HashMap** cuando… |
|-----------------------------------|----------------------------|
| Tienes una **lista ordenada** de elementos | El orden no importa |
| Accedes por **posición** | Buscas un dato **por una clave única** (DNI, ID, código, usuario…) |

## Casos de uso reales

- **Caché de datos:** si una aplicación consulta muchas veces la información de un mismo usuario, en
  lugar de ir cada vez a la base de datos se guarda temporalmente en un HashMap. Muy usado en servidores,
  aplicaciones web y APIs para mejorar el rendimiento.
- **Configuración de una aplicación:** `modo → producción`, `idioma → español`, `tema → oscuro`,
  `max_intentos → 3`.
- **Agrupar datos por categorías:** una clave puede tener una **lista** como valor:
  `programación → [Rust, Java, Python]`, `ofimática → [Excel, Word, PowerPoint]`, `diseño → [HTML, CSS]`.
- **Sesiones de usuario:** `token → id_usuario`, para comprobar rápidamente a quién pertenece un token
  (autenticación, sistemas de login).

## Crear e insertar

A diferencia de los vectores, **hay que importarlo**:

```rust
use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();   // mut: vamos a insertar elementos

    notas.insert("Ana", 8.5);         // insert(clave, valor)
    notas.insert("Luis", 4.2);
    notas.insert("Marta", 9.1);
    notas.insert("Pedro", 6.4);
    // Tipo inferido: HashMap<&str, f64>
}
```

- Mientras está vacío, el tipo es desconocido. Con el primer `insert`, Rust infiere `HashMap<&str, f64>`.
- Los *hints* de VS Code muestran `k:` (*key*, clave) y `v:` (*value*, valor).

## Imprimir

Un HashMap implementa `Debug` (no `Display`):

```rust
println!("{:?}", notas);   // todo en una línea
println!("{:#?}", notas);  // "bonito": un par por línea
```

## Obtener un valor por su clave: `get`

```rust
let nota_marta = notas.get("Marta");   // Option<&f64>
println!("{:?}", nota_marta);          // Some(9.1)

println!("{:?}", notas.get("Juan"));   // None (la clave no existe)
```

Igual que en los vectores, `get` devuelve un **`Option`**: `Some(valor)` si la clave existe y `None` si no.

## Recorrer un HashMap con `for`

```rust
for (alumno, nota) in &notas {   // alumno: &&str, nota: &f64
    println!("{} tiene una nota de {}", alumno, nota);
}
```

```
Ana tiene una nota de 8.5
Luis tiene una nota de 4.2
...
```

- Se usan **dos variables** entre paréntesis: una para la clave y otra para el valor.
- Se recorre con **`&notas`** (préstamo) para **no consumir** el HashMap (vídeo 11).

> **Nota (no aparece en el vídeo):** un `HashMap` **no garantiza ningún orden**. Al recorrerlo o
> imprimirlo, el orden puede cambiar entre ejecuciones. Si necesitas las claves ordenadas, existe
> `std::collections::BTreeMap`.

## Recapitulando

- Para qué sirven los HashMap y dónde se aplican.
- `use std::collections::HashMap;` + `HashMap::new()`.
- `insert(clave, valor)` para añadir elementos.
- `{:?}` / `{:#?}` para imprimirlo entero.
- `get(clave)` → `Option` para acceder a un elemento.
- `for (k, v) in &mapa` para recorrerlo.
