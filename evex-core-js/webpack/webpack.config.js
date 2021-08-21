const path = require('path');

module.exports = {
    context: __dirname,
    mode: 'production',
    devtool: 'inline-source-map',
    entry: '../src/index.ts',
    target: "web",
    node: {
        __dirname: false,
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            }
        ]
    },
    resolve: {
        extensions: [ '.ts' ],
        fallback: {
            fs: false,
        },
    },
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, "..", "bin"),
    },
};