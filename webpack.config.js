const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

exports.default = {
    mode: 'production',
    entry: ['./app/index.ts', './app/styles.css'],
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/
            },
            {
                test: /\.css$/i,
                use: [MiniCssExtractPlugin.loader, 'css-loader']
            }
        ]
    },
    experiments: {
        asyncWebAssembly: true,
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: __dirname,
            watchDirectories: ['./lib']
        }),
        new MiniCssExtractPlugin(),
        new HtmlWebpackPlugin({
            template: './app/index.html',
            favicon: './app/assets/favicon.png'
        })
    ]
};