# 12 · Vectores IV: iteradores explícitos

- **Vídeo:** https://www.youtube.com/watch?v=WYiWUfgwn5Y
- **Duración:** 11:20

## Resumen

Todo `for` usa un iterador por detrás, pero también se puede usar **explícitamente** con tres métodos
que equivalen a las tres formas del vídeo 11: `into_iter()`, `iter()` e `iter_mut()`. La ventaja de los
iteradores es que dan acceso a métodos para **transformar, filtrar y encadenar** datos (`map`, `filter`,
`collect`, `sum`…), que se ven en los siguientes vídeos.

## Equivalencias

### Préstamo inmutable: `&v` ≡ `v.iter()`

```rust
let numeros = vec![1, 2, 3];

for n in numeros.iter() {    // igual que: for n in &numeros   → n: &i32
    println!("{}", n);
}

println!("{:?}", numeros);   // ✅ el vector sigue disponible
```

### Préstamo mutable: `&mut v` ≡ `v.iter_mut()`

```rust
let mut numeros = vec![1, 2, 3];   // el vector sigue teniendo que ser mut

for n in numeros.iter_mut() {      // igual que: for n in &mut numeros  → n: &mut i32
    *n += 1;
}

println!("{:?}", numeros);         // [2, 3, 4]
```

### Mover el vector: `v` ≡ `v.into_iter()`

```rust
let numeros = vec![1, 2, 3];

for n in numeros.into_iter() {     // igual que: for n in numeros
    println!("{}", n);
}

// println!("{:?}", numeros);      // ❌ la propiedad se movió al iterador
```

## Tabla resumen

| | Mueve el vector (cambia el ownership) | Préstamo inmutable | Préstamo mutable |
|---|---|---|---|
| **Sin iterador** | `for n in numeros` | `for n in &numeros` | `for n in &mut numeros` |
| **Con iterador** | `numeros.into_iter()` | `numeros.iter()` | `numeros.iter_mut()` |
| Tipo de `n` | `i32` | `&i32` | `&mut i32` |

## ¿Qué ventaja tienen los iteradores?

1. **Legibilidad y simplicidad.** Con ejemplos tan sencillos apenas se nota ("me cuesta lo mismo escribir
   una cosa que otra"), pero con operaciones más complejas la versión sin iteradores se complica mucho.
2. **Procesar datos, no solo recorrerlos.** Permiten transformar datos, encadenar operaciones y escribir
   código más eficiente y expresivo.

**Usa iteradores cuando quieras:**

- **Transformar** datos → `map`
- **Filtrar** datos → `filter`
- **Encadenar** operaciones
- Escribir código más **legible**

Si solo vas a recorrer un vector sencillo para ver sus elementos, puedes usar cualquiera de las dos formas.

## Adelanto del próximo vídeo

- **`map`** y **`filter`** son métodos disponibles en cualquier tipo que implemente el trait `Iterator`.
- **`collect`** también es muy importante (técnicamente se apoya en el trait `FromIterator`).
- También está **`sum`**, entre otros.
- Todos estos métodos **solo están disponibles si usas iteradores**.
