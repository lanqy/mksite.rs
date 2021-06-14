---
title: 开始使用 ReasonML 和 React Native
description: 开始使用 ReasonML 和 React Native
created: 2018/05/30
author: lanqy
---

# 开始使用 ReasonML 和 React Native

译自：https://blog.callstack.io/getting-started-with-reasonml-and-react-native-299476389c3e

![1_s4A8_PwjA1Ichs6VnVq3Dg.jpeg](/images/1_s4A8_PwjA1Ichs6VnVq3Dg.jpeg)
Photo by Will O (https://unsplash.com/photos/St4qInZrYC4)

> 以下是如何开始使用 React Native和 ReasonML 的指南。为了本博文的目的，我假设您已经熟悉 React Native 并部分使用 ReasonML 。如果您尚未接触过 ReasonML，请查看[文档](https://reasonml.github.io/docs/en/global-installation.html)。

首先安装 [React Native CLI](https://facebook.github.io/react-native/docs/getting-started.html)：

```ocaml
npm i -g react-native-cli
```

现在我们可以初始化一个新的 React Native 项目，就像我们对每个 React Native 应用程序所做的一样：

```ocaml
react-native init MyReasonApp
```

## 添加 “ Reason 部分”

我们将需要 3 个包：

- [`bs-platform`](https://bucklescript.github.io/docs/en/what-why.html) - 将 ReasonML / OCaml 编译为干净，可读和高性能的 JavaScript 代码
- [`reason-react`](https://reasonml.github.io/reason-react) - ReactJS 的 Reason 绑定
- [`bs-react-native`](https://github.com/reasonml-community/bs-react-native) - React Native 的 BuckleScript 绑定

让我们将它们添加到我们的项目中：

```ocaml
npm i -P bs-platform reason-react bs-react-native
```

现在我们需要创建一个 `bsconfig.json` ，它是一个 [BuckleScript 的配置文件](https://bucklescript.github.io/docs/en/build-configuration.html)：

```ocaml
{
    "name": "my-reason-app",
    "bsc-flags": ["-bs-no-version-header", "-bs-super-errors"],
    "refmt": 3,
    "bs-dependencies": ["reason-react", "bs-react-native"],
    "reason": {
        "react-jsx": 2
    },
    "package-specs": {
        "module": "commonjs",
        "in-source": true
    },
    "sources": [
        {
            "dir": "src",
            "subdirs": true
        }
    ]
}
```

让我们在这里停一分钟。有几个不同于通常的设置。

首先是 [`"subdirs": true`](https://bucklescript.github.io/docs/en/build-configuration.html#sources)，使得 BuckleScript 知道它应该检查应该编译的代码的子目录。

另一个是 [`"in-source": true`](https://bucklescript.github.io/docs/en/build-configuration.html#package-specs) ，这个非常方便，与源文件一起生成输出（默认情况下，它们输出到 `lib/js` 目录下）。当我们引用 `.js` 文件或资源文件时，这非常有用。

没有它，要导入一个图像，你会参考它：

```ocaml
<Image
  style=Styles.icon
  source=(
    Required(Packager.require("../../../assets/right.png"))
  )
/>
```

使用 `"in-source": true` ，它会看起来像：

```ocaml
<Image
  style=Styles.icon
  source=(Required(Packager.require("./assets/right.png")))
/>
```

我更喜欢后者，因此我启用了该选项。

## React Native 中的 ReasonML

我们完成了配置，回顾一下，我们添加了三个软件包：`bs-platform`，`reason-react` 和 `bs-react-native`。然后我们添加了 `BuckleScript` 的配置文件 `bsconfig.json`，就是这样！ 🎉


现在我们来写一些 Reason 吧！

正如我们之前在 `bsconfig.json` 中定义的，我们将把所有的 ReasonML 代码保存在 `src` 目录中。在新创建的 `src` 目录中（在我们项目的根目录中），让我们添加`App.re` ，它可能看起来像这样：


```ocaml
open BsReactNative;

/* 这里我们定义一些样式 */
module Styles = {
    open Style;

    let container = 
        style([
            flex(1.),
            justifyContent(Center),
            alignItems(Center),
            backgroundColor(String("tomato")),
        ]);
    let text = style([color(String("#fff")), fontSize(Float(24.))]);
};

module Main = {
    let component = ReasonReact.statelessComponent("Main");

    let make = _children => {
        ...component,
        render: _self =>
            <View style=Styles.container>
                <Text style=Styles.text>
                    (ReasonReact.string("Let's play with ReasonML!"))
                </Text>
            </View>,
    };
};

let app = () => <Main />;
```

我还从项目的根目录中删除了 `App.js`（这是由 React Native CLI 生成的文件）。

我们需要做的最后一件事是将我们编译好的 `App` 导入到 `index.js` 中：

```ocaml
import { AppRegistry } from 'react-native';
import { app } from './src/App';

/*
 如果 "in-source" 选项为 false (默认为 false )，您将以这种方式导入app:
 import { app } from "./lib/js/src/App.js";
*/

AppRegistry.registerComponent('MyReasonApp', () => app);
```

最后，我们可以通过运行来编译代码:

```ocaml
npm run watch
```

这将监视您对 Reason 代码所做的任何更改并进行编译（如果没有错误的话）。

现在让我们开始运行 React Native 应用程序：

```ocaml
react-native run-ios
```

你应该看到：

![1_tsDhHE5u-a4v8Url4lXlUw.png](/images/1_tsDhHE5u-a4v8Url4lXlUw.png)

快乐 hacking ! 🎉

这里是与上述设置的 repo 链接：

https://github.com/knowbody/ReasonReactNativeApp

... 

[在 Twitter 上关注 Mateusz Zatorski](https://twitter.com/matzatorski)
