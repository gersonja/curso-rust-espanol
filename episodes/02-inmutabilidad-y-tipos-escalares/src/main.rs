//! Episodio 02: Inmutabilidad por Defecto y Tipos Escalares en Rust
//!
//! En este episodio demostramos:
//! 1. Inmutabilidad por defecto (`let`) vs Mutabilidad explícita (`let mut`).
//! 2. Shadowing (sombreado de variables): reutilización de nombres y cambio de tipo.
//! 3. Constantes (`const`): valores evaluados en tiempo de compilación.
//! 4. Tipos escalares: enteros con y sin signo, flotantes, booleanos y caracteres Unicode (4 bytes).

/// Constante global evaluada en tiempo de compilación.
/// Debe tener una anotación de tipo explícita y se nombra en SCREAMING_SNAKE_CASE.
const MAX_SERVER_CONNECTIONS: u32 = 100_000;

fn main() {
    println!("=== EPISODIO 02: INMUTABILIDAD Y TIPOS ESCALARES ===\n");

    // ---------------------------------------------------------
    // 1. Inmutabilidad por Defecto vs. Mutabilidad Explícita
    // ---------------------------------------------------------
    // Por defecto, toda variable en Rust es inmutable.
    // Esto previene que diferentes partes del código modifiquen el estado por accidente.
    let balance_inicial: i32 = 500;
    println!("1. Saldo inicial (inmutable): ${balance_inicial}");

    // Para permitir mutación, debemos declarar explícitamente `mut`.
    let mut saldo_dinamico: i32 = balance_inicial;
    saldo_dinamico += 250;
    println!("   Saldo dinámico tras depósito: ${saldo_dinamico}\n");

    // ---------------------------------------------------------
    // 2. El Poder del Shadowing (Sombreado de Variables)
    // ---------------------------------------------------------
    // Shadowing permite volver a declarar una variable con el mismo nombre usando `let`.
    // A diferencia de `mut`, el shadowing nos permite cambiar el TIPO de dato
    // conservando la inmutabilidad de la nueva variable.
    let input_usuario = "   42   "; // Tipo: &str (cadena de texto con espacios)
    let input_usuario: usize = input_usuario.trim().len(); // Tipo: usize (número de caracteres limpios)
    println!("2. Shadowing aplicado:");
    println!("   Longitud del texto limpio (mismo identificador, nuevo tipo): {input_usuario} caracteres\n");

    // ---------------------------------------------------------
    // 3. Constantes
    // ---------------------------------------------------------
    println!("3. Constante de servidor:");
    println!("   Límite de conexiones: {MAX_SERVER_CONNECTIONS} clientes concurrentes\n");

    // ---------------------------------------------------------
    // 4. Tipos Escalares a Nivel de Hardware
    // ---------------------------------------------------------
    // Un escalar representa un único valor en memoria.
    // Rust tiene 4 grupos de escalares primarios:

    // A) Enteros:
    // i8 a i128 (con signo: positivos y negativos)
    // u8 a u128 (sin signo: únicamente cero y positivos)
    // isize / usize (dependen de la arquitectura de la CPU: 64 bits en x86_64, 32 bits en x86)
    let puerto: u16 = 8080;
    let memoria_usada: usize = 1024 * 1024; // 1 MB
    println!("4. Enteros:");
    println!("   Puerto HTTP (u16): {puerto}");
    println!("   Bytes en memoria (usize): {memoria_usada} bytes");

    // Demostración de prevención de Overflow seguro:
    let base: u8 = 250;
    // Si sumamos 10 a 250 en u8 (máximo 255), se produce un desbordamiento.
    // Rust nos da métodos explícitos para manejarlo con total seguridad:
    match base.checked_add(10) {
        Some(resultado) => println!("   Suma segura: {resultado}"),
        None => println!("   [ALERTA DE SEGURIDAD] Desbordamiento aritmético prevenido: 250 + 10 excede el límite de u8 (255)"),
    }

    // B) Flotantes (IEEE-754):
    // f32 (precisión simple) y f64 (precisión doble, el predeterminado en Rust por velocidad de CPU moderna).
    let latencia_ms: f64 = 0.425;
    println!("\n5. Flotantes:");
    println!("   Latencia promedio: {latencia_ms} ms");

    // C) Booleanos (1 byte en memoria para alineación):
    let servidor_activo: bool = true;
    println!("\n6. Booleano:");
    println!("   Estado del servidor: {servidor_activo}");

    // D) Caracteres (char):
    // En C/C++, un `char` es de 1 byte (ASCII).
    // En Rust, un `char` representa un valor escalar Unicode (Unicode Scalar Value) de exactamente 4 bytes.
    // Puede representar acentos, caracteres chinos, emojis, etc.
    let letra: char = 'R';
    let emoji: char = '🦀';
    println!("\n7. Caracteres Unicode (4 bytes exactos cada uno):");
    println!("   Letra: {letra} | Mascota de Rust: {emoji}");

    println!("\n¡Demostración de tipos escalares completada exitosamente!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constante_servidor() {
        assert_eq!(MAX_SERVER_CONNECTIONS, 100_000);
    }

    #[test]
    fn test_shadowing_cambio_de_tipo() {
        let dato = "100";
        let dato: i32 = dato.parse().unwrap();
        assert_eq!(dato + 50, 150);
    }

    #[test]
    fn test_checked_add_overflow() {
        let maximo: u8 = 255;
        assert_eq!(maximo.checked_add(1), None);
    }

    #[test]
    fn test_tamanio_char_es_4_bytes() {
        assert_eq!(std::mem::size_of::<char>(), 4);
    }
}
