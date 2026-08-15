#![no_main]
use libfuzzer_sys::fuzz_target;
use br_tax_id::validate_tax_id; // Importing my function

fuzz_target!(|data: &[u8]| {
    // Attempts to convert the random bytes into a UTF-8 string
    if let Ok(texto_aleatorio) = std::str::from_utf8(data) {
        // Throw the text at your validator!
        // We don't care about Ok() or Err() here.
        // Our only goal is to ensure that this function NEVER causes a "panic!".
        let _ = validate_tax_id(texto_aleatorio);
    }
});