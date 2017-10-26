# Rust + HTML5 snake game

[Finished page](https://alamminsalo.github.io/snakewasm)

### Prequisites

* [Emscripten SDK](http://kripken.github.io/emscripten-site) (plenty of instructions to set up available by googling)
* Rust toolchain with ```wasm32-unknown-emscripten``` target installed
* Nodejs

### Building

Clone the repo and simply run ```npm install``` to download dependencies.

Then compile the project with ```npm run compile``` to compile rust code to .wasm files and bundle webcontent (html, css, js files).

To serve locally, run ```npm run serve``` and open up web-browser on localhost:8080

### Misc

* UI was made using vue.js but any framework will do (or even without)
* The rust code wrapper is inside ```game.js``` and rust functions it uses are visible in ```main.rs```. 
Made using instructions on [emscripten documentations](https://kripken.github.io/emscripten-site/docs/porting/connecting_cpp_and_javascript/Interacting-with-code.html)
* To me, a helpful tutorial to get started was [this article](https://medium.com/@ianjsikes/get-started-with-rust-webassembly-and-webpack-58d28e219635)

