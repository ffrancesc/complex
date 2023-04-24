const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackWatcherPlugin = require('wasm-pack-watcher-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const path = require('path');

exports.default = {
    mode: 'development',
    entry: ['./app/index.ts', './app/styles.css'],
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader']
            },
        ],
    },
    experiments: {
        asyncWebAssembly: true,
    },
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new WasmPackWatcherPlugin({
            sourceRoot: path.resolve(__dirname, 'lib'),
            crateRoot: path.resolve(__dirname),
            mode: 'dev'
        }),
        new HtmlWebpackPlugin({
            template: './app/index.html',
            favicon: './app/assets/favicon.png',
        }),
        new MiniCssExtractPlugin()
    ]
};