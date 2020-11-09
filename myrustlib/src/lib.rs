/*
Install wasm-pack:
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

Create this project:
    cargo new myrustlib --lib

Add wasm-bindgen = "0.2" to dependencies in Cargo.toml

Packing:
    wasm-pack build --release --target nodejs --out-dir pkg-node --no-typescript
    wasm-pack build --release --target bundler --out-dir pkg-web --no-typescript

Remove Eslint check on WEB:
    npm remove @vue/cli-plugin-eslint
*/

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
