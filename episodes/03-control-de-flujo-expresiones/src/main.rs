//! Episodio 03: Control de Flujo Expresivo en Rust: Todo es una Expresión
//!
//! En este episodio demostramos:
//! 1. La diferencia entre sentencias (`statement`) y expresiones (`expression`).
//! 2. `if` utilizado como expresión para asignar valores directamente.
//! 3. Bucles infinitos `loop` que devuelven resultados mediante `break valor`.
//! 4. Bucles condicionales `while`.
//! 5. Bucles `for` idiomáticos utilizando rangos numéricos e iteradores.

fn main() {
    println!("=== EPISODIO 03: CONTROL DE FLUJO EXPRESIVO EN RUST ===\n");

    // ---------------------------------------------------------
    // 1. Sentencias vs. Expresiones
    // ---------------------------------------------------------
    // En Rust, casi todo devuelve un valor (es una expresión).
    // Si una línea termina con punto y coma (;), se convierte en sentencia y devuelve `()`.
    // Si dejamos la última línea sin punto y coma en un bloque, ese bloque EVALÚA a ese valor.
    let calculo_bloque = {
        let x = 10;
        let y = 20;
        x + y // Sin punto y coma: devuelve 30
    };
    println!("1. Bloque evaluado como expresión: {calculo_bloque}");

    // ---------------------------------------------------------
    // 2. `if` como Expresión (Reemplazo del operador ternario)
    // ---------------------------------------------------------
    // En Rust no existe el operador ternario `condicion ? a : b`.
    // No hace falta, porque `if` es una expresión de primera clase:
    let es_usuario_premium = true;
    let descuento_porcentaje = if es_usuario_premium { 25 } else { 5 };
    println!("2. Asignación directa con 'if': Descuento del {descuento_porcentaje}%\n");

    // ---------------------------------------------------------
    // 3. `loop` con Retorno de Valor
    // ---------------------------------------------------------
    // `loop` crea un bucle infinito controlado por el programador.
    // Una característica única de Rust es que podemos pasar un valor a `break`,
    // y ese valor se convierte en el resultado del bucle:
    let mut intentos = 0;
    let token_generado = loop {
        intentos += 1;
        if intentos == 3 {
            // Pasamos el resultado directamente al romper el bucle
            break intentos * 1000;
        }
    };
    println!("3. Bucle 'loop' con retorno de valor: Token = {token_generado} (en {intentos} intentos)\n");

    // ---------------------------------------------------------
    // 4. Bucle `while`
    // ---------------------------------------------------------
    // Ejecuta el cuerpo mientras la condición booleana sea verdadera:
    let mut cuenta_regresiva = 3;
    print!("4. Bucle 'while': ");
    while cuenta_regresiva > 0 {
        print!("{cuenta_regresiva}... ");
        cuenta_regresiva -= 1;
    }
    println!("¡Despegue!\n");

    // ---------------------------------------------------------
    // 5. Bucle `for` Idiomático con Rangos
    // ---------------------------------------------------------
    // En Rust rara vez se usa el clásico bucle de C `for (int i=0; i<10; i++)`.
    // La forma idiomática y segura es iterar sobre colecciones o rangos:
    // `0..4` es un rango exclusivo (0, 1, 2, 3).
    // `0..=4` es un rango inclusivo (0, 1, 2, 3, 4).
    print!("5. Bucle 'for' con rango exclusivo (0..4): ");
    for numero in 0..4 {
        print!("{numero} ");
    }
    println!();

    print!("   Bucle 'for' invertido con .rev() (1..=3): ");
    for item in (1..=3).rev() {
        print!("{item} ");
    }
    println!("\n\n¡Control de flujo expresivo completado exitosamente!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_expression() {
        let habilitado = true;
        let estado = if habilitado { "ON" } else { "OFF" };
        assert_eq!(estado, "ON");
    }

    #[test]
    fn test_loop_break_value() {
        let mut count = 0;
        let res = loop {
            count += 1;
            if count == 5 {
                break count * 2;
            }
        };
        assert_eq!(res, 10);
    }
}
