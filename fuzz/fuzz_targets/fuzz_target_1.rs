#![no_main]
use libfuzzer_sys::fuzz_target;
use br_tax_id::validate_tax_id; // Importando minha função

fuzz_target!(|data: &[u8]| {
    // Tenta converter os bytes aleatórios em uma string UTF-8 
    if let Ok(texto_aleatorio) = std::str::from_utf8(data) {
        // Atira o texto contra o seu validador!
        // Não nos importamos com Ok() ou Err() aqui.
        // O nosso único objetivo é garantir que essa função NUNCA cause um "panic!".
        let _ = validate_tax_id(texto_aleatorio);
    }
});