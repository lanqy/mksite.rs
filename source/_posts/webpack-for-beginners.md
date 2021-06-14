---
title: Webpack 给初学者
created: 2018/05/25
description: Webpack 给初学者
author: lanqy
---
# Webpack 给初学者
### Webpack 是什么？
Webpack是一个模块打包和压缩工具
### 什么是模块打包？
模块打包就是把多个Javascript文件打包和压缩成一个单独的Javascript文件
### 怎样使用？
开始创建一个项目
```shell
$ mkdir try-webpack && cd try-webpack && mkdir src && npm init -y
$ touch index.html && touch src/main.js && touch webpack.config.js
```
如果你是Windows用户，可以通过<a href="https://git-scm.com/downloads" target="_blank">git bash</a>运行上面的命令。现在我们开始全局安装webpack:

```javascript
$ npm install -g webpack
```

我们通过一个简单的配置文件开始我们所有的前端项目，打开`webpack.config.js`文件，复制下面的代码粘贴到`webpack.config.js`中

```javascript
module.exports = {
  devtool: 'cheap-module-eval-source-map',
  entry: "./src/main.js",
  output: {
    path: __dirname + "/dist",
    filename: "bundle.js"
  }
};
```

这个文件告诉webpack我们的应用入口点在哪里，以及在哪个目录输出打包后的代码，通过命令行运行`webpack`,在dist目录下就会生成bundle.js文件。到目前为止，我们到应用是相当无聊的，我是<a href="http://mithril.js.org/" target="_blank">Mithril.js</a>的忠实粉丝,因此我们通过<a href="http://mithril.js.org/" target="_blank">Mithril.js</a>来做一些东西，首先通过npm来安装这个库。

```javascript
$ npm install --save mithril
```
现在打开`src/main.js`输入如下代码：

```javascript
var m = require('mithril');

var app = {
  view: function() {
    return m('div', 'hello world!')
  }
}

m.mount(document.getElementById('app'), app)
```
打开index.html,输入如下代码（请记得引入打包后的bundle.js文件）:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Webpack Tutorial</title>
</head>
<body>
  <div id="app"></div>
  <script src="dist/bundle.js"></script>
</body>
</html>
```
接着通过命令行运行`webpack`生成新的包（bundle.js文件），现在在浏览器中打开index.html,你将看到`hello world!` 。
如果你做到来这一步，那么恭喜你！你已经理解基本的webpack了。<br/>

让我们来稍微重构一下我们的app，我们来本地化我们的应用程序，这个要求我们把所有文本放在一个集中的地方，创建一个新的文件`src/resources.json`,这个文件里包含如下的JSON：
```json
{
  "en-US": {
    "HELLO_WORLD": "hello world!"
  }
}
```

回到`src/main.js`文件，通过json文件加载我们的信息（修改该文件为)：

```javascript
var m = require('mithril');
var resources = require('./resources.json')

var app = {
  view: function() {
    return m('div', resources['en-US'].HELLO_WORLD)
  }
}

m.mount(document.getElementById('app'), app)
```

<br/>
如果你迫不及待的想在命令行运行`webpack`，你将会看到类似于如下的错误信息

```javascript
ERROR in ./src/main.js
Module not found: Error: Cannot resolve 'file' or 'directory' ./resources.json in /../../try-webpack/src
 @ ./src/main.js 2:16-43
 ```

这个是loaders(加载器)报的错误，加载器是非常方便的工具，它可以使我们处理不同类型的内容。
<br/>
加载器可以用于转换ES2015成ES5，转换React JSX，编译TypeScript，加载Handlebars模版，加载和执行CSS，编译CSS预处理SASS，等等。有数以百计的加载器为不同的目的而诞生，现在我们只需要关心`json-loader`这个加载器。
<br/>
同样，我们通过npm来安装`json-loader`：
```shell
$ npm install --save-dev json-loader
```
我们需要修改一下我们的`webpack.config.js`文件，引入我们的加载器，具体如下：

```javascript
module.exports = {
  devtool: 'cheap-module-eval-source-map',
  entry: "./src/main.js",
  output: {
    path: __dirname + "/dist",
    filename: "bundle.js"
  },
  module: {
    loaders: [
      { test: /\.json$/, loader: 'json-loader' }
    ]
  }
};
```
再一次运行`webpack`,你的应用应该可以启动和运行了，如果你厌倦了每次输入WebPack，你也可以运行`webpack -w`来激活文件自动检测功能，这样当任何文件修改时自动重新打包。

<br/>
让我们来做最后一件事，您可能已经注意到了生成包是相当大的，对于这样一个简单的应用程序来说，多出来的文件大小时由于`webpack.config.js`文件中`devtool:cheap-module-eval-source-map`这行造成的。这行为我们包生成源码地图（source maps），并映射到原始文件，方便查看源码和调试。当我们发布生产时，我们必须确保没有这行代码，怎样去掉这行取决于你自己，但是现在我们仅仅手工删除这一行代码。让我们也安装本地的webpack，这样我们才可以使用UglifyJsPlugin插件。
<br/>
安装本地webpack

```shell
$ npm install --save-dev webpack
```

当我们准备把项目发布生成时，我们需要压缩打包的代码，Webpack可以帮你完成，让我们最后一次修改`webpack.config.js`文件：

```javascript
var webpack = require('webpack');

module.exports = {
  entry: "./src/main.js",
  output: {
    path: __dirname + "/dist",
    filename: "bundle.js"
  },
  module: {
    loaders: [
      { test: /\.json$/, loader: 'json-loader' }
    ]
  },
  plugins: [
    new webpack.optimize.UglifyJsPlugin({
      compress: {
        warnings: false
      }
    })
  ]
};
```
再一次运行 `webpack` 生成发布生成的压缩文件（bundle.js），如果以上你漏掉来哪一步，请在github上查看<a href="https://github.com/rwhitmire/webpack-demo">webpack-demo</a>,或者在Twitter上随时联系作者<a href="https://twitter.com/ry_js">@ry_js</a>。

注：本文翻译自 <a href="http://rwhitmire.com/2016/04/09/webpack-for-beginners.html">Webpack for Beginners</a>,已经在Twitter上征得原作者的同意。另外为了方便理解，我加上来一些我自己的理解（不是直译），第一次尝试翻译，有不对的地方欢迎指正，谢谢！：）
