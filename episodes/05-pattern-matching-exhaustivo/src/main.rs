//! Episodio 05: El Poder del match en Rust: Pattern Matching Exhaustivo
//!
//! En este episodio demostramos:
//! 1. La exhaustividad obligatoria de `match` frente al frágil `switch` de C/Java.
//! 2. Patrones con valores múltiples (`|`) y rangos inclusivos (`..=`).
//! 3. Desestructuración de tuplas con comodín (`_`) y guardas condicionales (`match guard`).
//! 4. Vinculación de valores con el operador `@` (at-binding).

fn main() {
    println!("=== EPISODIO 05: PATTERN MATCHING EXHAUSTIVO CON MATCH ===\n");

    // ---------------------------------------------------------
    // 1. Exhaustividad Básica y Valores Múltiples
    // ---------------------------------------------------------
    // A diferencia de C o Java, en Rust un `match` DEBE ser exhaustivo.
    // El compilador garantiza que no exista ningún caso sin manejar.
    let codigo_estado: u16 = 404;

    let mensaje = match codigo_estado {
        200 => "OK: Petición exitosa",
        301 | 302 => "REDIRECCIÓN: Recurso movido",
        400..=499 => "ERROR DEL CLIENTE: Petición inválida o no encontrada",
        500..=599 => "ERROR DEL SERVIDOR: Fallo interno",
        _ => "CÓDIGO DESCONOCIDO", // Comodín (catch-all) que cubre cualquier otro caso
    };
    println!("1. Clasificación HTTP ({codigo_estado}): {mensaje}\n");

    // ---------------------------------------------------------
    // 2. Desestructuración de Tuplas y Guardas (Match Guards)
    // ---------------------------------------------------------
    // Podemos hacer pattern matching desglosando datos complejos
    // y añadiendo condiciones lógicas adicionales mediante `if`:
    let coordenadas = (10, -5);

    print!("2. Coordenadas {:?}: ", coordenadas);
    match coordenadas {
        (0, 0) => println!("En el origen exacto (0, 0)"),
        (x, 0) => println!("Sobre el eje X en posición {x}"),
        (0, y) => println!("Sobre el eje Y en posición {y}"),
        (x, y) if x > 0 && y > 0 => println!("En el Cuadrante 1 (+, +)"),
        (x, y) if x > 0 && y < 0 => println!("En el Cuadrante 4 (+, -)"),
        _ => println!("En otros cuadrantes"),
    }
    println!();

    // ---------------------------------------------------------
    // 3. Vinculación de Patrones con `@` (At-Binding)
    // ---------------------------------------------------------
    // El operador `@` nos permite probar que un valor coincide con un rango
    // Y AL MISMO TIEMPO vincular ese valor a una variable local para usarlo:
    let nivel_acceso: u8 = 85;

    let rol_asignado = match nivel_acceso {
        0..=30 => "Invitado: Permisos básicos de lectura",
        rango_moderador @ 31..=70 => {
            println!("3. Nivel de moderador detectado con @: nivel {rango_moderador}");
            "Moderador: Edición y gestión"
        }
        rango_admin @ 71..=100 => {
            println!("3. Nivel de administrador detectado con @: nivel {rango_admin}");
            "Administrador: Control total del sistema"
        }
        _ => "Nivel inválido",
    };
    println!("   Rol asignado: {rol_asignado}\n");

    println!("¡Demostración de pattern matching completada exitosamente!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clasificacion_http() {
        let code = 200;
        let res = match code {
            200 => "OK",
            _ => "OTHER",
        };
        assert_eq!(res, "OK");
    }

    #[test]
    fn test_match_guard() {
        let punto = (5, 5);
        let cuadrante = match punto {
            (x, y) if x > 0 && y > 0 => 1,
            _ => 0,
        };
        assert_eq!(cuadrante, 1);
    }
}
