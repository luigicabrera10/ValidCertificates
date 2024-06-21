#![no_std]
extern crate pwasm_std;
extern crate pwasm_abi;
use pwasm_std::hash::{keccak256, H256};
use pwasm_abi::types::*;

// Definición del contrato
#[no_mangle]
pub extern "C" fn associate_tokens(user_address: Address, tokens: Vec<String>) -> bool {
    // Implementación para asociar tokens al usuario
    // Aquí se simula guardando los tokens en un mapa (HashMap)
    let mut token_map: pwasm_std::collections::HashMap<Address, Vec<H256>> = pwasm_std::collections::HashMap::new();
    
    // Convertir los tokens de tipo String a H256 (hashes)
    let mut token_hashes: Vec<H256> = Vec::new();
    for token in tokens {
        let token_hash = keccak256(&token.as_bytes());
        token_hashes.push(token_hash);
    }

    // Asociar los tokens con el usuario en el mapa
    token_map.insert(user_address, token_hashes);

    true // Retornar true si la asociación fue exitosa
}

// Función de prueba para obtener los tokens asociados a un usuario (no es parte del contrato real)
#[no_mangle]
pub extern "C" fn get_tokens(user_address: Address) -> Vec<H256> {
    let mut token_map: pwasm_std::collections::HashMap<Address, Vec<H256>> = pwasm_std::collections::HashMap::new();
    // Aquí se simula obtener los tokens asociados al usuario desde el mapa
    match token_map.get(&user_address) {
        Some(tokens) => tokens.clone(), // Clonar los tokens para devolverlos
        None => Vec::new(), // Si no hay tokens asociados, devolver un vector vacío
    }
}
