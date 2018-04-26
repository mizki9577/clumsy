const path = require('path')

module.exports = () => ({
  mode: 'development',
  entry: './src/main.js',

  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
        options: {
          presets: ['@babel/preset-env', '@babel/preset-react'],
          plugins: ['@babel/plugin-syntax-dynamic-import'],
        },
      },
    ],
  },

  devtool: 'source-map',
  target: 'web',

  devServer: {
    contentBase: './dist/',
  },
})

// vim: set ts=2 sw=2 et:
