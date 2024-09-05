const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "docs"),
    filename: "index.js",
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html",
    }),
  ],
  mode: "production",
  experiments: {
    asyncWebAssembly: true,
  },
  performance: {
    hints: false,
  },
};
