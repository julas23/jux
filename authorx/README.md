# Author-X (`authorx`)

**Text editor for JUX, with native column selection.**

A text editor where block and column selection is a first-class, default behaviour
rather than a modifier key most people never discover.

## Scope

- Column and multi-cursor selection, available by default
- Syntax highlighting, search and replace with regular expressions
- Large file handling
- Encoding and line-ending control

## Status

**Early planning.** No implementation yet.

## Known hard parts

- **Unicode.** Grapheme clusters, bidirectional text and input methods are where naive
  editors break. Column selection makes this harder, because a "column" is a visual
  position, not a byte offset or even a character offset.
- **Undo** that stays coherent across multiple simultaneous cursors.
- **Large files**, without reading them entirely into memory.

The text storage problem itself is well solved in Rust — `ropey` is the obvious
foundation, and multi-cursor editing is a first-class concept in the Helix and Kakoune
model, which is the right reference to study.

## Open question, and it is a big one

**Is this a text editor or a code editor?** The difference is years of work. A text
editor in the gedit or Mousepad class is a bounded project. A code editor competing
with VS Code means language servers, debugging, extensions and project management —
a different undertaking entirely.

This must be answered before any estimate is meaningful.
