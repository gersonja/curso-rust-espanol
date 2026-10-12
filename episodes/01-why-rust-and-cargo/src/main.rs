//! Episodio 01: Por qué Rust y el Ecosistema Cargo
//!
//! Este código demuestra los fundamentos de la ejecución nativa en Rust:
//! 1. Asignaciones en Stack vs. Heap.
//! 2. Liberación determinista de recursos (RAII / Trait `Drop`) sin recolector de basura.
//! 3. El comportamiento del compilador y optimizaciones de Cargo.

/// Estructura simple para demostrar la liberación determinista de memoria.
/// Implementa `Drop` para que podamos observar exactamente cuándo el hardware libera el recurso.
struct MemoryTracker {
    name: &'static str,
    bytes_allocated: usize,
}

impl MemoryTracker {
    fn new(name: &'static str, bytes_allocated: usize) -> Self {
        println!("  [ALLOC] Recurso '{}' creado en memoria ({} bytes)", name, bytes_allocated);
        Self { name, bytes_allocated }
    }
}

impl Drop for MemoryTracker {
    fn drop(&mut self) {
        println!(
            "  [DROP]  Recurso '{}' liberado de forma determinista por el compilador ({} bytes devueltos al OS)",
            self.name, self.bytes_allocated
        );
    }
}

fn main() {
    println!("=== 01. BIENVENIDO A RUST Y AL ECOSISTEMA CARGO ===");
    println!("Binario nativo compilado sin Máquina Virtual ni Garbage Collector.\n");

    // 1. Tipos en Stack: Tamaño fijo conocido en tiempo de compilación.
    // Muy rápidos, asignados contiguamente en el puntero de pila (Stack Pointer).
    let stack_number: i64 = 42;
    let stack_flag: bool = true;
    println!("1. Datos en el Stack:");
    println!("   Número (i64): {} | Booleano: {}", stack_number, stack_flag);
    println!("   Cero llamadas a malloc/allocator del sistema operativo.\n");

    // 2. Datos en el Heap: Tamaño dinámico o crecimiento en tiempo de ejecución.
    // Un objeto 'String' en Rust consta de 3 palabras en el Stack:
    // [ptr: puntero al Heap, len: longitud actual, cap: capacidad reservada].
    println!("2. Datos en el Heap:");
    {
        println!("--> Entrando a un nuevo bloque léxico (Scope local)...");
        let _tracker = MemoryTracker::new("Buffer_HTTP", 1024);

        let mut dynamic_payload = String::from("Rust Video Academy");
        dynamic_payload.push_str(" - Google Comprehensive Rust");

        println!("   Payload en Heap: \"{}\"", dynamic_payload);
        println!("   Longitud: {} bytes | Capacidad: {} bytes", dynamic_payload.len(), dynamic_payload.capacity());

        println!("<-- Saliendo del bloque léxico...");
        // Al terminar el scope, _tracker y dynamic_payload son liberados automáticamente.
        // No hay pausas de Garbage Collector ni riesgo de olvidar un free() manual.
    }

    println!("\n3. Garantías de compilación:");
    println!("   - Si compilas con 'cargo build', obtienes binarios con símbolos de depuración rápidos.");
    println!("   - Si compilas con 'cargo build --release', LLVM optimiza agresivamente a nivel ensamblador.");
    println!("\n¡Compilación limpia y ejecución completada exitosamente!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_values() {
        let val: i32 = 100;
        assert_eq!(val * 2, 200);
    }

    #[test]
    fn test_memory_tracker_lifecycle() {
        let tracker = MemoryTracker::new("TestBuffer", 512);
        assert_eq!(tracker.bytes_allocated, 512);
        assert_eq!(tracker.name, "TestBuffer");
    }
}
