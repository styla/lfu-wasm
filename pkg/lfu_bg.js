
const path = require('path').join(__dirname, 'lfu_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./lfu.js'] = require('./lfu.js');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
