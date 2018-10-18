const path                = require('path')
,     webpack             = require('webpack')
,     HTMLWebpackPlugin   = require('html-webpack-plguin');


basePath = process.cwd();


module.exports = {

  resolve: {
    extensions: ['.js', '.jsx'],
    alias: {
      /* TODO: Aliases for Modules */
      src: path.resolve(basePath, 'src')
    }
  },

  
  module: {
    rules: [
      /* TODO: styles/css 
        babel -> source-map */
    ]
  }

  plugins: [
    new webpack.NoEmitOnErrorsPlugin(),
    new HTMLWebpackPlugin({
      template: 'src/index.html'
    }),
    new webpack.DefinePlugin({
        'VERSION': JSON.stringify(require('./package.json').version)
    })
  ]
  
};

