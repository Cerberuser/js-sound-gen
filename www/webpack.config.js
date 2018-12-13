// // webpack.config.js
// const path = require('path');
// const HtmlWebpackPlugin = require('html-webpack-plugin');
//
// module.exports = {
//     entry: "./main.js",
//     output: {
//         path: path.resolve(__dirname, "dist"),
//         filename: "index.js",
//         globalObject: 'this',
//     },
//     resolve: {
//         extensions: ['.js', '.wasm']
//     },
//     plugins: [
//         new HtmlWebpackPlugin({
//             title: "Getting started with WASM"
//         })
//     ],
//     mode: "none"
// };

const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin');

const browserConfig = {
    entry: './main.js',
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
        globalObject: 'this',
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: "Getting started with WASM"
        })
    ],
    mode: "none"
};

const workerConfig = {
    entry: "./gen-worker.js",
    target: 'webworker',
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "gen-worker.js"
    },
    resolve: {
        extensions: ['.js', '.wasm']
    },
    mode: "none",
};

module.exports = [browserConfig, workerConfig];