const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async"  // Tell webpack how to handle .wasm files
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: 'index.html' }]
    })
  ],
  devServer: {
    static: {
      directory: path.join(__dirname),
    },
    port: 8080,
  },
  resolve: {
    extensions: [".js", ".wasm"],
  },
};
