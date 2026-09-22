# Orbit

**The JUX desktop shell.**

Orbit is the part you see and touch: the panel, the dock, the workspace model and the
window management that ties them together. It is a Wayland shell, written in Rust.

## Scope

- Panel — clock, weather, system tray, network, battery, volume, brightness, keyboard
  layout, window list, session controls
- Dock — application launchers, generated from `.desktop` files
- Workspaces — a fixed grid, shared across every connected display
- Window management — tiling, plus keyboard snapping to halves and to the full work area

## What makes it different

- **Workspaces span all displays.** Switching to workspace 2 moves every screen to
  workspace 2 at once, instead of each monitor keeping its own independent set.
- **Snapping matches the tiler exactly.** A window snapped to half the screen lands on
  the same pixels the tiling layout would have given it — gaps and borders included —
  so snapped and tiled windows never look subtly misaligned.
- **The dock is generated, not hand-written.** Drop a `.desktop` file in a directory and
  it appears, with icon size and row breaks chosen to fit the screen.
- **Modules are a list in the config**, not a hard-coded layout. Reordering or removing
  a panel widget is a config edit, not a fork.

## Status

**Early planning.** No implementation yet.

## Open questions

- Own compositor (Smithay) from the start, or shell-first on an existing compositor?
- Which window management model is the identity of this project: dynamic tiling,
  scrollable tiling, or the shared-workspace grid from the prototype?
- How the AI agent panel — a first-class shell surface with rendered diffs and one-click
  actions — fits into the layout. This is the most original element of the design and
  needs to be in the plan from the start, not bolted on later.
