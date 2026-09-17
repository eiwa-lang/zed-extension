# zed-eiwa

[Eiwa](https://github.com/leodouglas/eiwa) language support for [Zed](https://github.com/zed-industries/zed).

## Features

- Syntax highlighting for `.ei` files via [tree-sitter-eiwa](../tree-sitter-eiwa)
  (a fork of tree-sitter-kotlin extended with Eiwa syntax)
- ES6-style imports (`import { X } from "mod"`), `test` and `lib` blocks,
  ternary expressions, union types (`Int | Null`), bare `try`/`catch`
- Lambda-style `for` (`for (xs) { it * 2 }`, incl. map-style `for` as a value),
  `leave` with a value, untyped `catch (e)`
- Composition type system: `type` (generics, `: Contract` + `+ Skill` headers),
  `contract`, `skill`, first-class `enum`, type-bound companions
  (`type Config { } object { }`, `object S { } type (...) { }`)
- Varargs (`Int...`), platform targets (`object("windows") X`, `lib("posix") Y`),
  `of` map pairs (`["k" of "v"]`)
- Bracket matching and auto-closing pairs
- Comment toggling (`//` and `/* */`)
- Symbol outline (types, contracts, skills, enums, objects, libs, tests,
  functions, properties)

## Limitations

- No language server (LSP) yet: no autocomplete, go-to-definition or diagnostics.

## Installing (dev extension)

1. Clone this repository.
2. In Zed, open the command palette and run `zed: install dev extension`.
3. Select this directory (`zed-extension/`).

Zed will download and compile the tree-sitter grammar on first install.
