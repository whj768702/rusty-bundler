(function(modules) {
         const require = (id) => {
           const fn = modules[id];
           const module = { exports: {} };
           fn(require, module, module.exports);
           return module.exports;
         };
         require("./index.js");
       })({
         "./index.js": function(require, module, exports) {
const { greet } = require("./utils.js");
greet("Rust");

}, 
"./utils.js": function(require, module, exports) {
exports.greet= function(name) {
  console.log(`Hello, ${name}!`);
}

}, 

       });