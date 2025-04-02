const path = require("path");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./src/index.js",
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      title: "actix-web-static-files WebPack",
    }),
  ],
  output: {
    filename: "main.js",
    path: process.env.OUT_DIR
      ? path.resolve(process.env.OUT_DIR, "web", "dist", "bundle")
      : path.resolve(__dirname, "dist", "bundle"),
  },
};
