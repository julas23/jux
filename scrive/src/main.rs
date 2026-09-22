// SPDX-License-Identifier: GPL-3.0-or-later
//
// Scrive: PDF reader for the JUX desktop
//
// Esqueleto: existe para o workspace compilar e para `cargo run -p jux-scrive`
// responder alguma coisa. Ver scrive/README.md para escopo e perguntas em aberto.

fn main() {
    println!(
        "{} {} — {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
}
