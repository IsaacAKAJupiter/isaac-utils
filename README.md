# Isaac Utils

A collection of utilities made by and for Isaac (and maybe others).

## Current Utilities

- TODO

## Building

This is a rough draft for local building requirements. Also does not include the certificate that `pnpm build` is looking for.

### Windows

- Install rust, node, and pnpm.
- Run `pnpm i`.
- Run `pnpm build`.

### Fedora

- Install rust, node, and pnpm.
- Run `pnpm i`.
- Install dependencies.
  - `sudo dnf check-update`
  - `sudo dnf install webkit2gtk4.0-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel javascriptcoregtk4.1-devel libsoup3-devel webkit2gtk4.1-devel`
  - `sudo dnf group install "C Development Tools and Libraries"`
- Run `pnpm build`.

### Debian

- Install rust, node, and pnpm.
- Run `pnpm i`.
- Install dependencies.
  - `sudo apt update`
  - `sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`
- Run `pnpm build`.
