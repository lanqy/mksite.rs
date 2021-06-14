---
title: 在 ReasonReact中解码 JSON
description: 在 ReasonReact中解码 JSON
created: 2018/05/28
author: lanqy
---
# 在 ReasonReact中解码 JSON

译自：https://medium.com/@idkjs/decoding-json-in-reasonreact-cff3a07c1200

![在ReasonReact中解码JSON](/images/1_OizIr4Tc2m6lCGdr9478ew.png)

Reason 是一种新的来自 Facebook 的静态类型函数编程语言，可以编译为 Javascript。ReasonReact 是 React 的一个包装器，可以很容易地在 Reason 中使用。

我们将构建一个小型单页网络应用程序，通过其步伐放置 Reason React。该应用将显示与 Reason 相关的最佳 Github repos 列表。这是一个足够小的任务，我们可以在几个小时内完成它，但是它也有足够的复杂性使我们可以开始厌倦这种新语言。本教程预期不存在 Reason 的现有知识，但对静态类型的基本熟悉会有所帮助。

## 一个新项目

我们将使用使用 `create-react-app` 的 `reason-scripts`，这将为我们的应用程序创建一个起点，该应用程序将被称为 `decode-json`：

```ocaml
yarn create react-app decoding-reason --script-version reason-scripts
```
通过运行 `yarn start` 来启动项目,在浏览器中打开 `https://localhost:3000`。

## 记录类型

我们将解码来自此Github API请求的公共数据的响应：https://api.github.com/search/repositories?q=topic%3Areasonml&type=Repositories 但首先可以使用假数据进行设置。

创建一个名为 `RepoData.re` 的新文件并将其添加到其中。

```ocaml
type repo = {
    full_name: string,
    stargazers_count: int,
    html_url: string
};
```

## 文件既是模块

我们已经在文件的顶层定义了我们的类型。

> 顶层基本上意味着视觉上，没有缩进（非常糟糕的解释方式，但你明白了）。`let foo = 1`不嵌套在其他任何内容中都是顶级的。`let foo = 1`在一个函数体内时，则不是顶层。[@chenglou](https://reasonml.chat/t/bucklescript-top-level-declarations/399/3?u=idkjs)

在 Reason 中，每个文件都是一个模块，并且使用关键字 `let`，`type` 和 `module` 在文件顶层定义的所有东西都暴露在其他文件（即其他模块）中使用。在这种情况下，其他模块可以将我们的 repo 类型引用为 RepoData.repo。与 Javascript 不同，不需要导入来引用其他模块的东西。

让我们在 `app.re` 中使用我们的类型。repos 页面只是 repos 列表，列表中的每个项目都包含 repos 名称（链接到 Github 上的 repos）以及 repos 的 stars 数量。我们将定义一些虚拟数据，并在 `App.re` 中绘制一个名为 RepoItem 的新组件来表示 repos 列表中的项目：

```ocaml
// App.re
let component = ReasonReact.statelessComponent("App");

let make = (_children) => {
    ...component,
    render:(_self) => {
        /* our dummy data */
        let dummyRepo: RepoData.repo = {
            stargazers_count: 27,
            full_name: "jsdf/reason-react-hacker-news",
            html_url: "https://github.com/jsdf/reason-react-hacker-news"
        };

        <div className="App">
            <h1>{ReasonReact.stringToElement("Reason Projects")}</h1>
            <RepoItem repo=dummyRepo />
        </div>
    }
};

```

在声明开始 `let dummyRepo: RepoData.repo = {...}`，`dummyRepo` 是我们正在定义的常量的名称， `RepoData.repo` 是我们注释它的类型，它来自我们在其中定义它的位置 `RepoData.re`。请记住，该模块在项目的任何地方都可用。Reason 可以推断出我们声明的大部分事物的类型，但是在这里包含注释是很有用的，这样类型分析者可以告诉我们我们的测试数据是否犯了错误。

## Reason 中的返回值

请注意，渲染函数的主体现在包含在 `{}` 括号中，因为它包含多个语句。在 Javascript 中，如果我们在 `=>` 箭头函数的主体周围使用大括号，我们需要添加一个返回语句来返回一个值。然而在 Reason 中，函数返回最后一条语句产生的值，这里：

```ocaml
···
<div className="App">
    <h1>{ReasonReact.stringToElement("Reason Projects")}</h1>
    <RepoItem repo=dummyRepo />
</div>
```

自动成为返回值。如果你不想从函数返回任何东西，你可以创建最后一个语句 ()（在 [Reason](https://reasonml.github.io/docs/en/function.html#optional-labeled-arguments) 中称为 “unit”）。

## 在 ReasonReact 中定义组件

你现在可能会看到一个错误提示，说 RepoItem 文件或模块没有找到。这是因为我们在App组件渲染函数中添加了 `<RepoItem repo=dummyRepo />`，但是我们还没有创建该模块。添加一个名为 `RepoItem.re` 的新文件，包含：

```ocaml
let component = ReasonReact.statelessComponent("RepoItem");

let make = (~repo: RepoData.repo, _children) => 
{
    ...component,
    render: (_self) =>
    <div>{ReasonReact.string(repo.full_name)}</div>
};
```
这里发生了什么？让我们来剖析这个文件中发生了什么。

每个 Reason React 组件都是一个 Reason 模块，它定义了一个名为 make 的函数，它定义了 props 和 children 参数。props 被指定为[标签参数](https://reasonml.github.io/docs/en/function.html#labeled-arguments)。

```ocaml
let compoent = ReasonReact.statelessComponent("SomeComponent");

let make = (~someProp, ~anotherProp, _children) => /* some stuff here */;
```

`make` 函数返回一条记录。此记录中的第一件事通常是 `...component`，其中 `component` 是`ReasonReact.reducerComponent` 或 `ReasonReact.statelessComponent` 的返回值（对于分别使用状态和不使用状态的组件）。如果这看起来有点奇怪，可以把它看作是从 React组件类继承的，就像做 Foo 类的扩展等同于在ReactJS中扩展 `class Foo extends React.Component {...` 一样。

```ocaml
// RepoItem.re
let component = ReasonReact.statelessComponent("RepoItem");

let make = (~someProp, ~anotherProp, _children) => 
    {
        ...component,
        /* render and lifecycle methods go here */
    };


```

记录的其余部分是您可以从 React 中添加渲染函数和您习惯使用的生命周期方法的位置。

因此，回到 `RepoItem.re`：

```ocaml
let compoent = ReasonReact.statelessComponent("RepoItem");

let make = (~repo: RepoData.repo, _children) =>
    {
        ...component,
        render: (_self) =>
        <div>{ReasonReact.stringToElement(repo.full_name)}</div>
    }

```

我们这里有一个 `stateless component` (无状态的组件)，它接受一个名为repo的属性（注解 RepoData 模块中的类型回购），并渲染一个 div。

在ReactJS中，你可以使用 this.props 来访问render方法中的组件的属性。在 ReasonReact 中，我们接收属性作为 make 函数的标签参数，我们可以直接在渲染函数中使用它们（就像我们正在访问上面的 repos 属性的 full_name 字段一样）。

make 函数也会传递一个 children 参数，但我们并没有在这个组件中使用子元素，所以我们在 _children 参数名称的开始处放置了一个 _ 下划线。这只是让 Reason 知道我们并没有真正使用这个参数。尽管我们没有使用这个参数，但它仍然需要包含在函数参数中，否则会出错。

接下来，我们将充实渲染函数以呈现 repos 记录的字段：

```ocaml
let component = ReasonReact.statelessComponent("RepoItem");

let make = (~repo: RepoData.repo, _children) =>
    {
        ...component,
        render: (_self) =>
            <div className="RepoItem">
                <a href=repo.html_url>
                    <h2>{ReasonReact.string(repo.full_name)}</h2>
                </a>
                {ReasonReact.string(string_of_int(repo.stargazers_count) ++ " stars")}
            </div>
    };
```

请注意，我们必须使用 string_of_int 函数将 repo.stargazers_count 的 int 值转换为一个字符串。然后我们使用++字符串连接运算符将它与字符串 “stars” 结合起来。

现在是保存并查看浏览器进度的好时机。

![1_rVw1dhPmkJ_cyT63uDbN0g](/images/1_rVw1dhPmkJ_cyT63uDbN0g.png)

## 有状态的 React 组件又名 reducerComponent 和 Variants

我们的应用程序将加载一些数据然后渲染它，这意味着我们需要一个地方来加载数据。 React 状态组件似乎是一个明显的选择。所以我们会让我们的应用组件成为有状态的。我们通过将 `ReasonReact.statelessComponent` 更改为 `ReasonReact.reducerComponent` 来完成此操作。

在 `App.re`：

```ocaml
type state = {
    repoData: option(RepoData.repo)
};

/* 我们的虚拟数据 */

let dummyRepo: RepoData.repo = {
    stargazers_count: 27,
    full_name: "jsdf/reason-react-hacker-news",
    html_url: "https://github.com/jsdf/reason-react-hacker-news",
};

let repoItem = (repoData: option(RepoData.repo)) =>
    switch (repoData) {
        | Some(repo) => <RepoItem repo />
        | None => ReasonReact.string("Loading")
    };

let make = _children => {
    ...component,
    initialState: () => {
        repoData: Some(dummyRepo)
    },

    reducer: ((), _) => ReasonReact.NoUpdate,

    render: ({state:{repoData}}) =>
        <div className="App">
            <h1> (ReasonReact.string("Decoding JSON in ReasonReact")) </h1>
            (repoItem(repoData))
        </div>,
};


```

我们改变了一些关键的东西：我们已经为使用 Reason 的内置 option Variant 类型的组件状态定义了一个类型。这里简单地称为 `state`，`ReasonReact.statelessComponent` 已经成为 `ReasonReact.reducerComponent`，我们已经为组件添加了一个 `initialState` 函数，并且我们已经改变了渲染以将自身作为参数（删除 _ 下划线以便自 self 不再被忽略 ），现在正在使用 `{state：{repoData}` 作为 RepoItem 的属性。什么？！！以下语法称为解构。我们现在正在访问的 `self` 方法有一个 `state` 属性，并且使用 `{state:...}`我们说从上面的第22行开始使用它的状态。

## 变体！选项

我们将我们的 `state` 类型定义为：

```ocaml
type state = {
    repoData: option(RepoData.repo)
};
```

Reason 中 `option` 是由 “变体” 组成的类型。这基本上意味着这种类型的值可以是已经明确定义的几种可能变化中的一种。在 `option` 的情况下，变体是 `Some` 和 `None` 。`Some` 用于存在值（并且包含值本身），而 `None` 表示没有值（如 Javascript 中的 `null` ）。在这里，我们'包裹' dummyRepo  在 `Some` 变体中，因为我们有一个值。

该 `option` 告诉我们（和编译器），该 `state` 可以是 `Some` 或 `None` 的任何值。所以当我们使用这种类型时会有 `Some(RepoData.repo)` 或 `None` 。

那么为什么使用这个包装器，而不是让我们的 `repoData` 字段包含一个值或 null ？Reason 迫使我们在实际使用价值时处理两种可能的情况。这很好，因为这意味着我们不会意外忘记处理'null（空）'情况。请注意，在调用 `ReasonReact.reducerComponent` 之前，必须定义 `state` 类型，否则会出现类似“类型构造函数状态将脱离其作用域”的错误。我们将通过创建一个名为 `repoItem` 的变量来告诉组件在每种情况下应该做什么，并定义在类型状态下定义的变例中我们想要发生的事情。

## 选项和模式匹配

当我们定义组件的初始状态时，目前我们已经有了 `repoData` 伪数据，但是一旦我们从服务器加载它，它的初始值将为空。但是，在 Reason 中，您不能像记录 Javascript 那样只是将记录字段的值设为 `null`。相反，可能不存在的事物需要用称为 `option` 的另一种类型“包装”。我们可以改变我们的状态类型来表示如下：

```ocaml
type state = {repoData: option(RepoData.repo)};
```

并且在我们的 `initialState` 函数中，我们将我们的 `repo` 记录包装在 `Some()` 中：

```ocaml
initialState: () => {
    repoData: Some(dummyRepo),
}
```

在上面的代码中，我们使用 `Some` 和 `None` 变体来定义一个 `repoItem`，如果有一些数据，我们将这些数据传递给我们的 `<RepoItem />` 模块，并将它返回给我们组件中的 UI。如果没有数据，我们告诉该函数使用None选项，返回一个 `div `来呈现 “Loading” 到 UI。

然后在渲染 `div` 中，我们传递当前的 `repoData` ，然后传递给 `renderItem` 函数来处理在每种情况下要做的操作 `Some` 或 `None` 。我们无法直接将 `state.repoData` 作为 `RepoItem` 的 `repo` 的属性，因为它被包装在一个 `option()` 中，但是 `RepoItem` 期待它没有选项包装。那么我们如何解开它呢？我们使用模式匹配。这是 `Reason` 用来涵盖所有可能的情况（或者至少明确地抛出错误）的地方。模式匹配使用 `switch` 语句。然而，与 Javascript 中的 `switch` 语句不同，reason 中的 `switch` 语句可以匹配值的类型（例如 `Some` 和 `None` ），而不仅仅是值本身。我们将改变我们的渲染方法，使用 `switch` 在每种可能的情况下提供逻辑来呈现我们的 `repo` 项目。我们可以通过创建一个函数 `renderItem` 来处理每个 `case` 并根据结果进行渲染。

```ocaml
repoItem = (repoData: option(RepoData.repo)) => 
    switch (repoData) {
        | Some(repo) => <RepoItem repo />
        | None => ReasonReact.string("Loading")
    };
```

在这里你可以看到 `switch` 语句有一个 `case` 与 `state.repoData` 类型的 `Some` 类型匹配，并且将实际的 `repo` 记录提取到一个名为 `repo` 的变量中，然后它将它用在 => 右边的表达式中， 返回一个 <RepoItem> 元素。这个表达式只会在 `state.repoData` 是 `Some` 的情况下使用。或者，如果 `state.repoData` 为 `None`，则将显示文本 “Loading”。

我们将在我们的 div 中调用 repoItem 并将它作为 repoData 解构的 state.repoData。

```ocaml
render: ({state: {repoData}}) => 
    <div className="App">
        <h1> (ReasonReact.string("Decoding JSON in ReasonReact")) </h1>
        (repoItem(repoData))
    </div>,
```

如果您运行 `yarn start` 开始，您应该在浏览器中看到与以前相同的输出：

![1_rVw1dhPmkJ_cyT63uDbN0g](/images/1_rVw1dhPmkJ_cyT63uDbN0g.png)

## Reducer Components

那么，为什么 Reason React 中的有状态组件类型称为 reducerComponent ？与 ReactJS 相比，ReasonReact 处理组件状态更改的方式略有不同。如果你已经使用 [Redux](https://redux.js.org/) ，它会看起来很熟悉。如果你还没有，不要担心，这里不需要背景知识。

基本上，不是像 onClick 那样在事件处理程序中做一堆事情然后调用 `this.setState` ，我们只需要知道我们想要对组件状态做出什么样的改变，然后调用 `self.send` 一个“ `action` ”，它只是一个表示应该发生的状态更改的值，以及我们需要更改的任何信息。这意味着大部分状态变化代码都可以用纯函数隔离，这使得它更容易跟踪，并且更容易编写测试。

我们可以尝试通过这种方式进行状态更改，方法是首先将 `state` 设置为 `None` ，然后在用户单击按钮后更改状态。这是一个人为的例子，但它对说明状态变化很有用。点击此按钮后，假设我们正在从API加载数据:)。

首先，我们需要添加一个名为 `action` 的类型，它列举了可能发生在我们组件中的各种可能的状态变化。现在只有一个：`Loaded`，用于加载 repo 数据时：

```ocaml
type action = 
    | Loaded(RepoData.repo);
```

之后，我们添加一个 `reducer` 方法，它接受一个这样的动作和当前状态，然后计算并返回更新的状态：

```ocaml
reducer = (action, _state) => 
    switch (action) {
        | Loaded(loadedRepo) =>
            ReasonReact.Update({
                repoData: Some(loadedRepo)
            })
    };
```

您可以看到，我们的实现是对动作类型进行模式匹配并返回包含新状态的 `ReasonReact.Update`。现在我们只是为 `Loaded` 行动提供一个案例，但在未来，我们可以想象在这里实施其他类型的状态更改，以响应不同的 `action` 变体。

接下来我们更改 `initialState` ，以无 repo 数据开始：

```ocaml
initialState: () => {
    repoData: None
},
```

最后，我们在渲染函数中添加一个按钮元素。我们使用 `self.send` 方法添加到我们的解构对象中，为按钮的 `onClick` `prop` 创建一个处理函数。

`send` 采取点击事件调用我们想要使用的动作和它期望的任何值。在这里，`send(Loaded(dummyRepo))`，将点击转换为我们 `reducer` 的动作。像这样的处理程序也可以使用来自 `click` 事件对象的信息，但在这种情况下我们不需要它，所以我们把下划线 `_` 放在它之前忽略它。我们可以创建这样一个按钮：

```ocaml
<button onClick=(_event => send(Loaded(dummyRepo)))>
    (ReasonReact.string("Load Repos"))
</button>
```

我们可以显示一条消息，在初始空白状态下（当 `state.repoData` 为 `None` 时）点击按钮来代替呈现的 `RepoItem` ：

```ocaml
repoItem = (repoData: option(RepoData.repo)) => 
    switch (repoData) {
        | Some(repo) => <RepoItem repo />
        | None => ReasonReact.string("Click Button To Load")
    };
```

与在 JS React 中调用 `setState` 相比，使用 `action` 和 `reducer` 的额外步骤似乎过于复杂，但随着有状态组件的增长并具有更多可能的状态（它们之间可能存在越来越多的转换），组件很容易成为难以跟踪和无法测试的纠结。 这是 `action-reducer` 模型真正闪耀的地方。

您可以在 [render-button-detour](https://github.com/idkjs/decoding-json-in-reason-react/tree/render-button-detour) 附带的 repo 中看到此版本的代码。

好，现在我们知道如何进行状态更改，让我们把它变成一个更现实的应用程序。

## 使用数组与单个 Repo

在我们从 JSON 加载数据之前，还需要对组件进行一次更改。我们实际上想要显示 repo 列表，而不仅仅是一个清单，所以我们需要改变我们的状态类型：

```ocaml
type state = {
    repoData: option(array(RepoData.repo))
};
```
并对我们的虚拟数据进行相应的更改：

```ocaml
dummyRepos: array(RepoData.repo) = [|
  {
    stargazers_count: 27,
    full_name: "jsdf/reason-react-hacker-news",
    html_url: "https://github.com/jsdf/reason-react-hacker-news"
  },
  {
    stargazers_count: 93,
    full_name: "reasonml/reason-tools",
    html_url: "https://github.com/reasonml/reason-tools"
  }
|];
```

呃，`[| ... |]` 语法？这是 Reason 的数组字面量语法。如果你没有 `|` 管道字符（所以它看起来像正常的 JS 数组语法），那么你会定义一个 `List` 而不是一个数组。在 Reason 中列表是不可变的，而数组是可变的（如 Javascript 数组），但是如果处理可变数量的元素，则列表更容易处理。无论如何，我们正在使用一个数组。

我们需要查看代码并将所有引用 `repoData`  的地方都改为 `RepoData.repo`，而不是指定 `array(RepoData.repo)`。

最后，通过映射 `repos` 数组并为每个 `RepoItem` 创建一个 `<RepoItem />`，我们将改变渲染方法来渲染一个 `RepoItem` 数组而不是一个。 我们必须使用 `ReasonReact.array` 将元素数组转换为元素本身，以便它可以在下面的 JSX 中使用。


```ocaml
repoItems = (repoData: option(array(RepoData.repo))) =>
    switch (repoData) {
        | Some(repo) =>
            ReasonReact.array(
                Array.map(
                    (repo: RepoData.repo) => <RepoItem key=repo.full_name repo />,
                    repos,
                ),
            )
        | None => ReasonReact.string("Loading")
    };

let make = _children => {
    ...component,

    initialState: () => {
        repoData: Some(dummyRepos)
    },

    reducer: ((), _) => ReasonReact.NoUpdate,

    render: ({state: {repoData}}) =>
        <div className="App">
            <h1> (ReasonReact.string("Decoding JSON in ReasonReact")) </h1>
            (repoItems(repoData))
        </div>,
};
```

我在 `repo` 中标记了代码的各个部分。所以在这一点上，你可以检查 [`setting-up-arrays tag`](https://github.com/idkjs/decoding-json-in-reason-react/tree/setting-up-arrays)。

现在，加载一些真实的数据。

## BuckleScript

在获取我们的 JSON 并将其转换为记录之前，首先我们需要安装一些额外的依赖关系。运行：

```ocaml
npm install --save bs-fetch @glennsl/bs-json
```
或者

```ocaml
yarn add bs-fetch @glennsl/bs-json
```

以下是这些包的作用：[`bs-fetch`](https://github.com/reasonml-community/bs-fetch)：包装浏览器获取 API，以便我们可以从 Reason [`@glennsl/bs-json`](https://github.com/glennsl/bs-json) 中使用它：允许使用将从服务器获取的 JSON 转换为 Reason 记录。

这些包与我们一直使用的 Reason-to-JS 编译器一起工作，这就是所谓的 [BuckleScript](https://bucklescript.github.io/) 。

在我们可以使用这些新安装的 BuckleScript 包之前，我们需要让 BuckleScript 知道它们。为此，我们需要对项目根目录下的 `.bsconfig` 文件进行一些更改。在 `bs-dependencies` 部分中，添加 `bs-fetch` 和 `bs-json`：

```ocaml
{
"name": "reason-scripts",
"sources": ["src"],
"bs-dependencies": [
    "reason-react",
    "bs-jest","bs-fetch", // add this
    "@glennsl/bs-json" // and this too
],
// ...more stuff
```

您需要终止并重新启动 `yarn start` / `npm start` 命令，以便构建系统可以获取对 `.bsconfig` 的更改。

## 读取 JSON

现在我们已经安装了 `bs-json`，我们可以使用 `Json.Decode` 来读取 JSON 并将其转化为记录。

我们将在 `RepoData.re` 的末尾定义一个名为 `parseRepoJson` 的函数：

```ocaml
// RepoData.re

type repo = {
    full_name: string,
    stargazers_count: int,
    html_url: string
};

let parseRepoJson = (json: Js.Json.t): repo => {
    full_name: Json.Decode.field("full_name", Json.Decode.string, json),
    stargazers_count: Json.Decode.field("stargazers_count", Json.Decode.int, json),
    html_url: Json.Decode.field("html_url", Json.Decode.string, json)
}

```

我们定义了一个名为 `parseRepoJson` 的函数，它接受一个名为 `json` 的参数并返回 `RepoData.repo` 类型的值。`Json.Decode` 模块提供了一组函数，我们将它们组合在一起来提取 JSON 的字段，并确保我们得到的值是正确的类型。

## 不要重复自己

这看起来有点罗嗦。我们是否真的必须一遍又一遍地写 `Json.Decode` ？

不，Reason 有一些方便的语法来帮助我们，当我们需要一次又一次地引用特定模块的输出时。一种选择是“打开”模块，这意味着它的所有输出在当前作用域都可用，所以我们可以抛弃 `Json.Decode` 限定符：

```ocaml
Json.Decode;

let parseRepoJson = (json: repo) =>
{
    full_name: field("full_name", string, json),
    stargazers_count: field("stargazers", int, json),
    html_url: field("html_url", string, json)
};

```

但是，如果打开多个模块，这会引起名称冲突的风险。另一种选择是使用模块名称，后跟一个句点`.`在表达之前。在表达式内部，我们可以使用模块的任何导出，而不用模块名称进行限定：

```ocaml
let parseRepoJson = (json: Js.Json.t): repo =>
Json.Decode.{
    full_name: field("full_name", string, json),
    stargazers_count: field("stargazers_count", int, json),
    html_url: field("html_url", string, json),
};
```

注意 `(json: Js.Json.t):repo`。在这里，我们输入预期的 json 值作为 `Js.Json.t`，也就是说传入的 `repo` `json` 类型必须是(json:Js.Json.t)。请参阅 [@nikgraf](https://twitter.com/@nikgraf) 的 [egghead系列 Reason 类型参数视频](https://egghead.io/lessons/reason-type-parameters-in-reason) 以了解更多信息。事实上，如果你对Reason感兴趣并且没有看过，现在就去看看，然后再回来。然后每周观看一次，直到你找到它。每次你都会学到一些东西。

现在让我们通过添加一些代码来测试它，该代码定义了一个 JSON 字符串并使用我们的 `parseRepoJson` 函数来解析它。

在app.re中：

```ocaml
dummyRepos: array(RepoData.repo) = [|
    RepoData.parseRepoJson(
        Js.Json.parseExn(
            {js|
                {
                    "stargazers_count": 93,
                    "full_name": "reasonml/reason-tools",
                    "html_url": "https://github.com/reasonml/reason-tools"
                }
            |js}
        )
    )
|];
```

不要担心理解什么 `Js.Json.parseExn` 或奇怪的 `{js | ... | js}` 的东西（这是一个可选的[字符串字面量语法](https://bucklescript.github.io/bucklescript/Manual.html#_bucklescript_annotations_for_unicode_and_js_ffi_support)）。返回到浏览器，您应该看到从该 JSON 输入成功呈现页面。

![1_vJeQUw9FWZBb2lK_muJnaQ.png](/images/1_vJeQUw9FWZBb2lK_muJnaQ.png)

请参阅 repo 标签 [bs-json-parsing](https://github.com/idkjs/decoding-json-in-reason-react/tree/bs-json-parsing)。

## 获取数据

请参阅 [fetching-data](https://github.com/idkjs/decoding-json-in-reason-react/tree/fetching-data)

查看 [Github API 响应](https://api.github.com/search/repositories?q=topic%3Areasonml&type=Repositories)的形式，我们对items 字段感兴趣。

![1_Tc7TgXVmGv4i_xt4Lz4dxA.png](/images/1_Tc7TgXVmGv4i_xt4Lz4dxA.png)

该字段包含一个 repos 数组。我们将添加另一个函数，它使用我们的 `parseRepoJson` 函数将 items 字段解析为一个记录数组。

在 `RepoData.re`中：

```ocaml
parseReposResponseJson = json => 
    Json.Decode.field("items", Json.Decode.array(parseRepoJson), json);
```

最后，我们将使用 `bs-fetch` 包来向 API 发送 HTTP 请求。

但首先，更多的新语法！我保证这是最后一点。管道运算符 `|>` 只是将 `|>` 运算符左边的表达式的结果取出来，并使用该值调用 `|>` 运算符右边的函数。

例如，以下代码：

```ocaml
(doThing3(doThing2(doThing1(arg))));
```

通过管道运算符，我们可以做到：

```ocaml
arg |> doThing1 |> doThing2 |> doThing3
```


这让我们可以模拟类似 Javascript 中的 Promises 的链式 API ，不同之处在于 `Js.Promise.then_` 是我们用 promise 作为参数调用的函数，而不是 promise 对象上的方法。

在 `RepoData.re` 中：

```ocaml

let reposUrl = "https://api.github.com/search/repositories?q=topic%3Areasonml&type=Repositories";

let fetchRepos = () => {
    Fetch.fetch(reposUrl)
    |> Js.Promise.then_(Fetch.Response.text)
    |> Js.Promise.then_(
        jsonText =>
            Js.Promise.resolve(parseReposResponseJson(Js.Json.parseExn(jsonText)))
    );
}

```

我们可以通过暂时打开 `Js.Promise` 来使 Promise 链式 `fetchRepos` 更简洁：

```ocaml
fetchRepos = () =>
    Js.Promise.(
        Fetch.fetch(reposUrl)
        |> then_(Fetch.Response.text)
        |> then_(
            jsonText =>
                resolve(parseReposResponseJson(Js.Json.parseExn(jsonText)))
        )
    );
```

最后，回到 `App.re` 中，我们将添加一些代码来加载数据并将其存储在组件状态中：

```ocaml
type state = {
    repoData: option(array(RepoData.repo))
};

type action = 
    | Loaded(array(RepoData.repo));

let component = ReasonReact.reducerComponent("App");

let dummyRepos: array(RepoData.repo) = [|
    RepoData.parseRepoJson(
        Js.Json.parseExn(
            {js|
                {
                    "stargazers_count": 93,
                    "full_name": "reasonml/reason-tools",
                    "html_url": "https://github.com/reasonml/reason-tools"
                }
            |js},
        ),
    ),
|];

let repoItems = (repoData: option(array(RepoData.repo))) =>
    switch (repoData) {
        | Some(repos) =>
            ReasonReact.array(
                Array.map(
                    (repo: RepoData.repo) => <RepoItem key=repo.full_name repo />,
                    repos,
                ),
            )
        | None => ReasonReact.string("Loading")
    };

let reducer = (action, _state) => 
    switch (action) {
        | Loaded(loadedRepo) => ReasonReact.Update({
            repoData: Some(loadedRepo)
        })
    };

let make = _children => {
    ...component,
    initialState: () => {
        repoData: None
    },

    didMount: self => {
        let handleReposLoaded = repoData => self.send(Loaded(repoData));

        RepoData.fetchRepos()
        |> Js.Promise.then_(repoData => {
                handleReposLoaded(repoData);
                Js.Promise.resolve();
            })
        |> ignore;
    },

    reducer,

    render: ({state: {repoData}}) =>
        <div className="App">
            <h1> (ReasonReact.string("Decoding JSON in ReasonReact")) </h1>
            (repoItems(repoData))
        </div>,
};

```

首先我们实现 `didMount` 生命周期方法。我们使用 `self.send` 创建一个名为 `handleReposLoaded` 的函数来处理我们加载的数据并更新组件状态。我们将在我们的 `RepoData.fetchRepos()` 函数中调用它，并将它传递给期望的 `repoData` 值。 `handleReposLoaded` 然后将该值传递给 `self.send`，我们将它传递给定义的 `action` 类型。***self.send 与 `action` 类型一起使用以便使用它，请确保您已经定义了要使用它的 `action` 。*** 因此，使用我们的 `RepoData.fetchRepos()` 函数，我们加载数据。就像 Javascript中的链式 Promise ，我们将其传递到 `Js.Promise.then_` 中，在那里我们用加载的数据调用 `handleReposLoaded` 函数，更新组件状态。

我们通过返回 `Js.Promise.resolve()` 来结束 promise 链。定义 promise 链的整个表达式然后 `|>` 传送到一个称为 `ignore` 的特殊函数，它只是告诉 Reason 我们不打算对 promise 链表达式计算的值做任何事情（我们只关心副作用 它具有调用更新函数的功能）。

这就是它在浏览器中的样子：

![1_lYil0QkDtiH1PQLDrXvg3g.png](/images/1_lYil0QkDtiH1PQLDrXvg3g.png)

## 添加一些 CSS

让我们回到 `index.re`。将此代码添加到文件的顶部：

```ocaml
[%bs.raw {|
require('./index.css');
|}];
```

这`%bs.raw` 的东西允许我们在 `{|` 和 `|}` 之间加入一些简单的 Javascript 代码。在这种情况下，我们只是使用它来以通常的 Webpack 方式包含一个 CSS 文件。保存后，您应该看到应用于应用的一些样式更改。您可以打开为我们制作的 `create-react-app` 的 `index.css` 文件，并根据您的内容定制样式。让我们添加一些 `margin`，以便 ui 不会被推到左侧。打开 `index.css` 并将其更改为：

```ocaml
{
    margin:2em;
    padding:0;
    font-family: sans-serif;
}
```

您还可以通过传递使用 [`ReactDOMRe.Style.make`](https://reasonml.github.io/reason-react/docs/en/style.html#docsNav) 创建的样式属性，在 React 组件中使用内联样式：

```ocaml
style={ReactDOMRe.Style.make(~color="red", ~fontSize="68px")()}
```

就是这样！

你可以在[这里](https://decoding-json-in-reason-react.netlify.com/)看到完成的应用程序。完整的源代码在 [Github](https://github.com/idkjs/decoding-json-in-reason-react) 上可用。

## 奖金：使用 Netlify 和 git 子树部署 Demo。

然后 `yarn build` 生成以获得项目的生产版本。

从项目的 `.gitignore` 文件中删除 `build` （它由 create-react-app 默认忽略）。

将以下命令添加到 package.json 的脚本键值中：

```ocaml
...
    "deploy": "git subtree push --prefix build origin gh-pages",
...
```

确保 git 知道你的子树（你的网站的子文件夹）。运行：

```ocaml
add build && git commit -m "Initial dist subtree commit"
```

运行 `yarn deploy`。

如果没有，请注册 Netlify 。登录后，点击大蓝 “Git的新站点” 按钮。

选择您的 repo，将目录更改为您的 gh-pages 目录，然后单击 “Deploy Site” 按钮。

![1_UbMYnovAa1GUt0PYsO7x9A.png](/images/1_UbMYnovAa1GUt0PYsO7x9A.png)

## 谢谢！

如果您对本文有任何反馈，可以发推文给我：[@_idkjs](https://www.twitter.com/@_idkjs)。感谢 [@ur_friend_james](https://www.twitter.com/@ur_friend_james) 的原始帖子，可以在[这里](https://jamesfriend.com.au/a-first-reason-react-app-for-js-developers)找到。
