// ============================================================
// MOTOR DE TRÁFICO AÉREO — Árbol AVL en Rust
// Materia: Estructuras de Datos / Programación II
// ============================================================

// --- ESTRUCTURAS BASE ---

#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // clave del árbol AVL
}

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

/// Rotación simple a la derecha (caso LL).
/// take() transfiere el ownership del hijo izquierdo
/// sin clonar ni violar las reglas del borrow checker.
fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

/// Rotación simple a la izquierda (caso RR).
fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if vuelo.altitud < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo));
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo));
    } else {
        return nodo; // altitud duplicada, no se inserta
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
}

// ============================================================
// FASE 2: BÚSQUEDA DE VUELO — O(log n), solo lectura
// ============================================================

// TODO: implementar buscar_vuelo
// fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> { ... }

// ============================================================
// FASE 3: ELIMINACIÓN (ATERRIZAJE)
// ============================================================

// TODO: implementar eliminar_vuelo
// fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> { ... }

// ============================================================
// FASE 4: ALERTA DE PROXIMIDAD
// ============================================================

// TODO: implementar vuelos_en_rango
// fn vuelos_en_rango(nodo: &Option<Box<Nodo>>, min: u32, max: u32) -> usize { ... }

// ============================================================
// MAIN
// ============================================================

fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    let datos = vec![
        ("AV123", 5000),
        ("UA456", 3000),
        ("IB101", 2000),
        ("AF999", 4000),
        ("TA222", 3500),
        ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    println!("Árbol cargado con {} vuelos.", 6);

    // Aquí el estudiante invoca sus funciones:
    // buscar_vuelo, eliminar_vuelo, vuelos_en_rango
}
