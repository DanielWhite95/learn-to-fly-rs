# Learn To Fly With Rust (eGUI)

This repo is the implementation of [the fantastic tutorial about Neural Network
and Genetic Algorithm](https://pwy.io/posts/learning-to-fly-pt1/) by @Patryk27

This project contains two implementation:
- using the [eGUI library](https://github.com/emilk/egui) to display the UI instead of using WASM.
- using wasm to render into a Web Page

A live demo can be tested here: https://danielwhite95.github.io/learn-to-fly-rs/

## Requirements

The app should run on any OS that has support for eGUI.
To install you need to have a rust toolchain installed and cargo (it has been
tested with Rust 1.76+)

To build the app simply run:
```
cargo build
```

### Native Window


To run the application, use this command instead:
```
cargo run
```


### Web UI

To start the Web UI you need first to pack rust code into a WebAssmebly module.
To do this, you need to install the tool [**wasm-pack**](https://rustwasm.github.io/wasm-pack/installer/).
Move into `libs/lib-simulation-wasm` and run this command:

```bash
$ wasm-pack build
```

Once completed, go into the folder `www` and install dependencies:

```bash
$ npm i 
```

and start the server:
```bash
$ npm run start
```
