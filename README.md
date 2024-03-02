Long John Silver
================

This is a [Leptos](https://leptos.dev/) learning project, intended for people who run FFXIV maps.

Development
-----------

To get started, make sure you've installed Rust on your system:

1. Install Rustup according to the instructions on [this page](https://www.rust-lang.org/tools/install)
2. Install Node.js according to the instructions on [this page](https://nodejs.org/en/download)
3. In a shell, `cargo install trunk` to install the [Trunk](https://trunkrs.dev/) build tool
4. Git clone this repository
5. Open a shell and CD into the repo directory, then run:
6. `npm install` to install the Bulma CSS library
7. `rustup toolchain install nightly`
8. `rustup override set nightly`
9. `rustup target add wasm32-unknown-unknown`

To run a local application and open it in a browser window, `trunk serve --open`