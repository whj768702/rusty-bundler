const multiply = require("./utils.js").default; const { add } = require("./utils.js");
console.log(add(2, 3));
console.log(multiply(2, 3));

module.exports = {  };

function add(a, b) {
  return a + b;
}

function multiply(a, b) {
  return a * b;
}



module.exports = { add: add, default: multiply };

