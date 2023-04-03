const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackWatcherPlugin = require('wasm-pack-watcher-plugin');
const path = require('path');

module.exports = {
    mode: 'development',
    entry: './app/index.ts',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    experiments: {
        asyncWebAssembly: true,
    },
    watchOptions: {
        poll: 40
    },
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'app/index.html',
            favicon: 'app/c.png',
        }),
        new WasmPackWatcherPlugin({
            sourceRoot: path.resolve(__dirname, 'lib'),
            crateRoot: path.resolve(__dirname),
            mode: "release"
        })
    ],
};