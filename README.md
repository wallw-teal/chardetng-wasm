# chardetng-wasm

Makes [`chardetng`](https://github.com/hsivonen/chardetng) (Firefox's character encoding detection library) available to JS via Web Assembly.

## Usage

If you are using the repo directly, build before trying the examples

* Browser
  * ES5: see [examples/es5.html](blob/master/examples/es5.html)
  * ESM: use a bundler such as `npm init wasm-app`
* Node
  * ESM: see [examples/node-esm.js](blob/master/examples/node-esm.js) (NOTE: requires `--experimental-wasm-modules`)

## API

### detect(uint8Array): String

Given a buffer, guess the character encoding. This is simple wrapper which is equivalent to:

```javascript
function detect(uint8Array) {
  const detector = EncodingDetector.new();

  // does not assume end of stream
  detector.feed(uint8Array, false);

  // defaults to 'com' topLevelDomain and allows UTF-8
  return detector.guess(undefined, true);
}
```

### EncodingDetector

This is an interface to the original `EncodingDetector` struct in Rust. [See those docs for details](https://docs.rs/chardetng/0.1.10/chardetng/struct.EncodingDetector.html).

All arguments other than the buffer for `feed()` are optional.

Example:
```javascript
const detector = EncodingDetector.new();

// ... set up a stream of bytes into a buffer
// while reading ...
detector.feed(buffer); // isLast defaults to false

// Optional:
// when finished, if all the feeds represent a full file then mark it finished with no bytes sent
detect.feed('', true);

// these are the defaults
const topLevelDomain = 'com';
const allowUTF8 = false;

console.log(detector.guess(topLevelDomain, allowUTF8));
console.log(detector.guess());
```

## Building

### Setup
Prerequisites:
```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

# grab the nightly compiler
rustup toolchain install nightly

# set nightly as the default for this project
rustup override set nightly

# install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
You will also need node stable and npm
```bash
npm install
```

### Build
`npm run build`

## License

Copyright BIT Systems

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
