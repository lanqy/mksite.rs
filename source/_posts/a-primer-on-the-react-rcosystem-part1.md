---
title: React 生态系统入门第一部分（共三个部分）
created: 2016/07/14
description: React 生态系统入门第一部分（共三个部分）
author: lanqy
---

# React1 生态系统入门第一部分（共三个部分）


译自: http://patternhatch.com/2016/07/06/a-primer-on-the-react-ecosystem-part-1-of-3/


- <a href="#user-content-update">更新</a>
- <a href="#user-content-introduction">介绍</a>
- <a href="#user-content-installment">这一期中</a>
- <a href="#user-content-prerequisite">先决条件</a>
- <a href="#user-content-code">代码</a>
- <a href="#user-content-description">项目描述</a>
- <a href="#user-content-creation">创建项目</a>
- <a href="#user-content-webpack">Webpack</a>
- <a href="#user-content-babel">Babel</a>
- <a href="#user-content-hot">热模块更换</a>
- <a href="#user-content-component">第一个React组件</a>

<h4 id="user-content-update">更新</h4>
<p>2016.07.06: 第一次发布</p>
<h4 id="user-content-introduction">介绍</h4>
<p>过去8个月，我一直在工作中使用React生态系统，我们创建应用来监控我们的交易状态，编辑我们的交易风险检查和控制我们的战略。</p>
<p>它证明了梦幻般的生态系统，鉴于其组合的性质和平易近人的学习曲线，React几乎已经成为提供一个消费级的企业级用户界面的有趣交付物。</p>
<p>我想通过一个入门的教程 分享我的学习经验，通过这个入门你将学习到：</p>

- 配置开发React应用的环境 
- 创建React组件并让他们对数据变化做出反应
- 通过Redux管理应用的状态

<p>如果你是第一次接触React生态系统，我希望通过这个入门教程，你将很轻松的开始创建你的React应用</p>
<h4 id="user-content-installment">在这一期中</h4>
<p>在第一部分中，我们将会配置一个基本的React开发环境，在文章的末尾，你将拥有一个创建应用和动态重载的开发环境，当你的文件发生改变的时候，我们也会创建我们的的第一个React组件
</p>

<h4 id="user-content-prerequisite">先决条件</h4>
<p>我假设你已经熟悉Node和NPM并且已经安装好这两个工具。你还应该熟悉(Javascript)ES6。你不需要是ES6专家，但是知道主要的新功能如箭头函数和解构赋值。</p>
<p>写这篇文章的时候，我使用的是Node6.2.0和NPM3.8.9版本</p>
<h4 id="user-content-code">代码</h4>
<p>你可以在这里找到第一部分的所有代码，每个部分将有自己的分支</p>
<h4 id="user-content-description">项目描述</h4>
<p>我们将要创建一个应用依托于Spotify的后台，我们的应用将允许用户：</p>

- 检索艺术家的专辑
- 检索给定专辑的曲目
- 播放曲目开头30秒

<p>以下是一个样本：</p>
<img src="/images/mockup.png" />
<p>使用Spotify的后台，这样我们可以把更多的注意力集中放在前端开发上。</p>

<h4 id="user-content-creation">创建项目</h4>
<p>让我们开始创建我们的项目</p>

```js
mkdir respotify
cd respotify
npm init -y
```
<p>这个将在我们的根目录下创建一个package.json文件</p>
<p>下一步，在根目录下创建一个src目录,在src目录中创建一个index.html文件：</p>

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Respotify</title>
  </head>
  <body>
    <h1>Respotify</h1>
    <div id="user-content-container"></div>
    <script src="/bundle.js"></script>
  </body>
</html>
```
<p>index.html是我们应用的入口，其中有两行代码比较有趣:</p>

```html
 <div id="user-content-container"></div>
  <script src="/bundle.js"></script>
```
<p>我们的React组件将渲染到id为container的div中，这个将会更加清晰，通过这个编文章</p>
<p>构建的时候，我们要放在一起将所有的JS文件合并成一个文件bundle.js，并且在index.html引用它</p>
<p>下一步，创建index.js：</p>

```js
console.log("Hello,world!");
```
<p>index.js是我们的应用程序的主要的Javascript文件，现在，我们仅仅打印一些日志，以确保我们的构建正常工作。</p>

这个时候，我们的目录结构应该像这样：

```js
respotify
 -src
  --index.html
  --index.js
 --package.json
```

<h4 id="user-content-webpack">Webpack</h4>
<p>现在好了，我们将从头开始组建我们的构建环境，你可能听说过很多开发样本，例如<a href="https://github.com/kriasoft/react-starter-kit">这里</a>提供了完整的开发配置环境，虽然这个很有用，但是我觉得至少自己会创建基本的配置，这样你才知道主要的东西是如何工作的</p>
<p>我们从<a href="https://webpack.github.io/">Webpack</a>开始构建我们的环境，简单地说，Webpack为我们打包静态资源，虽然有很多不同类型的打包工具，但是Webpack提供的一些功能，获得React社区的青睐，我们只是使用其中的一部分。</p>
<p>让我们开始安装Webpack：</p>

```js
npm install webpack --save-dev
```
<p>一旦安装完整，检查package.json文件Webpack是否在你的dev依赖。</p>
<p>我们使用Webpack做两件事：</p>
- 获取我们的应用程序代码，并从代码中生成静态资源
- 启动一个开发服务器来服务静态资源

<p>因为Webpack是配置驱动的，我们就从这里开始，在根目录下创建一个webpack.config.js文件：</p>

```js
const webpack = require('webpack'); // line 1
const path = require('path');// line 2
 // line 3
const PATHS = { // line 4
  app: './src/index.js', // line 5
  dist: path.join(__dirname, 'dist') // line 6
}; // line 7
 // line 8
module.exports = { // line 9
  entry: { // line 10
    javascript: PATHS.app // line 11
  }, // line 12
  output: { // line 13
    path: PATHS.dist, // line 14
    publicPath: '/', // line 15
    filename: 'bundle.js' // line 16
  }, // line 17
  devServer: { // line 18
    contentBase: PATHS.dist // line 19
  } // line 20
}; // line 21
```
<p>让我们继续</p>
<p>在webpack.config.js文件顶部，我们引用webpack和path模块，然后为我们的应用程序定义几个常量</p>
<p>[第9行] 这个是我们Webpack配置的开始</p>
<p>[第10行] 这里最终成为我们bundle.js文件的<a href="https://webpack.github.io/docs/configuration.html#entry">入口</a>，在这个例子中，我们有一个单独的JS文件（index.js）服务，做为我们的入口文件，Webpack将构建index.js以及任何其他JS文件在index.js中声明依赖的，与依赖那些依赖等，最终合成一个单一的bundle.js文件。想象一下一个依赖折叠成一个节点的图形。</p>
<p>[第13行] <a href="https://webpack.github.io/docs/configuration.html#output">这里</a>是我们设置的配置文件。在这个例子中，我们告诉Webpack：</p>
- 输出其编译结果到dist目录下
- 将输出的文件的网址设置为首页路径
- 调用其编译的bundle.js结果

<p>[第18行] 这里我们制定Webpack服务器从dist目录下请求静态资源目录。</p>
<p>现在尝试在您的终端运行此命令（确保您在您的项目的根目录和节点在您的路径下）：</p>

```js
node node_modules/webpack/bin/webpack.js
```
<p>你将看到已经在根目录下创建一个dist目录，目录中有一个bundle.js文件在里面。</p>

<p>如果你看看bundle.js,你将看到一些Webpack特定的方法和一个console.log声明在文件底部。</p>

<p>这个时候，您的项目目录结构看起来应该像这样：</p>

```js
Respotify
-dist
 --bundle.js
-node_modules
-src
 --index.html
 --index.js
--package.json
--webpack.config.js
```
<p>现在我们也需要提供引用bundle.js的index.html文件，这个文件目前不在我们的dist目录下，因此我们需要复制index.html到dist目录下。</p>
<p>要实现这个，我们需要安装文件加载程序包，安装方式如下：</p>

```js
npm install file-loader --save-dev
```
<p>我们将修改webpack.config.js文件来引用index.html文件，我们还将包括一个模块对象来指定我们的第一个<a href="https://webpack.github.io/docs/loaders.html">加载程序</a>。本质上，加载器就是加载或预编译运行的文件，在这种情况下，我们使用文件<a href="https://github.com/webpack/file-loader">加载</a>复制index.html来输出（dist）目录，添加以下高亮的行到webpack.config.js文件中：</p>

```js
const webpack = require('webpack');
const path = require('path');
 
const PATHS = {
  app: './src/index.js',
  html: './src/index.html', // 高亮行
  dist: path.join(__dirname, 'dist')
};
 
module.exports = {
  entry: {
    javascript: PATHS.app,
    html: PATHS.html // 高亮行
  },
  output: {
    path: PATHS.dist,
    publicPath: '/',
    filename: 'bundle.js'
  },
  devServer: {
    contentBase: PATHS.dist
  },
  // 高亮部分
  module: { 
    loaders: [
      {
        test: /\.html$/,
        loader: "file?name=[name].[ext]"
      }
    ]
  }
  // 高亮部分
};
```

<p>现在我们再次运行之前的命令：</p>

```js
node node_modules/webpack/bin/webpack.js
```

你应该看到index.html已经复制到dist目录项了，dist目录现在应像这样：

```js
dist
--bundle.js
--index.html
```

我们在开发的过程中，我们需要重新编译bundle.js，每次修改代码都要重新在命令行编译Webpack必然很乏味，如果我们要创建一个在我们修改代码的时候可以自动编译和刷新浏览器实时看到效果，我们该怎样做呢？

这个时候就应该使用<a href="https://webpack.github.io/docs/webpack-dev-server.html">Webpack dev server</a>了,让我们来安装它：

```js
npm install webpack-dev-server --save-dev
```

一旦安装完成，我们运行一下命令：

```js
node node_modules/webpack-dev-server/bin/webpack-dev-server.js
```

这将启动开发服务器在:<a href="http://localhost:8080/webpack-dev-server/">http://localhost:8080/webpack-dev-server/</a>

如果您访问这个链接，你应该会看到我们的应用：

<img src="/images/respotify_initial_dev.png" />

试着去修改index.html文件中的h1标签并且保存它，webpack开发服务器将自动检测并重新加载。每次重新加载你也会看到控制台输出日志。因此现在我们有一个webpack服务器，服务我们的静态资源和重新加载当有代码有修改的时候。

让我们完善一下我们的package.json文件，创建一些脚本，以至于我们更方便处理一些事情。

添加以下高亮行package.json的脚本对象中：

```js
"scripts": {
  "start": "node node_modules/webpack-dev-server/bin/webpack-dev-server.js", // 高亮
  "build": "node node_modules/webpack/bin/webpack.js", // 高亮
  "test": "echo \"Error: no test specified\" && exit 1"
},
```
如果你没有停止webpack开发服务器，停掉它然后运行用一下命令运行Webpack开发服务：

```js
npm run start
```

使用一下命令构建:

```js
npm run build
```

这个时候，我们基本的构建已经完成，下一步，我们将扩大我们的构建过程，允许我们使用最新的ES6功能。


<h4 id="user-content-babel">Babel</h4>

<a href="https://kangax.github.io/compat-table/es6/">ES6支持</a>各种不同的浏览器，因此这样使用所有的ES6功能，不需要考虑各个浏览器的兼容性呢？一个解决方案就是把ES6代码转成ES5，这正是我们接下来要做的。

我们将使用<a href="http://babeljs.io/">Babel</a>来做这个事情,我们需要Babel来做两件事：

- 转ES6代码到ES5
- 转<a href="https://facebook.github.io/react/docs/jsx-in-depth.html">JSX</a>成JavaScript。JSX是React DSL，类似HTML。你将学习它，当你开始React开发的时候。

开始安装这些Babel包：

```js
npm install babel-core babel-loader babel-preset-es2015 babel-preset-react --save-dev
```

让我们看看每个包都干嘛的：

- <a href="https://github.com/babel/babel/tree/master/packages/babel-core">babel-core</a> Babel核心编译器。
- <a href="https://github.com/babel/babel-loader">babel-loader</a> Webpack的Babel加载器。
- <a href="https://babeljs.io/docs/plugins/preset-es2015/">preset-es2015</a> 一套ES6转ES5的Babel插件。
- <a href="https://babeljs.io/docs/plugins/preset-react/">babel-preset-react</a> 一套JSX转成JS的Babel插件。

在开始结合Babel之前，让我们来改改index.js，让它包含以下ES6代码，修改之后看起来像这样：

```js
const greeting = (name) => {
  console.log(`Hello, ${name}!`);
};
greeting('world');
```

现在执行 npm run build命令，看看文件bundle.js底部，你应该可以看到之前定义的原始的ES6方法，没有任何变化，考虑到这一点，让我们结合Babel。

第一，我们将添加babel-loader到我们的webpack.config.js文件中，回想一下，我们如何使用文件加载器将index.html文件复制到dist目录中，我们现在为Babel添加第二个加载对象，添加一下高亮行到你到module对象中：

```js
module: {
  loaders: [
    {
      test: /\.html$/,
      loader: "file?name=[name].[ext]"
    },
    //高亮部分
    { 
      test: /\.js$/,
      exclude: /node_modules/,
      loaders: ["babel-loader"]
    }
    //高亮部分
  ]
}

```

这里，我们要求Webpack应用babel-loader在所有以.js为后缀到并且不在node_modules文件夹中的文件。

下一步，我们在项目根目录中创建一个.babelrc文件，内容如下：

```js
{
"presets":["react","es2015"]
}
```

这里要求Babel使用前面我们安装的<a href=" presets"> presets</a>。

就这样。

再一次执行 npm run build 和检查bundle.js，我们的greeting 方法现在应该转成了ES5。

好，我们几乎已经完成了，下一步我们要做的就是重新加载React组件，当我们做改动的时候，不会丢失状态信息。

<h4 id="user-content-hot">热模块更换</h4>

让我们开始安装<a href="https://facebook.github.io/react/">React</a>和<a href="https://facebook.github.io/react/docs/top-level-api.html#reactdom">ReactDOM</a>，以及<a href="https://gaearon.github.io/react-hot-loader/">React Hot Loader</a>。注意我们安装React和ReactDOM作为正常的依赖，而React Hot Loader作为开发依赖。安装方式如下：

```js
npm install react react-dom --save
```

```js
npm install react-hot-loader --save-dev
```
下一步，我们让webpack使用热更换，打开webpack.config.js修改babel-loader像这样：

```js
loaders:["react-hot","babel-loader"]
```
重要的是，React-hot被添加为webpack过程从右到左的之前。

下一步，打开package.json编辑 start npm脚本如下：

```js
"start": "node node_modules/webpack-dev-server/bin/webpack-dev-server.js --hot --inline",
```

在这一点上，我们进一步加强我们的开发配置重新加载React组件而状态不丢失，但是这个意味着什么呢？要回答这一问题，我们需要创建一个React组件来测试我们的配置。

<h4 id="user-content-component">我们的第一个React组件</h4>

在src目录下创建一个greeting.js:

```js
import React from "react"; //第1行
 
export default React.createClass({ //第3行
  render: function() {//第4行
    return ( //第5行
      <div className="greeting">
        Hello, {this.props.name}!
      </div>
    ); //第9行
  }
});
```

我们刚刚创建我们的第一个React组件，让我们一行一行来看。

［第一行］在顶部，我们导入React。

［第三行］我们通过<a href="https://facebook.github.io/react/docs/top-level-api.html#react.createclass">React.createClass方法</a>创建一个React组件类，传入一个描述我们组件的规范对象。

［第四行］每个React组件都必须有一个render方法，它返回组件的标记。

［第五－九行］在render方法里，我们返回一个类HTML 包含一个class名称为gretting的div 包裹这一个字符串，字符串本身把Hello和一些引用值连接起来。

我们将继续深入理解第五－九行，但是现在，让我们先把这个组件在屏幕上展示出来。

打开index.js修改它如下:

```js
import React from "react";
import ReactDOM from "react-dom";
import Greeting from './greeting';
 
ReactDOM.render(
  <Greeting name="World"/>,
  document.getElementById('container')
);
```
我们再一次从导入React开始，接着我们导入ReactDOM，用于渲染DOM的API接口，我们也导入我们的greeting component。

当我们调用ReactDOM的render方法时，我们传入两个参数：

- 1.我们需要渲染的组件。
- 2.我们要把组件渲染到的位置。

在我们的例子中，我们将渲染我们的greeting组件到一个id为container的div中，这个div在我们的index.html中。

注意第一个参数是怎么样子的，我们传入greeting组件，它好像是一个HTML元素，它包含了一个键值对，其中键的名字为name，值为world。然后name就成为我们greeting组件的属性。

大体上，我们可以这么描述index.js：

<strong>渲染greeting组件，把world赋给它的name属性，把这个组件渲染到id为container的div上。</strong>

现在，当我们渲染greeting组件时，我们将接收name属性和包含它的值的字符串，重启开发服务并验证这个组件渲染：

<img src="/images/respotify_first_component.png" />

测试它的重载，试着用h1标签包裹字符串，如下：

```js
<h1>Hello, {this.props.name}!</h1>
```

保存它，你将看到组件局部更新，而不需要全部重新加载。

现在，让我们退后一步，重新看greeting.js代码片段：

```js
 return (
      <div className="greeting">
        Hello, {this.props.name}!
      </div>
    );
```

我们返回的标记实际上是JSX，一个React特定的DSL，就像HTML用方便描述树结构。我们的Babel配置会把JSX转换成等效的纯JavaScript。有两个重要的概念要记住：

- JSX不是JavaScript的扩展。
- JSX不是必须的，你可以使用纯JavaScript的调用方式代替。

要展示第二点，我们使用<a href="http://babeljs.io/repl/"> Babel REPL</a>,在REPL中，确保选中ES2015 和 React preset这两个选项，复制greeting.js的内容到左侧的编辑器中，你会看到转换输出到右侧，一个代码片段类似这样：

```js
<div className="greeting" />
```

将转换为：

```js
_react2.default.createElement("div", { className: "greeting" });
```

以上的纯JavaScript你可以直接使用，然而，我强烈推荐使用JSX，这个让你的React组件方便阅读和书写。事实上，它看起来像HTML也有助于减少学习曲线，当然，由于JSX是JavaScript，跟HTML有一些细微的差异，比如JSX的className和HTML的class，我们将深入理解更多JSX的功能，可以去看看Facebook的指南了解更多的东西。

让我们把注意力放在div中的这行代码：

```js
 Hello, {this.props.name}!
```

在JSX中，用花括号括起来的是一个JavaScript表达式，因此，在这里，我们访问这个组件的props对象，反过来，在props上访问name属性。

还记得在index.js中的这一行代码吗？

```js
<Greeting name="World"/>
```

当你实例化一个组件，你可以传入大量的表示该组件状态的键值对。因此，当我们实例化我们的greeting组件，我们也应该指定一个名为name的状态。这个值的状态叫做“world”。


在实际的greeting组件中，该状态通过组件的props对象和使用相同的name标签来传递。花括号里的值被计算，并且返回“Hello，world”渲染到浏览器中。

记住主要的一点，就是props是不可变的。换句话说，一旦状态被传递到一个组件，接受组件就不能再改变它了。这样有助于你的组件更容易理解和调试。以及将状态管理分离到几个模块中。

试着在index.js中修改name的值。你将看到Webpack检测变化和自动加载，greeting组件将接收到新的状态，并作出响应。


如果你跟着一起做下来，并且一切都正常工作，那么恭喜你！你刚刚构建了一个基于打包、转换、热更新的React应用开发环境。此外，你还创建了一个React组件，记住，你可以添加更多的环境，比如压缩文件，选择性打包，启用和禁用各种开发工具。但是对于我们而言，这样已经够了。

在下一部分中，我们将进一步创建更多的React组件来完善我们的Spotify客户端，我们还将添加静态分析来保持我们的代码整洁明了。

预计下一部分会在这一期发布之后的两个星期内完成。在此之前，如果你有任何意见和建议，请在下面留言或者给我发<a href="http://patternhatch.com/about/">邮件</a>。



