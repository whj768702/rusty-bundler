// Bundled by rust-bundler

// module: examples/simple/index.js
import { greet } from "./utils.js";
greet("Rust");


// module: examples/simple/./utils.js
export function greet(name) {
  console.log(`Hello, ${name}!`);
}

