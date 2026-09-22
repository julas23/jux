# JUX

**A Wayland desktop environment and application suite, written in Rust.**

JUX is one desktop, built as a set of independent components that share a design
language, a configuration model and a toolkit. The goal is a desktop that is fast and
visually modern without being a reskin of something else, and complete enough to be
used as-is.

Everything here is a clean reimplementation. Existing projects are read as reference,
model and example — never as a source of copied code.

## Components

| Component | Command | What it is |
|---|---|---|
| **Orbit** | `orbit` | The desktop shell — compositor, panel, dock, workspaces, window management |
| **xTree-Silver** | `xts` | File manager |
| **xSight** | `xsight` | Image viewer |
| **Scrive** | `scrive` | PDF reader |
| **eXact** | `exct` | Calculator — standard, financial and scientific |
| **Author-X** | `authorx` | Text editor with native, first-class column selection |

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

- **Licence.** Not yet chosen. Since this is a clean reimplementation, the choice is
  free and is about who may use the result, not about compatibility. Dependency
  licences are what need checking — not the licences of the projects studied.
- **GUI toolkit.** `iced` is the leading candidate, for consistency across the whole
  suite. Alternatives under consideration: Slint, GTK4, or raw `wayland-client`.
- **Compositor strategy.** Whether Orbit ships its own compositor from the start
  (Smithay) or runs its shell on an existing one first.

## Built from

The starting point is a working bspwm + eww desktop: panel, dock, workspace model,
window snapping and a dock generator driven by `.desktop` files. That prototype
validated the design; JUX is the native rewrite of it.
