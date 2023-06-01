# Running the project locally
 * install devserver if not yet installed: cargo install devserver.

 * run the command: 
	devserver

* visit localhost:8080


## while developing

  * Use the following command to watch the file ./examples/game.rs for changes, and compile after each save an convert
  * The rust code to a WebAssembly Binary file located in ./target/wasm32-unknown-unknown/debug/examples/game.wasm
	* cargo watch -x 'build --example game --target wasm32-unknown-unknown'
