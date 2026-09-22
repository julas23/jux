// SPDX-License-Identifier: GPL-3.0-or-later
//
// Author-X: text editor with native column selection for the JUX desktop
//
// Esqueleto: existe para o workspace compilar e para `cargo run -p jux-authorx`
// responder alguma coisa. Ver authorx/README.md para escopo e perguntas em aberto.

fn main() {
    println!(
        "{} {} — {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
}
