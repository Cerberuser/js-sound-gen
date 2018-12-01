// webpack.config.js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: "./main.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
        globalObject: 'this',
    },
    resolve: {
        extensions: ['.js', '.wasm']
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: "Getting started with WASM"
        })
    ],
    mode: "none"
};