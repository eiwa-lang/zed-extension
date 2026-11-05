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

## Language server (`eiwa-lsp`)

This extension registers the Eiwa-native language server (`eiwa-lsp`,
stdio LSP 3.17): diagnostics via `eiwac check`, hover, completion,
go-to-definition and document symbols for `.ei` files.

1. Build the server (repo `eiwa-lang/lsp/`): `eiwa build`
   (or `eiwac build -o bin/eiwa-lsp src/main.ei`).
2. Install this folder as a dev extension
   (palette → `zed: install dev extension`).
3. Point Zed at the binary in `settings.json`
   (GUI apps don't inherit your shell `PATH`, so use an absolute path;
   this also makes the `lsp.eiwa-lsp` key valid):

```json
{
  "lsp": {
    "eiwa-lsp": {
      "binary": { "path": "/abs/path/to/eiwa-lsp" }
    }
  }
}
```

4. Reload the window, open a `.ei` file, save to see diagnostics.
   Logs: palette → `dev: open language server logs` → `eiwa-lsp`.

`eiwa-lsp` shells out to `eiwac check` for diagnostics, so `eiwac` must be
on `PATH` too (same absolute-path caveat applies; `EIWAC` env overrides it).

## Installing (dev extension)

1. Clone this repository.
2. In Zed, open the command palette and run `zed: install dev extension`.
3. Select this directory (`zed-extension/`).

Zed will download and compile the tree-sitter grammar on first install.
