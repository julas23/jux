// SPDX-License-Identifier: GPL-3.0-or-later
//
// JUX desktop shell: compositor, panel, dock, workspaces and window management
//
// Esqueleto: existe para o workspace compilar e para `cargo run -p jux-orbit`
// responder alguma coisa. Ver orbit/README.md para escopo e perguntas em aberto.

fn main() {
    println!(
        "{} {} — {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
}
