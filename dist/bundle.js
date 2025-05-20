const modules = {
 "./index.js": function(require, module, exports) {
const msg = require("./utils.js");;
console.log(msg);

  },
 "./utils.js": function(require, module, exports) {
module.exports = "hello world!";

  },
};


        function require(id) {
            const module = {exports: {}};
            modules[id](require, module, module.exports);
            return module.exports;
        }
        
require("examples/simple/index.js");
