# Motor de Tráfico Aéreo — Árbol AVL en Rust

Parcial de Estructuras de Datos / Programación II  
Aeropuerto Internacional de Santa Ana

## Estructura del Proyecto

```
radar_avl/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs   ← todo el código del parcial
```

## Fases Implementadas

| Fase | Descripción | Estado |
|------|-------------|--------|
| Base | Inserción AVL + rotaciones | ✅ |
| 1 | Análisis de memoria (ownership, Box) | ✅ (doc) |
| 2 | `buscar_vuelo` — O(log n) | ⬜ |
| 3 | `eliminar_vuelo` + rebalanceo | ⬜ |
| 4 | `vuelos_en_rango` | ⬜ |

## Cómo compilar y ejecutar

```bash
cargo run
```

## Notas de Memoria (Fase 1)

- `Option::take()` extrae el valor del `Option` dejando `None`,
  transfiriendo ownership sin clonar.
- `Box<Nodo>` permite tamaño conocido en compile-time al almacenar
  el puntero (8 bytes) en lugar del nodo recursivo directamente.
