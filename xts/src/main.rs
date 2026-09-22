// SPDX-License-Identifier: GPL-3.0-or-later
//
// xTree-Silver: file manager for the JUX desktop
//
// Esqueleto: existe para o workspace compilar e para `cargo run -p jux-xts`
// responder alguma coisa. Ver xts/README.md para escopo e perguntas em aberto.

fn main() {
    println!(
        "{} {} — {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
}
