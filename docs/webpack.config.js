module.exports = {
  entry: './main.js',
  output: {
    path: __dirname,
    filename: 'bundle.js',
  },
  mode: 'development',
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [
              '@babel/preset-env',
              '@babel/preset-react',
            ],
            plugins: [
              '@babel/plugin-syntax-dynamic-import',
            ],
          },
        },
      },
    ],
  },
}

// vim: set ts=2 sw=2 et:
