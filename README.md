# JUX

**A Wayland desktop environment and application suite, written in Rust.**

JUX is one desktop, built as a set of independent components that share a design
language, a configuration model and a toolkit. The goal is a desktop that is fast and
visually modern without being a reskin of something else, and complete enough to be
used as-is.

Everything here is a clean reimplementation. Existing projects are read as reference,
model and example — never as a source of copied code.

## Components

| Component | Command | Crate | What it is |
|---|---|---|---|
| **Orbit** | `orbit` | `jux-orbit` | The desktop shell — compositor, panel, dock, workspaces, window management |
| **xTree-Silver** | `xts` | `jux-xts` | File manager |
| **xSight** | `xsight` | `jux-xsight` | Image viewer |
| **Scrive** | `scrive` | `jux-scrive` | PDF reader |
| **eXact** | `exct` | `jux-exct` | Calculator — standard, financial and scientific |
| **Author-X** | `authorx` | `jux-authorx` | Text editor for structuring text — column selection and level indentation |

Crate names carry a `jux-` prefix; binary names do not. `orbit` and `xts` are already
taken on crates.io by placeholder crates, and the prefix keeps the namespace safe
without changing what you type in a terminal.

## Building

A Cargo workspace, one crate per component. Requires Rust 1.85 or newer (edition 2024).

```sh
cargo build --workspace     # everything
cargo run -p jux-exct       # one component
```

Every binary currently prints its name and description and exits — the crates are
skeletons, present so the workspace builds and the layout is real.

## Toolkit

**Rust + [iced](https://iced.rs), across every component.** One toolkit for the whole
suite buys visual consistency for free and leaves one surface to learn and maintain
instead of several.

[Slint](https://slint.dev) was considered seriously and rejected. It is lighter, better
documented and has a more stable API — but three things decided against it here:

- **Layer-shell.** Orbit's panel and dock must be `wlr-layer-shell` surfaces. On iced
  that path is production-proven: `libcosmic` exists for exactly this and ships in
  COSMIC's own panel and applets. Slint's only route is a third-party bridge that
  describes itself as pre-1.0 and not ready for production.
- **Licence.** Slint's GPL option is `GPL-3.0-only`, which would pin our binaries to
  version 3 and make our `-or-later` unexercisable. The alternative — Slint's
  royalty-free licence — is free for desktop but requires displaying Slint attribution
  in the application. A desktop environment should not be contractually obliged to show
  another company's badge.
- **Reference material.** The architecture here follows COSMIC. With iced, `libcosmic`
  and every COSMIC application are directly readable references for the problems we will
  hit. With Slint they are one language removed.

Worth naming specifically: [`cosmic-text`](https://github.com/pop-os/cosmic-text) solves
shaping, layout, grapheme clusters, bidirectional text and editing — which is exactly the
list of hard parts in Author-X's README. iced's text editing is built on it.

This decision would deserve revisiting if the shell were dropped from scope or if
footprint became the single dominating metric.

## Principles

- **Configuration over forking.** Layout, modules, colours and behaviour come from a
  config file. Nobody should have to edit source code to move a panel or drop a widget.
- **Event-driven, never polling.** The shell sleeps until something changes. The target
  is 0.0% CPU on an idle screen and sub-16ms response to input.
- **Adopt, don't rewrite.** Browser, office suite, image editor and IDE are not our
  problem. Depth in the shell, breadth by adoption.
- **Explain the why.** Comments say why a line exists, not what it does. A contributor
  should be able to understand a decision without asking.

## Status

**Early planning.** Directory structure and intent only — no implementation yet.
Each component has its own README describing scope and open questions.

## Open decisions

These are deliberately unresolved and tracked here so they are not decided by accident:

- **Compositor strategy.** Whether Orbit ships its own compositor from the start
  (Smithay) or runs its shell on an existing one first.

## Licence

JUX is free software, licensed under the **GNU General Public License, version 3 or
(at your option) any later version** — `GPL-3.0-or-later`. The full text is in
[LICENSE](LICENSE).

This applies to every component in the repository. A copyleft licence is a deliberate
choice: work that comes back to this project stays available to everyone who uses it.

Because JUX is a clean reimplementation rather than a derivative of any existing
project, this licence was free to choose. What still has to be checked, per component,
is the licence of each **dependency** — a crate's terms apply to whatever links against
it, and some are incompatible with distributing a desktop application at all.

## Built from

The starting point is a working bspwm + eww desktop: panel, dock, workspace model,
window snapping and a dock generator driven by `.desktop` files. That prototype
validated the design; JUX is the native rewrite of it.
