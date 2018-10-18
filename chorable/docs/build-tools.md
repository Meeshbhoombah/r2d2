# Build Tools

## Todos
- [ ] Install...
    + [ ] webpack
    + [ ] webpack-dev-server (dev)
    + [ ] style-loader/css-loader
    + [ ] babel-loader
        * [ ] babel-preset-env
    + [ ] html-webpack-plugin (dev)
- [ ] webpack.base.config
    + [ ] devtool
    + [ ] Module aliases
- [ ] webpack.dev.config
    + [ ] Hot Module Reloader

## Webpack
- [When and Why to use Webpack](https://blog.andrewray.me/webpack-when-to-use-and-why/)

### Loaders
**style-loader**
Add css to DOM by injecting a `<style>` tag.
- [Webpack Docs](https://webpack.js.org/loaders/style-loader/)

**css-loader**
Interpret `@import` and `url()`.
- [Webpack Docs](https://webpack.js.org/loaders/css-loader/)

**babel-loader**
Transpile ES6 and React to ES5 with `@babel/preset-env`.
- [Webpack Docs](https://webpack.js.org/loaders/babel-loader/)
- [Babel Usage Guide](https://babeljs.io/docs/en/usage)

### Plugins
**HTMLWebpackPlugin**
Automatically builds `index.html` with bundled assets
- [Webpack Docs](https://webpack.js.org/plugins/html-webpack-plugin/)

## References
- [react-spa-template](https://github.com/meetajhu/react-spa-template)
- [Alchemy Base Config](https://github.com/daostack/alchemy/blob/dev/webpack.dev.config.js)
- [Alchemy Dev Config](https://github.com/daostack/alchemy/blob/dev/webpack.dev.config.js)

