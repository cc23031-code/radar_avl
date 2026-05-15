# Motor de Tráfico Aéreo — Árbol AVL en Rust

## Fase 1: Análisis de Seguridad y Propiedad

### 1. ¿Qué ocurre con la propiedad cuando se usa `Option::take()`?

`Option::take()` extrae el valor contenido en un `Option<T>`, dejando `None`
en su lugar y transfiriendo la **propiedad (ownership)** del valor al llamador.

En las rotaciones del árbol, esto es indispensable porque Rust no permite
tener dos dueños del mismo dato al mismo tiempo. Por ejemplo, en `rotar_derecha`:

```rust
let mut x = y.izquierdo.take(); // y.izquierdo queda en None
                                // x ahora es dueño del nodo
y.izquierdo = x.derecho.take(); // reasignamos punteros sin clonar nada
```

Sin `take()`, intentar mover `y.izquierdo` mientras `y` está prestado
causaría un error de compilación del *borrow checker*.

---

### 2. Prueba de escritorio — inserciones [5000, 3000, 2000, 4000, 3500, 6000]

Ver archivo `fase1_prueba_escritorio.svg` en este repositorio.

| Paso | Valor | Acción | Rotación |
|------|-------|--------|----------|
| 1 | 5000 | Raíz del árbol | Ninguna |
| 2 | 3000 | Hijo izquierdo de 5000 | Ninguna |
| 3 | 2000 | Hijo izq de 3000 · bal(5000)=2 | **Simple Derecha (LL)** — 3000 sube |
| 4 | 4000 | Hijo izq de 5000 | Ninguna |
| 5 | 3500 | bal(5000)=2, caso RL | **Doble (Der + Izq)** — 3500 sube |
| 6 | 6000 | Hijo derecho de 5000 | Ninguna |

---

### 3. ¿Por qué `Box<Nodo>` en lugar de `Nodo`?

Una estructura recursiva como `Nodo` contiene campos de tipo `Nodo` dentro
de sí misma. Rust necesita conocer el tamaño exacto de cada tipo en tiempo
de compilación, y `Nodo { izquierdo: Nodo, ... }` tendría tamaño **infinito**
(cada Nodo contiene otro Nodo que contiene otro Nodo...).

`Box<T>` resuelve esto: es un puntero de tamaño fijo (8 bytes en 64 bits)
que apunta a memoria en el *heap*. El compilador sabe que un `Box<Nodo>`
siempre ocupa exactamente el tamaño de un puntero, sin importar qué tan
profundo sea el árbol.
