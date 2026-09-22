// SPDX-License-Identifier: GPL-3.0-or-later
//
// xSight: image viewer for the JUX desktop
//
// Esqueleto: existe para o workspace compilar e para `cargo run -p jux-xsight`
// responder alguma coisa. Ver xsight/README.md para escopo e perguntas em aberto.

fn main() {
    println!(
        "{} {} — {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
}
