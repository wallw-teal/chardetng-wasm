let _callbacks = [];

function onReady(fn) {
  if (self.chardetng.detect) {
    fn();
  } else {
    _callbacks.push(fn);
  }
}

const rust = import('./dist/chardetng_wasm.js')
  .then(module => {
    self.chardetng.detect = module.detect;
    self.chardetng.EncodingDetector = module.EncodingDetector;
    _callbacks.forEach(fn => fn());
    _callbacks = null;
  })
  .catch(console.error);

self.chardetng = {
  onReady
};
