---
title: BuckleScript 绑定技巧
created: 2018/05/25
description: BuckleScript 绑定技巧
author: lanqy
---
# BuckleScript 绑定技巧

译自： https://jwheatley.co/bucklescript-binding-tips/

> 这是一组适用于当前或未来项目的简单绑定提示，减少了对试验 / 错误的需求，并减少了深度文档搜索解决与下面示例相关的问题。

## 保持 React.js 中的 js

```ocaml
module Container = {
  [@bs.module "./styled/Container"]
  external js : ReasonReact.reactClass = "default";
  let make = children =>
    ReasonReact.wrapJsForReason(
      ~reactClass=js,
      ~props=Js.Obj.empty(),
      children
      );
};
```

我将外部 js 命名为如果我需要将 React 组件的规范传递到 Reason 中使用的基于 JS 的库，我可以使用 `<ComponentName>.js` 来访问它。

例如：

```ocaml
open Bindings.ReactRouter;
let component = ReasonReact.statelessComponent("App");

let make = (_children) => {
  ...component,
  render: (_self) =>
    <Router>
      <div>
        <Header />
      </div>
      <Switch>
        <Route exact=true path="/" component=Home.js />
        <Route component=NoMatch.js />
      </Switch>
      <Footer />
      </div>
    </Router>
}
```

## 属性

这是 Khoa Nguyen 关于使用 BuckleScript 对象“特殊创建函数”绑定 ReasonReact 属性的“[正确方法](https://khoanguyen.me/writing-reason-react-bindings-the-right-way/)”。

## 使 ReasonReact 组件对 JS 更友好

```ocaml
let component = ReasonReact.statelessComponent("App");

let make = _children => {
  ...component,
  render: self => <div />
};

let default = ReasonReact.wrapReasonForJs(~component, _jsProps => make([||]));
```

使用为 JS 封装的组件创建“默认”变量，您可以通过它的 BuckleScript 输出文件名简单地导入它，而不用担心命名的导出。例如: 用 `import App from './App.bs'`; 代替 `import { AppWrapped } from './App.bs'`;

## 非代码导入

```ocaml
type assetT;
[@bs.module] external twitterSvg : assetT = "../assets/svg/twitter.svg";
```

我个人不会假设在 JS 端会输出什么类型（int，string 等）。我只是给它一个泛型类型，让 Webpack 或 Metro Bundler 处理它，但它通常会。

## 全局

```ocaml
[@bs.val] external setInterval : (unit => unit, int) => int = "setInterval";
[@bs.val] external clearInterval : int => unit = "clearInterval";
```

要与 BuckleScript 绑定的全局 JS 变量是 `bs.val`，而不是 `bs.module`。

## 函数重载

```ocaml
module Date = {
  type t;
  [@bs.new] external fromValue: float => t = "Date";
  [@bs.new] external fromString: string => t = "Date";
};

let date1 = Date.fromValue(107849354.);
let date2 = Date.fromString("1995-12-17T03:24:00");
```

如果你绑定的 JS 函数采用不同类型的参数，你可以使用不同类型的不同名称进行多次绑定，以表示你将传递给它的东西。

```ocaml
module Date = {
  type t;
  [@bs.new] external make : ([@bs.unwrap] [ | `Value(float) | `String(string)]) => t = "Date";
};

let date1 = Date.make(`Value(107849354.));
let date2 = Date.make(`String("1995-12-17T03:24:00"));
```

你也可以使用 `bs.unwrap` 来“重载”你的函数类型。
