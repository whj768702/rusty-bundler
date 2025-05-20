const modules = {
 "./index.js": function(require, module, exports) {
require('./utils.js');
require('./utils.js');
require('./utils.js');
const msg = require('./utils.js');
console.log(msg);

  },
 "./utils.js": function(require, module, exports) {
console.log("hello world!");
module.exports = "hello world!";

  },
};


        const cache = {};
        function require(id) {
            if(cache[id]){
                return cache[id].exports;
            }
            const module = {exports: {}};
            cache[id] = module;
            modules[id](require, module, module.exports);
            return module.exports;
        }
        
require("./index.js");
