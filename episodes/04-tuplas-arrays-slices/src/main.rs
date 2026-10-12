//! Episodio 04: Estructuras Fijas en Memoria: Tuplas, Arrays y Slices Básicos
//!
//! En este episodio demostramos:
//! 1. Tuplas: agrupaciones heterogéneas, acceso posicional (`.0`) y destructuring.
//! 2. Arrays: colecciones homogéneas contiguas en el Stack con longitud fija (`[T; N]`).
//! 3. Seguridad de límites (Bounds Checking): por qué Rust no permite Buffer Overflows.
//! 4. Slices básicos (`&[T]`): vistas no propietarias con fat pointers (puntero + longitud).

fn main() {
    println!("=== EPISODIO 04: TUPLAS, ARRAYS Y SLICES BÁSICOS ===\n");

    // ---------------------------------------------------------
    // 1. Tuplas: Agrupación Heterogénea
    // ---------------------------------------------------------
    // Una tupla agrupa valores de diferentes tipos bajo un mismo identificador.
    // Su tamaño es fijo y se almacena en el Stack.
    let respuesta_servidor: (u16, &str, bool) = (200, "OK", true);

    // Acceso por índice posicional con sintaxis de punto (.0, .1, .2):
    println!("1. Tupla con acceso posicional:");
    println!("   Código HTTP: {}", respuesta_servidor.0);
    println!("   Mensaje:     {}", respuesta_servidor.1);
    println!("   Éxito:       {}\n", respuesta_servidor.2);

    // Desestructuración (Destructuring) de tuplas:
    let (codigo, mensaje, _) = respuesta_servidor;
    println!("   Desestructuración: Recibido {codigo} con estado '{mensaje}'\n");

    // La tupla vacía o Unit Type `()`:
    // Es el valor que devuelven las funciones o expresiones que no retornan nada.
    println!("   Unit Type (): {:?}", ());

    // ---------------------------------------------------------
    // 2. Arrays: Memoria Contigua en el Stack
    // ---------------------------------------------------------
    // Un array en Rust contiene elementos del MISMO tipo y su tamaño N es fijo
    // y debe conocerse en tiempo de compilación.
    // Tipo: [T; N] -> [i32; 5]
    let lecturas_temperatura: [i32; 5] = [21, 23, 22, 25, 20];

    println!("2. Array contiguo en Stack:");
    println!("   Primer elemento: {}", lecturas_temperatura[0]);
    println!("   Total elementos: {}", lecturas_temperatura.len());
    println!("   Bytes ocupados en Stack: {} bytes (5 * 4 bytes de i32)\n", std::mem::size_of_val(&lecturas_temperatura));

    // Inicialización rápida con valor repetido:
    // [0; 4] crea un array de 4 ceros: [0, 0, 0, 0]
    let buffer_ceros = [0u8; 4];
    println!("   Array repetido [0u8; 4]: {:?}\n", buffer_ceros);

    // ---------------------------------------------------------
    // 3. Slices Básicos: Vistas sin Copia (Fat Pointers)
    // ---------------------------------------------------------
    // Un slice (`&[T]`) es una referencia a una porción contigua de un array.
    // En el Stack solo almacena 2 palabras de máquina (un Fat Pointer):
    // 1. Puntero al primer elemento en la memoria.
    // 2. Longitud de la subsección.
    // Cero duplicación de memoria en Heap ni en Stack.
    let subseccion: &[i32] = &lecturas_temperatura[1..4]; // Elementos en índices 1, 2 y 3 (23, 22, 25)

    println!("3. Slice básico (&[i32]):");
    println!("   Subsección [1..4]: {:?}", subseccion);
    println!("   Longitud del slice: {}", subseccion.len());
    println!("   Primer valor del slice: {}\n", subseccion[0]);

    // Función que recibe un slice para aceptar cualquier array de cualquier tamaño:
    analizar_metricas(&lecturas_temperatura);
    analizar_metricas(subseccion);

    println!("\n¡Demostración de estructuras fijas completada exitosamente!");
}

/// Función idiomática que recibe un slice `&[i32]`.
/// Esto permite que funcione tanto con arrays completos de 5 elementos como con subsecciones.
fn analizar_metricas(datos: &[i32]) {
    println!("   -> Analizando métricas ({} elementos): primer elemento = {}", datos.len(), datos[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tupla_valores() {
        let t = (10, "test");
        assert_eq!(t.0, 10);
        assert_eq!(t.1, "test");
    }

    #[test]
    fn test_slice_tamano() {
        let arr = [1, 2, 3, 4, 5];
        let sl = &arr[1..3];
        assert_eq!(sl.len(), 2);
        assert_eq!(sl[0], 2);
        assert_eq!(sl[1], 3);
        analizar_metricas(sl);
    }
}
