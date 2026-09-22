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

## Toolkit

**Rust + [iced](https://iced.rs)**, via [`libcosmic`](https://github.com/pop-os/libcosmic)
for the layer-shell surfaces. The panel and the dock are not ordinary windows — they are
`wlr-layer-shell` surfaces, which is what makes a window behave as a bar instead of
floating in the layout. libcosmic exists for precisely this and ships in COSMIC's own
panel and applets. See the [root README](../README.md#toolkit).

## Compositor

**Orbit ships its own compositor, built on [Smithay](https://smithay.github.io), from
the start.** It is not a shell layered on someone else's compositor.

The reason is the one thing that cannot be retrofitted cheaply: **animation and visual
effects are part of the compositor's render loop, not a layer above it.** A desktop that
is meant to feel fast and modern has to be designed that way from the first commit.
COSMIC is currently adding blur, shadows and a Vulkan renderer in its second epoch,
after shipping 1.0 — which is exactly the cost of leaving it for later.

### What Smithay gives, and what it does not

Smithay provides the plumbing: core Wayland protocols and extensions, session handling,
graphics and input backends, and `Allocator`/`Renderer` traits with GLES and Pixman
implementations.

It explicitly does **not** provide window management or drawing logic. The tiling
policy, the workspace model, the layout algorithm and the entire effects system are
ours to write. That is the point — it is where the identity of this project lives — but
it should be understood as the bulk of the work, not a detail.

### Starting points

- **`smallvil`** — Smithay's minimal example compositor. The skeleton to begin from.
- **`anvil`** — Smithay's full reference compositor: multi-GPU, XWayland, all backends.
  The place to read when a real problem appears.

### Backends, and why the first window is not far away

Smithay offers three, and the first one matters more than it looks:

- **winit** — runs the compositor *as a window inside an existing session*. Development
  and debugging happen without leaving a working desktop and without TTY switching.
- **X11** — the same idea, inside an X11 session.
- **udev/DRM + libinput** — the real thing, on a TTY.

This is what makes "own compositor from day one" a reasonable choice rather than a leap:
the first window appears inside the desktop you already use, and the current bspwm + eww
session stays the daily driver until Orbit is ready to replace it.

### Milestones

1. A `smallvil`-equivalent running nested on the winit backend: a window opens, focus moves.
2. udev/DRM + libinput backend — runs on a TTY as a real session.
3. `wlr-layer-shell` support, so the panel and dock can attach.
4. The window management policy: the shared workspace grid, tiling, snapping.
5. The animation and effects system.
6. Multi-monitor, hotplug and output configuration.
7. Daily-drivable.

The render loop must treat **animation as a first-class concept from milestone 1**, even
though the first visible effect only lands at milestone 5. Retrofitting that is the
mistake this whole decision exists to avoid.

### The accepted trade-off

This is the slower path to a usable desktop, and it was chosen deliberately. The five
applications in the suite are ordinary Wayland clients, unaffected by this decision, and
can proceed in parallel.

## Status

**Early planning.** No implementation yet.

## Open questions

- Own compositor (Smithay) from the start, or shell-first on an existing compositor?
- Which window management model is the identity of this project: dynamic tiling,
  scrollable tiling, or the shared-workspace grid from the prototype?
- How the AI agent panel — a first-class shell surface with rendered diffs and one-click
  actions — fits into the layout. This is the most original element of the design and
  needs to be in the plan from the start, not bolted on later.
