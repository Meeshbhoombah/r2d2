const path        = require('path')
,     webpack     = require('webpack')
,     merge       = require('webpack-merge')
,     baseConfig  = require('./webpack.base.config.js');


module.exports = merge(baseConfig, {
    devtool: 'nosources-source-map',
    
    output: {
        filename    :  'bundle-[hash:8].js'
        path        :  path.resolve(__dirname, 'dist')
        publicPath  :  '/'
    }
    
});

