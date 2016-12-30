module.exports = {
    entry: "./src/app.js",
    output: {
        path: "./dist",
        publicPath: "dist/",
        filename: "3rd.js"
    },
    module: {
        loaders: [
            {
                test: /\.js$/,
                loader: "babel",
                exclude: /node_modules/
            }
        ]
    },
    resolve: {
        alias: {
            "vue$": "vue/dist/vue.common.js"
        }
    }
}
