const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './es5.js',
  output: {
    path: path.resolve(__dirname, 'dist/es5'),
    filename: 'chardetng.es5.min.js',
  },
  plugins: [
  ],
  experiments: {
    syncWebAssembly: true,
  },
  mode: 'production'
};
