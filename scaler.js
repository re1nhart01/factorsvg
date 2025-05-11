const svgpath = require("./svgpath");

function scale50(d, scale) {
  return svgpath(d)
    .abs()
    .scale(scale)
    .round(1)
    .toString();
}
const scale = +process.argv[3];
const input = process.argv[2];

if (!input) {
  console.error("Usage: node scaler.js '<path_d>'");
  process.exit(1);
}

const result = scale50(input, scale);
console.log(result);