# eXact (`exct`)

**Calculator for JUX.**

Three calculators in one window, switched with a single control rather than buried in a
menu: standard, financial and scientific.

## Scope

- **Standard** — everyday arithmetic, memory, percentage
- **Financial** — time value of money (`n`, `i`, `PV`, `PMT`, `FV`), interest rate
  conversion, amortisation, cash-flow analysis. The reference behaviour is an HP-12C.
- **Scientific** — trigonometry, logarithms, powers and roots, statistics, number bases
- Calculation history, and keyboard operation as a first-class path

## Status

**Early planning.** No implementation yet.

## Why this one is first

eXact is the smallest component by a wide margin and the most completely specified —
financial calculator behaviour is a solved, documented problem. That makes it the right
place to settle the questions that every other component depends on: which GUI toolkit,
what the configuration format looks like, how theming works, how the build and packaging
are laid out.

Shipping it proves the stack on something whose correctness can actually be verified.
