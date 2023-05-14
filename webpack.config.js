const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const sveltePreprocess = require("svelte-preprocess");

const path = require('path');


exports.default = {
    mode: 'production',
    entry: ['./app/main.ts', './app/styles.css'],
    resolve: {
        alias: {
            svelte: path.dirname(require.resolve("svelte/package.json")),
        },
        extensions: [".mjs", ".js", ".ts", ".svelte"],
        mainFields: ["svelte", "browser", "module", "main"],
        conditionNames: ['svelte']
    },
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
            },
            {
                test: /\.svelte$/,
                loader: "svelte-loader",
                options: {
                    emitCss: false,
                    hotReload: true,
                    preprocess: sveltePreprocess({
                        tsconfigFile: "tsconfig.json",
                    }),
                    hotOptions: {
                        noPreserveState: true,
                        noReload: false,
                        optimistic: false,
                    },
                },
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