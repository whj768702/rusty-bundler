const modules = {
  "./utils.js": function (require, module, exports) {
    function add(a, b) {
      return a + b;
    }

    function multiply(a, b) {
      return a * b;
    }



    module.exports = { add: add, default: multiply };

  },
  "./index.js": function (require, module, exports) {
    const multiply = require("./utils.js").default; const { add } = require("./utils.js");
    console.log(add(2, 3));
    console.log(multiply(2, 3));

    module.exports = {};

  },
};


const cache = {};
function require(id) {
  if (cache[id]) {
    return cache[id].exports;
  }
  const module = { exports: {} };
  cache[id] = module;
  modules[id](require, module, module.exports);
  return module.exports;
}

require("./index.js");
