use wasm_bindgen::prelude::*;

// Einfache Grußfunktion (wie im Artikel)
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hallo, {}! Willkommen bei Rust + WebAssembly 🚀", name)
}

// Mathematische Berechnungen - zeigt Performance-Vorteil
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Arbeit mit Arrays - wichtig für Datenverarbeitung
#[wasm_bindgen]
pub fn sum_array(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

// String-Verarbeitung
#[wasm_bindgen]
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

// Boolean-Logik
#[wasm_bindgen]
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Logging für Debugging (erscheint in Browser Console)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn debug_message(msg: &str) {
    log(&format!("🦀 Rust sagt: {}", msg));
}

// Tests für die Funktionen
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("Tester");
        assert!(result.contains("Tester"));
        assert!(result.contains("Rust + WebAssembly"));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_sum_array() {
        let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_array(&numbers), 15.0);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("Hello"), "olleH");
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(18));
    }
}
