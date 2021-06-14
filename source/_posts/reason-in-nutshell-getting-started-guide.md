---
title: Reason 入门手册
created: 2018/05/30
description: Reason 入门手册
author: lanqy
---
# Reason 入门手册

译自：https://zaiste.net/reason_in_nutshell_getting_started_guide/

![Reason](/images/zaiste-reason.png)

> 本教程旨在提供对 Reason 的全面但相对较短的介绍。

[Reason](https://reasonml.github.io/) 是一种构建在 [OCaml](https://ocaml.org/) 之上的编程语言。它提供了函数式和面向对象的功能，并且具有类型安全性并关注性能。它是在 Facebook 创建的。它的语法与 JavaScript 类似。目的是使 JavaScript 与 JavaScript 程序员的互操作更容易。Reason 可以访问 JavaScript 和 OCaml 生态系统。OCaml 创建于 1996 年，它是一种具有类型推导的函数式编程语言。

Reason 网站包含一个[在线游乐场](https://reasonml.github.io/en/try.html)。它允许使用该语言并直观的查看生成的 JavaScript。它也可以从 OCaml 转换为 Reason。

## 为什么

- 在 JavaScript 类型注释中，linting 或统一格式是作为外部依赖项提供的，例如 Flow，TypeScript，ESLint 或 Prettier。Reason 提供了这些功能的开箱即用。这使得开发过程更加简化和方便。
- Reason 提供对 [ReasonReact](https://reasonml.github.io/reason-react/) 的 React 支持。它还支持 JSX 语法（ React 中使用的类似 HTML 的语法）。开箱即用
- Reason 还有生成本机二进制文件的能力。生成的代码是高性能的。没有虚拟机开销。它提供了一个便于部署过程的二进制文件。

## 它是如何工作的

Reason 被编译为 OCaml 的抽象语法树。这使 Reason 成为一个转译器。 OCaml 不能直接在浏览器中运行。AST 可以转换为各种目标。可以使用 BuckleScript 将 AST 编译为 JavaScript。它还提供了 OCaml 和 JavaScript 生态系统之间的互操作。

BuckleScript 速度极快，可生成可读的 JavaScript。它还提供了外部函数接口（FFI）以允许与 JavaScript 现有库的互操作性。检查[BuckleScript 基准](https://github.com/neonsquare/bucklescript-benchmark)。 BuckleScript 由 Messanger 团队在 Facebook 上使用，在 WebAssembly spec 解释器上由 Google 使用。在这里检查 Bucklescript 演示。 BuckleScript 由 [Hongbo Zhang](https://twitter.com/bobzhang1988/) 创建。

## 你好 Reason

我们将使用 BuckleScript 生成一个 Reason 项目。该工具提供即时可用的项目模板，称为 `themes`。

我们先从全局安装 `bs-platform` 开始：

```ocaml
npm install -g bs-platform
```

我们现在可以使用 `bs-platform` 提供的 `bsb` 二进制文件生成项目脚手架。我们将使用 `basic-reason` 模板从最基本的 Reason 项目结构开始。

```ocaml
bsb -init reason-1 -theme basic-reason
```

```ocaml
Making directory reason-1
Symlink bs-platform in /Users/zaiste/code/reason-1
```

以下是通过 BuckleScript 从 `basic-reason` 模板生成的 Reason 目录结构：

```ocaml
.
├── README.md
├── bsconfig.json
├── lib
├── node_modules
├── package.json
└── src
    └── Demo.re
```

`bsconfig.json` 包含 Reason 项目的 BuckleScript 配置。它允许指定要通过源编译的文件，通过 `bs-dependencies` 的 BuckleScript 依赖关系，编译器的附加标志等等。

下一步是构建项目。这将采取 Reason 代码并通过 BuckleScript 传递它以生成 JavaScript。默认情况下，编译器将以 Node.js 为目标。

```ocaml
npm run build
```

```ocaml
(* 输出 *)
> reason-1@0.1.0 build /Users/zaiste/code/reason-1
> bsb -make-world

ninja: Entering directory `lib/bs'
[3/3] Building src/Demo.mlast.d
[1/1] Building src/Demo-MyFirstReasonml.cmj
```

最后，我们可以使用 `node` 来运行 由 BuckleScript 生成的文件。

```ocaml
node src/Demo.bs.js

(* 将输出 Hello, BuckleScript and Reason! *)
```

## 语法 101

在本节中，我将详细介绍我发现的特殊的，新的或不同的语法元素。

### 模块

Reason 文件是模块。在 JavaScript 或类似的编程语言中没有 `require` 或 `import` 语句。模块定义必须以模块名称作为前缀以在外部工作。该功能来自 OCaml 。因此，您可以自由移动文件系统中的模块文件，而无需修改代码。

### 函数

函数使用 `let` 和 `=>` 来定义。

```ocaml
let greet = name =>
    Js.log("Hello, " ++ name ++ "!");

greet("Zaiste");
```

`++` 运算符用于连接字符串。

函数的输入参数可以被标记。这使得函数调用更加明确：传入的值不再需要遵循函数定义中的参数顺序。用 `~` 作为参数名称的前缀使其标记。

```ocaml
let greet = (~name, ~location) =>
    Js.log("Hello, " ++ name ++ "! You're in " ++ location);

greet(~location="Vienna", ~name="Zaiste");
```

### 数据结构

#### 变体

变体是一个数据结构，它保存来自一组固定值的值。这也被称为标记或不相交联合或代数数据类型。变体中的每个案例都必须大写。可选，它可以接收参数。

```ocaml
type animal = 
    | Dog
    | Cat
    | Bird;
```

#### 记录

这是一个记录

```ocaml
let p = {
    name: "Zaiste",
    age: 13
}
```

记录需要明确的类型定义。

```ocaml
type person = {
    name: string,
    age: int
}
```

在模块的作用域中，类型将被继承：p 绑定将被识别为 `person` 类型。在模块之外，您可以通过在文件名前添加前缀来引用该类型。

```ocaml
let p: Person.person = {
    name: "Sean",
    age: 12
}
```

有一个约定为每个类型创建一个模块并将类型命名为 `t` 以避免重复，即用 `Person.t` 代替 `Person.person`。

### 异步编程和 Promise

通过 BuckleScript 提供的内置 Promise 支持，作为 JS.Promise 模块提供。以下是使用 [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) 进行 API 调用的示例：

```ocaml
Js.Promise.(
    Fetch.fetch(endpoint)
    |> then_(Fetch.Response.json)
    |> then_(json => doSomethingOnReponse(json) |> resolve)
)
```

您需要使用 `then_` ，因为 `then` 是 OCaml 中的保留字。

### 模式匹配

模式匹配是基于提供值的形状的调度机制。在 Reason 中，模式匹配是通过 `switch` 语句实现的。它可以与变体类型或解构机制一起使用。

```ocaml
switch pet {
    | Dog => "woof"
    | Cat => "meow"
    | Bird => "chirp"
};
```

我们可以使用模式匹配进行列表解构：

```ocaml
let numbers = ["1", "2", "3", "4"];
switch numbers {
    | [] => "Empty"
    | [n1] => "Only one number: " ++ n1
    | [n1, n2] => "Only two numbers"
    | [n1, _, n3, ...rest] => "At least three numbers"
}
```

或者，我们可以将其用于记录解构

```ocaml
let project = {
    name: "Huncwot",
    size: 101101,
    forks: 42,
    deps: [{name: "axios"}, {name: "sqlite3"}]
};

switch project {
    | {name: "Huncwot", deps} => "Matching by `name`"
    | {location, years: [{name: "axios"}, ...rest]} => "Matching by one of `deps`"
    | project => "Any other situation"
}
```

### 可选值

option() 是 Reason 中描述 “ `nullable` (可空)” 值的内置变量：

```ocaml
type option('a) = None | Some('a);
```

### 不同

- `unit` 意味着“无”
- `unit => unit` 是一个函数的签名，它不接受任何输入参数并且不返回任何值;主要用于回调函数

## Reason 中的 React

### 你好 ReasonReact

[ReasonReact](https://reasonml.github.io/reason-react/) 是一个 Reason 中用于创建 React 应用程序的内置功能。

我们使用 BuckleScript 及其 `React` 模板创建一个 ReasonReact 项目。

```ocaml
bsb -init reasonreact-1 -theme react
```

Reason 团队推荐 ReasonReact 项目脚手架使用此方法。也可以使用带有 [reason-scripts](https://github.com/reasonml-community/reason-scripts) 模板的 yarn 来获得更完整的起点。

ReasonReact 提供了两种类型的组件：`statelessComponent` 和 `reducerComponent`。与 `stateless` (无状态)组件相反，`reducer` (减速器)组件是有状态的，提供类似 Redux 的 `reducer`。

```ocaml
let s = ReasonReact.string;

let component = ReasonReact.statelessComponent("App");

let make = (~message, _children) => {
    ...component,
    render: _self => 
        <h1 className="header">(s(message))</h1>
}
```

如前所述 `~` 指定一个带标号的参数来自由排序函数的输入参数。绑定名称中的 `_` 告诉编译器该函数的主体中未使用该参数。扩展运算符（`...`）与 `component` (组件)一起意味着我们扩展了现有的组件。在这个例子中，我们也重写（覆盖）了 `render` 函数。

Reason 中的 JSX 比 React 更严格：我们需要使用 `ReasonReact.string()` 显式包装字符串。

### 构建非平凡的 ReasonReact 应用程序

让我们构建一个超越显示预定义数据的 ReasonReact 应用程序。我们将为趋势库创建一个 GitHub 查看器。目的是展示如何与外部 API 集成，如何管理状态以及如何使用 React 的生命周期方法的方法。

为了这个例子的目的，我们将使用 [reason-scripts](https://github.com/reasonml-community/reason-scripts) 来引导我们的 Reason 项目。

```ocaml
yarn create react-app reasonreact-github --scripts-version reason-scripts
```

安装依赖

```ocaml
cd reasonreact-github
yarn
```

从以下开始：

```ocaml
yarn start
```

***存储库***是这个应用程序的中心概念。我们首先定义一个描述该实体的类型。我们将把它放在一个名为 Repo 的单独模块中。

```ocaml
type t = {
    name: string,
    size: int,
    forks: int
};
```

从现在开始，我们可以从应用程序中的任何 Reason 文件引用此类型的 `Repo.t`，而不需要 requiring（引入）它。

### 管理状态

我们已经看到了一个无状态的组件。现在让我们创建一个具有状态的组件。在我们的上下文中，我们将使用 `RepoList` 组件管理从 GitHub 的 API 中获取的趋势库列表。

首先定义由 `RepoList` 组件管理的状态的类型。

```ocaml
type state = {
    repos: list(Repo.t)
}
```

但是，有一个问题。最初，在从 GitHub API 获取趋势库列表之前，`repos` 是未定义的。Reason 类型系统不允许我们有 `undefined ` (未定义) 的值。我们可以用一个空列表来模拟初始状态，但这不是最优的。空列表还可能意味着我们对提取趋势库的查询没有返回任何结果。

让我们使用 Reason 的可选值来处理这种情况。

```ocaml
type state = {
    repos: option(list(Repo.t))
}
```

下一步是定义该组件的可能操作。在ReasonReact中， `actions` (操作)表示为变体。现在我们只会有一个名为 `ReposFetched` 的 `action` (操作)。

```ocaml
type action = 
    | ReposFetched(list(Repo.t));
```

为了在 ReasonReact 中创建一个有状态的组件，我们需要使用 `reducerComponent()` 函数。

```ocaml
let component = ReasonReact.reducerComponent("App");
```

这样的组件允许定义描述状态如何响应于 `actions` (动作)而被转换的 `reducer` (减速器)。Reducer将当前状态作为输入采取`actions` (动作)并将新状态作为输出返回。`reducer` (减速器)必须是纯粹的函数。

```ocaml
reducer: (action, _prevState) => {
    switch action {
        | ReposFetched(repos) =>
            ReasonReact.Update({repos: Some(repos)})
    }
};
```

基于我们在 `reducer()` 方法中收到的参数，我们是模式匹配 `action` (操作)。模式匹配必须是详尽的。所有变体值必须匹配。`reducer` 定义放置在组件的 `main` 函数中。

为了完成组件的定义，我们来定义它的初始状态：

```ocaml
initialState: () => {
    repos: Some([
        {
            name: "Huncwot",
            size: 101101,
            forks: 42
        }
    ])
};
```

### 与 API 集成

我们将使用 [bs-fetch](https://github.com/reasonml-community/bs-fetch) 从外部 API 获取数据。它是一个 BuckleScript 库，充当 Fetch API 之上的一个薄层。一旦数据被提取，我们将使用 `bs-json` 来提取我们感兴趣的字段。

开始安装 `bs-fetch` 和 `bs-json`：

```ocaml
npm i bs-fetch @glennsl/bs-json
```

将它们添加到 `bsconfig.json` 中的 `bs-dependencies`：

```ocaml
{
    "bs-dependencies": [
        ...,
        "bs-fetch",
        "@glennsl/bs-json"
    ]s
}
```

我们将 Repo 类型定义为一组三个字段：`name`，`size` 和 `forks`。一旦从 GitHub API 获取有效载荷，我们就解析它以提取这三个字段。

```ocaml
let parse = json =>
    Json.Decode.{
        name: json |> field("name", string),
        size: json |> field("size", int),
        forks: json |> field("forks", int)
    }
```

`field` 是 `Json.Decode` 的一个方法。`Json.Decode.{...}`（注意这个点号）打开 `Json.Decode` 模块。它的属性现在可以在这些大括号内使用，而不需要使用 `Json.Decode` 作为前缀。

由于 GitHub 在 `items` 下返回 `repos` ，我们定义另一个函数来获取该列表。

```ocaml
let extract = (fields, json) =>
    Json.Decode.(
        json |> at(field, list(parse))
    );
```

最后，我们可以发出请求并通过解析函数传递返回的数据：

```ocaml
let list = () => 
    Js.Promise.(
        Fetch.fetch(endpoint)
        |> then_(Fetch.Response.json)
        |> then_(text => extract(["items"], text) |> resolve)
    );
```

### React 生命周期方法

让我们使用 `didMount` 生命周期方法触发从 GitHub API 获取存储库。

```ocaml
didMount: self => {
    let handle = repos => self.send(ReposFetched(repos));
    Repo.list()
    |> Js.Promise.then_(repos => {
        handle(repos);
        Js.Promise.resolve();
    });
    |> ignore;
};
```

`handle` 是一个将 `ReposFetched` 操作分派给 `reducer` 的方法。一旦承诺解决，操作将把获取的存储库传送到 `reducer` (减速器)。这将更新我们的状态。

### 渲染

由于我们区分了非初始化状态和存储空列表，因此处理初始***加载进度***消息很简单。

```ocaml
render: self => {
    <div>
    (
        switch self.state.repos {
            | None => s("Loading repositories...")
            | Some([]) => s("Empty list")
            | Some(repos) =>
            <ul>
            (
                repos
                |> List.map((repo: Repo.t) => <li> (s(repo.name)) </li>)
                |> Array.of_list
                |> ReasonReact.array
            )
            </ul>
        }
    )
    </div>
};
```

### 错误处理

无

### CSS 中的类型

使用 [bs-css](https://github.com/SentiaAnalytics/bs-css) 的 CSS 类型。

通过 `yarn` 安装

```ocaml
yarn add bs-css
```

并将它添加到 `bsconfig.json` 中的 `bs-dependencies`：

```ocaml
"bs-dependencies": [
  ...,
  "bs-css"
]
```

组件：

```ocaml
let style = 
    Css.(
        {
            "header": style([backgroundColor(rgba(111, 37, 35, 1.0)), display(Flex)]),
            "title": style([color(white), fontSize(px(28)), fontWeight(Bold)]),
        }
    );

let make = _children => {
    ...component,
    render: _self =>
    <header className=style##header>
        <h1 className=style##title> (s("This is title")) </h1>
    </header>
};
```

## 词汇

- `rtop` 是 Reason 的交互式命令行。
- Merlin 是 OCaml 和 Reason 的自动完成服务文件。
- [@bs...] FFI 的 Bucklescript 注释

## 其他资源

- [Awesome ReasonML](https://github.com/vramana/awesome-reasonml)
- [Real World OCaml](https://realworldocaml.org/)
- http://2ality.com/archive.html?tag=reasonml
- https://jamesfriend.com.au/a-first-reason-react-app-for-js-developers
- https://github.com/reasonml-community/reason-scripts
- https://dev.to/jlewin_/reasonml-getting-started-53gi
- https://medium.com/@ryyppy/a-quick-look-on-my-reasonml-workflow-with-vscode-637685f9417a
- https://redex.github.io/
- https://github.com/arecvlohe/reasonml-cheat-sheet
- https://news.ycombinator.com/item?id=16500481
- [ReasonTools browser extension](https://github.com/reasonml/reason-tools)

## 待定

```ocaml
module History = {
    type h;
    [@bs.send] external goBack : h => unit = "";
    [@bs.send] external goForward: h => unit = "";
    [@bs.send] external go : (h, ~jumps: int) => unit = "";
    [@bs.get] external length : h => int = "";
};
```

BuckleScript 允许我们将原始 JavaScript 与 Reason 代码混合使用。

```ocaml
[%bs.raw {|require('./app.css')|}];
```