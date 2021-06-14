---
title: 用 Javascript 编写一个编译器
created: 2018/05/21
description: 用 Javascript 编写一个编译器
author: lanqy
---
# 用 Javascript 编写一个编译器

译自： https://medium.com/@kosamari/how-to-be-a-compiler-make-a-compiler-with-javascript-4a8a13d473b4#.evrubxdub

在布鲁克林 布什威克的一个美好的星期天，我在本地书店发现了一本John Maeda写的“数字设计”的书，这本书里一步一步的教DBN编程语言--一种90年代末在麻省理工学院媒体实验室诞生的语言，旨在以可视化的方式介绍计算机编程概念。
<img src="/images/dbn.png" />

DNB 代码示例 来自 http://dbn.media.mit.edu/introduction.html

我迫切地想让SVG来实现DBN并运行它在浏览器在2016年将是一个有趣的项目,而不是安装Java环境执行原始DBN源代码。

我想我需要些一个DBN到SVG的编译器，所以编写一个编译器的任务已经开始。 “写一个编译器”听起来像很多计算机科学...但我从来没有接触过这方面，我可以写一个编译器吗？

### 让我们先试着做一个编译器

编译器是一种采用一段代码并将其转换为其他代码的机制。 让我们将简单的DBN代码编译成一个物理图。

在这个DBN代码中有3个命令，“Paper”定义纸张的颜色，“Pen”定义笔的颜色，“Line”画一条线。 颜色参数100表示CSS中的100％黑色或rgb（0％，0％，0％）。 在DBN中产生的图像总是灰度的。 在DBN中，纸张始终为100×100，线宽始终为1，并且线由起点的x y坐标和从左下角计数的终止点定义。

让我们试着成为一个编译器自己。 停在这里，抓住一张纸和一支钢笔，尝试编译以下代码作为绘图。

```js
Paper 0
Pen 100
Line 0 50 100 50
```

你在中间从左侧到右侧画了一条黑线吗？ 恭喜！ 你刚刚实现了一个编译器。

<img src="/images/1-aDJskliFHSIIfYhr8aN3UA.png" />
<p style="color:gray; font-size: 80%; text-align: center;">编译结果</p>

### 编译器是怎样工作的？

让我们来看看刚才在编译器中发生了什么。

#### 1.词汇分析（标记化）

我们做的第一件事是用空格分隔每个关键字（称为tokens）。 当我们分离单词时，我们还为每个标记分配了原始类型，如“word”或“number”。

<img src="/images/1-lM4hjuI28Dodn-DfnXQu4A.png" />

#### 2.解析（语法分析）

一旦一个文本块被分成tokens，我们就经历了每一个文本，并试图找到tokens之间的关系。
在这种情况下，我们将与command关键字相关联的数字分组在一起。 通过这样做，我们开始看到代码的结构。

<img src="/images/1-Masaunh04PyclWIGhztHmg.png" />

#### 3.转换

一旦我们通过解析分析语法，我们将结构转换为适合最终结果的结构。 在这个案例中，我们将绘制一个图像，所以我们将把它一步一步转换为人类的语言。

<img src="/images/1-ExV6vUNKZ4-IpG15-CAeFw.png" alt="" />

#### 4.代码生成

最后，我们制作一个编译结果，一个绘图。 在这一点上，我们只是按照我们在上一步中绘制的指令。

<img src="/images/1-250m-6zI6slTBirOxHX7kw.png" alt="" />

这是编译器做的！

我们做的图是编译结果（当你编译C代码时像.exe文件）。 我们可以把这张图纸传递给任何人或任何设备（扫描仪，相机等）来“运行”，每个人（或设备）都会在中间看到一条黑线。

### 让我们写一个编译器

现在我们知道编译器如何工作，让我们在JavaScript中做一个。 此编译器使用DBN代码并将其转换为SVG代码。

#### 1.Lexer方法

正如我们可以将英语句子“I have a pen”分割为[I，have，a，pen]，词法分析器将一个代码字符串拆分成小的有意义的块（token）。 在DBN中，每个token由空格分隔，并分类为“word”或“number”。

```js
function lexer (code) {
  return code.split(/\s+/)
          .filter(function (t) { return t.length > 0 })
          .map(function (t) {
            return isNaN(t)
                    ? {type: 'word', value: t}
                    : {type: 'number', value: t}
          })
}
```

```js
输入: "Paper 100"
输出:[
  { type: "word", value: "Paper" }, { type: "number", value: 100 }
]
```

#### 2.Parser方法

解析器通过每个标记，找到语法信息，并构建一个称为AST（抽象语法树）的对象。 你可以把AST看作我们代码的映射 - 一种理解一段代码结构的方法。
在我们的代码中，有2种语法类型“NumberLiteral”和“CallExpression”。 NumberLiteral表示该值是一个数字。 它用作CallExpression的参数。

```js
function parser (tokens) {
  var AST = {
    type: 'Drawing',
    body: []
  }
  // extract a token at a time as current_token. Loop until we are out of tokens.
  while (tokens.length > 0){
    var current_token = tokens.shift()

    // Since number token does not do anything by it self, we only analyze syntax when we find a word.
    if (current_token.type === 'word') {
      switch (current_token.value) {
        case 'Paper' :
          var expression = {
            type: 'CallExpression',
            name: 'Paper',
            arguments: []
          }
          // if current token is CallExpression of type Paper, next token should be color argument
          var argument = tokens.shift()
          if(argument.type === 'number') {
            expression.arguments.push({  // add argument information to expression object
              type: 'NumberLiteral',
              value: argument.value
            })
            AST.body.push(expression)    // push the expression object to body of our AST
          } else {
            throw 'Paper command must be followed by a number.'
          }
          break
        case 'Pen' :
          ...
        case 'Line':
          ...
      }
    }
  }
  return AST
}
```

```js
输入: [
  { type: "word", value: "Paper" }, { type: "number", value: 100 }
]
输出: {
  "type": "Drawing",
  "body": [{
    "type": "CallExpression",
    "name": "Paper",
    "arguments": [{ "type": "NumberLiteral", "value": "100" }]
  }]
}
```

#### 3. Transformer 方法

我们在前面的步骤中创建的AST很好地描述代码中发生了什么，但是它没有用于创建SVG文件。
例如。 “纸张”是一个只存在于DBN范例中的概念。 在SVG中，我们可以使用<rect>元素来表示一个Paper。 变换函数将AST转换为另一个支持SVG的AST。

```js
function transformer (ast) {
  var svg_ast = {
    tag : 'svg',
    attr: {
      width: 100, height: 100, viewBox: '0 0 100 100',
      xmlns: 'http://www.w3.org/2000/svg', version: '1.1'
    },
    body:[]
  }

  var pen_color = 100 // default pen color is black

  // Extract a call expression at a time as `node`. Loop until we are out of expressions in body.
  while (ast.body.length > 0) {
    var node = ast.body.shift()
    switch (node.name) {
      case 'Paper' :
        var paper_color = 100 - node.arguments[0].value
        svg_ast.body.push({ // add rect element information to svg_ast's body
          tag : 'rect',
          attr : {
            x: 0, y: 0,
            width: 100, height:100,
            fill: 'rgb(' + paper_color + '%,' + paper_color + '%,' + paper_color + '%)'
          }
        })
        break
      case 'Pen':
        pen_color = 100 - node.arguments[0].value // keep current pen color in `pen_color` variable
        break
      case 'Line':
        ...
    }
  }
  return svg_ast
 }
```

```js
输入: {
  "type": "Drawing",
  "body": [{
    "type": "CallExpression",
    "name": "Paper",
    "arguments": [{ "type": "NumberLiteral", "value": "100" }]
  }]
}
输出: {
  "tag": "svg",
  "attr": {
    "width": 100,
    "height": 100,
    "viewBox": "0 0 100 100",
    "xmlns": "http://www.w3.org/2000/svg",
    "version": "1.1"
  },
  "body": [{
    "tag": "rect",
    "attr": {
      "x": 0,
      "y": 0,
      "width": 100,
      "height": 100,
      "fill": "rgb(0%, 0%, 0%)"
    }
  }]
}
```
#### 4. Generator 函数

作为这个编译器的最后一步，generator函数创建基于我们在上一步中创建的新AST的SVG代码。

```js
function generator (svg_ast) {

  // create attributes string out of attr object
  // { "width": 100, "height": 100 } becomes 'width="100" height="100"'
  function createAttrString (attr) {
    return Object.keys(attr).map(function (key){
      return key + '="' + attr[key] + '"'
    }).join(' ')
  }

  // top node is always <svg>. Create attributes string for svg tag
  var svg_attr = createAttrString(svg_ast.attr)

  // for each elements in the body of svg_ast, generate svg tag
  var elements = svg_ast.body.map(function (node) {
    return '<' + node.tag + ' ' + createAttrString(node.attr) + '></' + node.tag + '>'
  }).join('\n\t')

  // wrap with open and close svg tag to complete SVG code
  return '<svg '+ svg_attr +'>\n' + elements + '\n</svg>'
}
```

```js
输入: {
  "tag": "svg",
  "attr": {
    "width": 100,
    "height": 100,
    "viewBox": "0 0 100 100",
    "xmlns": "http://www.w3.org/2000/svg",
    "version": "1.1"
  },
  "body": [{
    "tag": "rect",
    "attr": {
      "x": 0,
      "y": 0,
      "width": 100,
      "height": 100,
      "fill": "rgb(0%, 0%, 0%)"
    }
  }]
}
输出:
<svg width="100" height="100" viewBox="0 0 100 100" version="1.1" xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="100" height="100" fill="rgb(0%, 0%, 0%)">
  </rect>
</svg>
```

#### 5.整合所有代码在一起

让我们称这个编译器为“sbn编译器”（SVG由数字编译器）。
我们使用词法分析器，解析器，变换器和生成器方法创建一个sbn对象。 然后添加一个“编译”方法来调用链中的所有4个方法。
我们现在可以将代码字符串传递给编译方法并获取SVG。

```js
var sbn = {}
sbn.VERSION = '0.0.1'
sbn.lexer = lexer
sbn.parser = parser
sbn.transformer = transformer
sbn.generator = generator

sbn.compile = function (code) {
  return this.generator(this.transformer(this.parser(this.lexer(code))))
}

// call sbn compiler
var code = 'Paper 0 Pen 100 Line 0 50 100 50'
var svg = sbn.compile(code)
document.body.innerHTML = svg
```

我做了一个<a href="https://kosamari.github.io/sbn/">交互式演示</a>，显示此编译器中的每个步骤的结果。 sbn编译器的代码发布在<a href="https://github.com/kosamari/sbn">github</a>上。 我现在在编译器中添加更多的功能。 如果你想看我们在这篇文章中做的基本编译器，请查看<a href="https://github.com/kosamari/sbn/tree/simple">简单的分支</a>。

<img src="/images/1-7ADpMcLo1VOnW4-fF2vjDg.png" />
https://kosamari.github.io/sbn/

### 编译器应不应该使用递归和遍历等？

是的，这些都是精彩的技术来构建一个编译器，但这并不意味着你必须优先采取这种方法。

我开始为DBN编程语言的一个小子集编译器，一个非常有限的小功能集。 从那时起，我扩展了范围，现在规划添加功能，如变量，代码块和循环到这个编译器。 在这一点上使用这些技术是一个好主意，但这不是开始的要求。

### 写编译器是很棒的一件事

你可以通过自己的编译器做什么？ 也许你可能想在西班牙语中制作新的类似JavaScript的语言...español脚本如何？

```js
// ES (español script)
función () {
  si (verdadero) {
    return «¡Hola!»
  }
}
```
任何事情都是有可能的，这里有人使用<a href="http://www.emojicode.org/">Emoji（表情符号代码）</a>和<a href="http://www.dangermouse.net/esoteric/piet.html">彩色图像（Piet编程语言）</a>开发变成语言。

## 学习编写编译器中学到了什么？

编译器很有趣，但最重要的是，它教会了我很多关于软件开发。 这里有几个我在学习编译器时学到的东西。
<img src="/images/1-AREFc7UVIAu_YIgk46EwaA.png" />

我怎么想象自己做一个编译器

#### 1.不熟悉的东西没关系

就像我们的词法分析程序,你不需要一开始就知道它。如果你不真正理解一段代码或技术,可以直接说“这我知道”，并且继续下一个步骤。不要有压力,最终你会知道。

#### 2.尽量友好提示错误信息

解析器的作用是遵循规则并检查是否按照这些规则输入内容。 所以很多时候错误发生时，尝试发送有用的消息和提示语。 很容易说“它不工作的方式”（像“ILLEGAL Token”或“undefined is not a function” 的错误JavaScript），但相反，尽可能地告诉用户发生了什么。

这也适用于团队沟通。 当有人被困在一个问题，而不是说“是的，不工作”，也许你可以开始说“我会google关键字，如___和___。”或“我建议阅读这个页面上的文档” 需要为他们做工作，但你当然可以帮助他们通过提供一点点帮助更好更快地完成工作。

Elm是一个拥抱<a href="http://elm-lang.org/blog/compiler-errors-for-humans">这种方法</a>的编程语言。 他们在他们的错误信息中写了“也许你想试试这个？

#### 3.上下文是一切

最后，就像我们的变换器将一种类型的AST转换为另一种更适合的最终结果，一切都是上下文特定的。

没有一个完美的方式来做事情。 不要因为这个事情它是流行的或你以前做过才去做，先考虑上下文。 对一个用户工作的事情可能是另一个用户的灾难。

此外，欣赏这些转换器做的工作。 你可能知道你的团队中的转换器 - 一个真正善于弥合差距的人。 转换器的这些工作可能不直接创造代码，但它是生产优质产品的一个重要的工作。

希望你喜欢这篇文章，也希望我说服你开发一个编译器是一件很棒的事情！

这是我在哥伦比亚的JSConf哥伦比亚2016年在哥伦比亚麦德林举行的演讲的摘录。 如果你想了解该演讲，请在这里<a href="http://kosamari.com/presentation/jsconfcolombia-2016/#0">查看幻灯片</a>。



