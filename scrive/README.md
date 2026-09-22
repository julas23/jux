# Scrive (`scrive`)

**PDF reader for JUX.**

Reading, searching, annotating and printing PDF documents.

## Scope

- Rendering, text selection and search
- Outline, thumbnails and page navigation
- Annotations and form filling
- Printing

## Status

**Early planning.** No implementation yet. Of all JUX components, this is the one with
the largest gap between ambition and available foundations.

## The rendering problem

There is no mature pure-Rust PDF renderer. `hayro` describes itself as being at a very
early stage; `fop-pdf-renderer` converts PDF to images. A production-quality viewer
today means a Rust interface over a C/C++ rendering backend.

Backend licences matter, because a dependency's licence applies to the result:

| Backend | Licence | Verdict |
|---|---|---|
| MuPDF | AGPL | Unusable for a distributed desktop application |
| Poppler | GPL-2.0 | Would constrain the licence of the whole project |
| **pdfium** | BSD-3-Clause | The only one that constrains nothing |

## Open questions

- Wrap `pdfium` and accept that this component is not pure Rust, or wait on — or
  contribute to — a Rust renderer reaching maturity?
- Is a PDF reader part of the first release at all, or is an existing one adopted until
  the foundations improve?
