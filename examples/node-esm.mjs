// run with node --experimental-wasm-modules node-esm.mjs <file>
import {detect} from '../dist/chardetng_wasm.js';
import fs from 'fs';
import path from 'path';

if (process.argv.length < 3) {
  console.log('usage: node --experimental-wasm-modules node-esm.mjs <file>');
  process.exit();
}

const file = path.join(process.cwd(), process.argv[2]);
const content = fs.readFileSync(file);
console.log(detect(content));
