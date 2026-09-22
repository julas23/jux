# Author-X (`authorx`)

**A text editor for structuring text.**

Author-X is a plain text editor built around two ideas that most editors treat as
afterthoughts: **column selection** and **level indentation**. It is for outlining,
restructuring, reshaping tabular scratch data and organising notes — not for writing
software.

## Two features, both first-class

### Column selection

Block and column selection is default behaviour, not a modifier key most people never
discover. Select a rectangle of text across lines, type into all of them at once, delete
a column, insert a column.

### Level indentation

Tab at the start of a line moves it one level in; Shift+Tab moves it one level out.
Levels are **structure**, not whitespace: a line's level defines where it sits in an
outline, so text can be folded, promoted, demoted and moved as a unit, carrying its
children with it.

The result is an outliner that never leaves plain text. No database, no proprietary
format — a file written by Author-X opens correctly in anything, because it is just
tabs and lines.

## Scope

- Column and multi-cursor selection, available by default
- Level indentation, folding, and moving a line with or without its children
- Search and replace, with regular expressions
- Large file handling
- Encoding and line-ending control

## Explicitly out of scope

This is the fence that keeps the project finishable, and it is deliberate:

- **No language integration.** No language servers, no autocompletion, no diagnostics.
- **No syntax highlighting** driven by language grammars.
- **No debugging, no build tasks, no extensions, no project management.**

Author-X does not compete with a code editor. Writing software is what VS Code, Helix
and their kind are for. Author-X organises the structure of text, and that is the whole
of it.

## Status

**Early planning.** No implementation yet.

## Known hard parts

- **Unicode.** Grapheme clusters, bidirectional text and input methods are where naive
  editors break. Column selection makes this harder, because a "column" is a visual
  position — not a byte offset, and not even a character offset.
- **Undo** that stays coherent across multiple simultaneous cursors.
- **Large files**, without reading them entirely into memory.

Text storage itself is a solved problem in Rust: `ropey` is the obvious foundation, and
multi-cursor editing is a first-class concept in the Helix and Kakoune model, which is
the right reference to study.

The Unicode problem is largely solved too, and by the toolkit we already chose:
[`cosmic-text`](https://github.com/pop-os/cosmic-text) handles shaping, layout, grapheme
clusters, bidirectional text and editing, and iced's text editing is built on it. That
is a considerable head start on the nastiest part of this component.

## Open design questions

Scope is settled; these are about how the two features meet.

- **Column selection across lines at different levels.** If a rectangle spans a line at
  level 1 and a line at level 3, does the column start from the left margin or from each
  line's own indentation? Both are defensible and they feel completely different to use.
- **Tab as structure versus tab as a character.** If Tab at the start of a line changes
  level, what inserts a literal tab mid-line — and what happens when a file already
  contains tabs that were never meant as levels?
- **What a level is in a file that uses spaces.** Whether Author-X reads space
  indentation as levels on open, and what it writes back.
