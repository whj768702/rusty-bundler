import multiply, { add } from "./utils.js";
console.log(add(2, 3));
console.log(multiply(2, 3));

function add(a, b) {
  return a + b;
}

function multiply(a, b) {
  return a * b;
}

export { add };
export default multiply;
