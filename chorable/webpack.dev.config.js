const path        = require('path')
,     webpack     = require('webpack')
,     merge       = require('webpack-merge');


const baseConfig  = require('./webpack.base.config.js');


module.exports = merge(baseConfig, {
  mode: 'development',

  devtool: 'cheap-module-eval-sourcemap',

  devServer: {
    contentBase: path.resolve(__dirname, 'src'),
    hot: true,
    publicPath: '/',
    historyApiFallback: true,
    port: 3000
  },
    
  output: {
    filename    :  'bundle.js',
    path        :  path.resolve(__dirname, 'dist'),
    publicPath  :  '/'
  },

  entry: [
    // activate HMR for React
    'react-hot-loader/patch',

    // bundle the client for webpack-dev-server
    // and connect to the provided endpoint
    'webpack-dev-server/client?http://localhost:3000',

    // bundle the client for hot reloading
    // only- means to only hot reload for successful updates
    'webpack/hot/only-dev-server',

    // the entry point of our app
    __dirname + '/src/index.js',
  ],

  plugins: [
    new webpack.HotModuleReplacementPlugin(),
    new webpack.NamedModulesPlugin()
  ]
});

