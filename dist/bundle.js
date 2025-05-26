const modules = {
 "./c.js": function(require, module, exports) {
const a = require('./a.js');
module.exports = {  };

  },
 "./b.js": function(require, module, exports) {
const c = require("./c.js");
module.exports = {  };

  },
 "./a.js": function(require, module, exports) {
const b = require('./b.js');
module.exports = {  };

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
                
require("./c.js");
