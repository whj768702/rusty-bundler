const modules = {
 "./utils.js": function(require, module, exports) {
// exports.greet= function(name) {
//   console.log(`Hello, ${name}!`);
// }
module.exports = "hello";

  },
 "./index.js": function(require, module, exports) {
// const { greet } = require("./utils.js");
// greet("Rust");
const msg = require('./utils.js');
console.log(msg);

  },
};


        function require(id) {
            const module = {exports: {}};
            modules[id](require, module, module.exports);
            return module.exports;
        }
        
require("examples/simple/index.js");
