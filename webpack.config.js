module.exports = {
    entry: "./src/app.js",
    output: {
        library: "lib3rd",
        path: "./dist/",
        publicPath: "dist/",
        filename: "3rd.js"
    },
    module: {
        loaders: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                loader: "babel",
                query: {
                    presets: ["es2015", "stage-0"]
                }
            }
        ]
    },
    vue: {
        loaders: {
            js: "babel-loader"
        }
    },
    resolve: {
        alias: {
            "vue$": "vue/dist/vue.common.js"
        }
    }
}
