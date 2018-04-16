const path = require('path')

module.exports = (env={}) => ({
  entry: './main.js',
  mode: env.production ? 'production' : 'development',

  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [
              '@babel/preset-react',
              '@babel/preset-env',
            ],
          },
        },
      },
      {
        test: /\.(html|css)$/,
        use: ['file-loader?name=[path][name].[ext]'],
      },
    ],
  },

  devtool: env.production ? 'source-maps' : 'eval',
})

// vim: set ts=2 sw=2 et:
