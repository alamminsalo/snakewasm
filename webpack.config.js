var HtmlWebpackPlugin = require('html-webpack-plugin');
module.exports = {
  entry: './src/index.js',
  output: {
    filename: 'bundle.js',
    path: __dirname + '/dist',
  },
  module: {
    rules: [
	{
		test: /\.css$/,
		use: [ 'style-loader', 'css-loader']
	},
      {
        test: /\.rs$/,
        use: {
          loader: 'rust-wasm-loader',
          options: {
            // The path to the webpack output relative to the project root
            path: ''
          }
        }
      }
    ]
  },
  plugins: [
	  new HtmlWebpackPlugin({
		  title: 'Rust WASM snake',
		  template: 'src/app.html'
	  })
  ],
  // The .wasm 'glue' code generated by Emscripten requires these node builtins,
  // but won't actually use them in a web environment. We tell Webpack to not resolve those
  // require statements since we know we won't need them.
  externals: {
    'fs': true,
    'path': true,
  }
}
