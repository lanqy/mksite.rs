---
title: ReasonML 入门
created: 2018/05/28
description: ReasonML 入门
author: lanqy
---
# ReasonML 入门

译自： https://dev.to/jlewin_/reasonml-getting-started-53gi


在本教程中，我们将使用Reason构建一个小型天气应用程序。有一个链接到页面底部的源代码。本教程假设您对React有基本的了解，因为我们将使用ReasonReact绑定来构建应用程序。如果您以前没有使用过 React，那么[这篇文章](https://dev.to/tylermcginnis/a-comprehensive-guide-to-reactjs-in-2018--4nbc)是一个很好的开始。

## 什么是 Reason

Reason 是 OCaml 的新语法，由 Facebook 开发，受 JavaScript 影响很大。它有 100％ 的类型覆盖率，这导致了一个非常强大的类型系统。

Reason 也适用于跨平台开发。我们可以使用 BuckleScript 将我们的代码编译成（可读的）JavaScript，从而打开整个 Web 平台。感谢 OCaml，它也可以使用 Reason 进行本地开发。

此外，Reason 还可以访问整个 JS 和 OCaml 生态系统，并提供 ReasonReact 以使用 ReactJS 构建 UI 组件。文档中有一个[有用的页面](https://reasonml.github.io/docs/en/what-and-why.html)，可以更详细地解释优势！

## 要求

首先，让我们确保我们安装了正确的工具。

我们将使用 Create React App 引导项目。如果您之前没有使用过，请通过运行 `npm i -g create-react-app` 进行安装。还有两个我们需要开始的软件包：

- Reason CLI: Reason 工具链。[检查安装文档](https://github.com/reasonml/reason-cli#1-choose-your-platform)。
- 在撰写本文时，macOS用户可以通过运行 `npm i -g reason-cli@3.1.0-darwin`。
- BuckleScript: `npm i -g bs-platform`。

我也使用 [vscode-reasonml](https://github.com/reasonml-editor/vscode-reasonml) 编辑器插件。如果您使用的是其他编辑器，请检查[插件列表](https://reasonml.github.io/docs/en/editor-plugins.html)以找到适合您的插件。

## 我们的第一个组件

要开始，我们将为我们的应用程序创建样板代码：

`create-react-app weather-app --scripts-version reason-scripts`

这给了我们一个基本的 App 组件：

```ocaml
[%bs.raw {|require('./app.css')|}];

[@bs.module] external logo : string = "./logo.svg";

let component = ReasonReact.statelessComponent("App");

let make = (~message, _children) => {
    ...component,
    render: (_self) =>
        <div className="App">
            <div className="App-header">
                <img src=logo className="App-logo" alt="logo" />
                <h2> (ReasonReact.stringToElement(message)) </h2>
            </div>
            <p className="App-intro">
                (ReasonReact.stringToElement("To get started, edit"))
                <code> (ReasonReact.stringToElement(" src/app.re ")) </code>
                (ReasonReact.stringToElement("and save to reload."))
            </p>
        </div>
};
```

我们可以使用 `yarn start` 开始编译和运行。我们来看看一些有趣的部分......

```ocaml
[%bs.raw {|require('./app.css')|}];
```

BuckleScript 允许我们将原始的 JavaScript 代码混合到我们的代码中，从一个一行代码到一个完整的库(如果我们只是在 hacking)。这应该很少使用，但是在我们开始的时候可以快速开始。

```ocaml
let component = ReasonReact.statelessComponent("App");
```

我们将使用两种类型的ReasonReact组件：`statelessComponent` 和 `reducerComponent`。无状态组件按照他们在锡上所说的话做。 Reducer组件是有状态的，并且内置了类似 Redux 的 reducers。 我们稍后再讨论。

```ocaml
let make = (~message, _children) => { ... }
```
这是定义我们组件的方法。 这两个参数具有不同的符号：`〜` 是一个带标签的参数，意味着我们可以通过名称引用参数，而 `_` 是一种更明确的方式显示参数未被使用（否则编译器会给我们一个警告）。

`...component` 扩展运算符意味着我们的 make 函数正在构建我们刚定义的组件，覆盖默认值。

```ocaml
<h2> (ReasonReact.stringToElement(message)) </h2>
```

JSX 中的 Reason 比正常的 React 更严格。我们不能仅仅编写 `<h2> {message} </h2>`，而是必须将 `message` 字符串显式转换为 JSX 元素。

稍后我们将构建自己的组件时，我们将使用此样板。

## Reason 中的类型

我们创建一个新文件 `WeatherData.re`。这将为我们的天气记录定义数据结构和任何相关方法。首先，我们来创建一个类型：

```ocaml
type weather = {
    summary: string,
    temp: float
};
```

在这个文件中，我们可以使用这个数据结构创建新记录，编译器会知道它是一个 Weather 项目。从其他文件中，我们需要告诉编译器该类型是什么。在 Reason 中，[文件可以作为模块引用](https://reasonml.github.io/docs/en/faq.html#i-don-t-see-any-import-or-require-in-my-file-how-does-module-resolution-work)，这意味着我们不必显式导入它们！我们可以这样做：

```ocaml
let today: WeatherData.weather = {
    summary: "Warm throughout the day",
    temp: 30.5
};
```

我之前提到 Reason 有 100％ 的类型覆盖率，但我们只定义了我们的 Weather 类型......其余覆盖范围从哪里来？我们可以明确地为每个我们使用的变量定义一个类型，例如：`let greeting: string = "Hello"`;但幸运的是 OCaml 系统可以为我们推断类型。所以，如果我们写 `let greeting = "Hello"` ;编译器仍然会知道 greeting 是一个字符串。这是 Reason 中的一个关键概念，可确保类型安全。

## 保持状态

回到我们的项目，让我们修改app.re，以便它可以存储我们想要显示的数据。这将涉及：

- 定义我们的状态类型
- 设置我们的初始状态（目前有一些虚拟数据）
- 定义可应用于状态的操作(`actions`)
- 定义组件的 `reducers` 来处理这些事件

`Actions` 定义了我们可以对操作状态做不同的事情。例如，`Add` 或 `Subtract`。 Reducers 是纯粹的函数，它定义了这些动作如何影响状态，就像在 Redux 中一样。他们采取 `action` 和我们以前的状态作为参数，并返回一个[更新类型](https://reasonml.github.io/reason-react/docs/en/state-actions-reducer.html#state-update-through-reducer)。

```ocaml
type state = {
    weather: WeatherData.weather
};

type action = 
    | WeatherLoaded(WeatherData.weather);

let component = ReasonReact.reducerComponent("App");

let dummyWeather: WeatherData.weather = {
    summary: "Warm throughout the day",
    temp: 30.5
};

let make = (_children) => {
    ...component,

    initaState: () => {
        weather: dummyWeather
    },

    reducer: (action, _prevState) => {
        switch action {
            | WeatherLoaded(newWeather) =>
              ReasonReact.Update({
                  weather: newWeather
              })
        }
    },

    render: (self) =>
        <div className="App">
            <p> (ReasonReact.stringToElement(self.state.weather.summary)) </p>
        </div>
};

```

这里有两个新的 Reason 概念：变体和模式匹配。

```ocaml
type action = 
    | WeatherLoaded(WeatherData.weather);
```

这是一个变体：代表不同值的选择的数据结构（像枚举）。变体中的每个案例都必须大写，并且可以选择接收参数。在 ReasonReact 中，`action` 表示为变体。这些可以与 switch 表达式一起使用：

```ocaml
switch action {
    | WeatherLoaded(newWeather) =>
      ReasonReact.Update({...})
}
```

这是 Reason 中最有用的功能之一。这里我们是基于我们在 `reducer()` 方法中接收到的参数的模式匹配 `action`。如果我们忘记处理一个案例，编译器知道，并会告诉我们！

![错误提示例子](/images/sfutmltmhu1fmzzjs9zs.png)
Reason 编译器捕获未处理的案例。

在前面的例子中，我们使用解构来访问 newWeather 的值。我们也可以使用它来根据它们包含的值匹配 `actions`。这给了我们一些[非常强大的行为](https://reasonml.github.io/docs/en/pattern-matching.html)！

## 获取数据

到目前为止，我们的应用呈现虚拟天气数据 - 现在让我们从 API 中加载它。我们将把获取和解析数据的方法放在我们现有的 WeatherData.re 文件中。

首先，我们需要安装 [bs-fetch](https://github.com/reasonml-community/bs-fetch) ：`npm i bs-fetch` 和 [bs-json](https://github.com/glennsl/bs-json)：`npm i @glennsl/bs-json`。我们还需要将它们添加到我们的 `bsconfig.json` 中：

```ocaml
{
    ...
    "bs-dependencies": [
        "bs-fetch",
        "@glennsl/bs-json"
    ]
}
```

我们将使用 [Yahoo Weather API](https://developer.yahoo.com/weather) 来获取我们的数据。我们的 `getWeather()` 方法将调用 API，然后使用`parseWeatherResultsJson()` 解析结果，然后解析天气项目：

```ocaml
type weather = {
    summary: string,
    temp: float
};

let url = "https://query.yahooapis.com/v1/public/yql?q=select%20item.condition%20from%20weather.forecast%20where%20woeid%20in%20(select%20woeid%20from%20geo.places(1)%20where%20text%3D%22london%22)%20AND%20u%3D%22c%22&format=json&env=store%3A%2F%2Fdatatables.org%2Falltableswithkeys";

let parseWeatherJson = json: weather => 
    Json.Decode.{
        summary: field("text", string, json),
        temp: float_of_string(field("temp", string, json))
    };

let parseWeatherResultsJson = json =>
    Json.parseOrRaise(json) |> Json.Decode.(at([
        "query",
        "results",
        "channel",
        "item",
        "condition"
    ], parseWeatherJson));

let getWeather = () =>
    Js.Promise.(
        Bs_fetch.fetch(url)
            |> then_(Bs_fetch.Response.text)
            |> then_(
                jsonText => {
                    let result = parseWeatherResultsJson(jsonText);
                    resolve(result);
                }
            )
    );

```

```ocaml
Josn.parseOrRaise(json) |> Json.Decode.(at([
    ...
], parseWeatherJson));
```

这会在通过指定字段遍历数据之前解析 JSON 字符串响应。然后它使用 parseWeatherJson() 方法来解析在条件字段中找到的数据。

```ocaml
Json.Decode.{
    summary: field("text", string, json),
    temp: float_of_string(field("temp", string, json))
};
```

在这个片段中，字段和字符串是 `Json.Decode` 的属性。这个新的语法“打开” `Json.Decode`，所以它的属性可以在大括号内自由使用（而不是重复`Json.Decode.foo`）。该代码生成一个 `weather` 项目，使用 `text` 和 `temp` 字段分配 `summary` 和 `temp`。

`float_of_string` 完全符合你的期望：它将字符串中的温度（从API获得）转换为浮点数。

## 更新状态

现在我们有一个返回 `promise` 的 `getWeather()` 方法，我们需要在 App 组件加载时调用它。ReasonReact 对 React.js 有一组类似的生命周期方法，但有一些[细微差别](https://reasonml.github.io/reason-react/docs/en/lifecycles.html)。我们将使用 `didMount` 生命周期方法使 API 调用获取天气。

首先，我们需要改变我们的状态，以表明可能没有状态的天气项目 - 我们将摆脱虚拟数据。 `option()` 是 Reason 中的一个内置变体，它描述了一个“`nullable`（可空）”值：

```ocaml
type option('a) = None | Some('a);
```

我们需要在我们的状态类型和初始状态中指定 `None`，并在 `WeatherLoaded` reducer 中指定 `Some(weather)`：

```ocaml
type state = {
    weather: option(WeatherData.weather)
};

// ...

let make = (_children) => {
    ...component,

    initialState: () => {
        weather: None
    },

    reducer: (action, _prevState) => {
        switch action {
            | WeatherLoaded(newWeather) =>
              ReasonReact.Update({
                  weather: Some(newWeather)
              })
        }
    },

    // ...
};
```

现在，我们实际上可以在组件装入时发出 API 请求。查看下面的代码，`handleWeatherLoaded` 是一个将我们的 `WeatherLoaded` action 分派给 reducer 的方法。

注意：重要的是从大多数组件生命周期中返回 `ReasonReact.NoUpdate`。reducer 将在下一次改变中处理所有状态变化。

```ocaml
let make = (_children) => {
    // ...

    didMount: (self) => {
        let handleWeatherLoaded = weather => self.send(WeatherLoaded(weather));
        WeatherData.getWeather()
            |> Js.Promise.then_(
                weather => {
                    handleWeatherLoaded(weather);
                    Js.Promise.resolve();
                }
            )
            |> ignore;

            ReasonReact.NoUpdate;
    },

    // ...
};
```

如果我们现在运行我们的应用程序，我们会遇到错误...我们当前正在尝试呈现关于 `self.state.weather` 的信息，但是直到我们收到来自 API 的响应时，它才设置为 None。让我们更新我们的应用程序组件，以便在等待时显示加载消息：

```ocaml
let make = (_children) => {
    // ... 

    render: (self) => 
        <div className="App">
            <p>
            {
                switch self.state.weather {
                    | None =>
                        ReactReact.stringToElement("Loading weather...");
                    | Some(weather) => 
                        ReactReact.stringToElement(weather.summary);
                }
            }
            </p>
        </div>
};
```

结果...

![效果图](/images/ezgif-3-cf07dc176b.gif)

## 错误处理

我们没有想过的一件事是如果我们无法加载数据会发生什么。如果API停机，或者它返回了我们预料不到的情况呢？我们需要认识到这一点并拒绝承诺：

```ocaml
let getWeather = () =>
    Js.Promise.(
        Bs_fetch.fetch(url)
            |> then_(Bs_fetch.Response.text)
            |> then_(
                jsonText => {
                    switch (parseWeatherResultsJson(jsonText)){
                    | exception e => reject(e);
                    | weather => resolve(weather);
                    };
                }
            )
    );
```

```ocaml
switch (parseWeatherResultsJson(jsonText)) {
    | exception e => reject(e);
    | weather => resolve(weather);
};
```

这个 switch 语句试图解析 API 响应。如果发生异常，它会拒绝承诺那个错误。如果解析成功，该承诺将与天气项目一起解决。

接下来，我们将改变我们的状态，让我们认识到是否发生错误。我们来创建一个新的类型，它向我们之前的 `Some('a)` 或 `None` 添加一个 `Error` 案例。

```ocaml
type optionOrError('a) = 
    | Some('a)
    | None
    | Error;

type state = {
    weather: optionOrError(WeatherData.weather)
};
```

在这样做的同时，我们还需要向渲染函数添加一个 `Error` 案例 - 我会让你自己添加。最后，我们需要创建一个新的 `action`和 `reducer`，以便在我们的 `getWeather()` 承诺被拒绝时使用。

```ocaml
// ...
type action = 
    | WeatherLoaded(WeatherData.weather)
    | WeatherError;

let make = (_children) => {
    ...component,

    didMount: (self) => {
        let handleWeatherLoaded = weather => self.send(WeatherLoaded(weather));
        let handleWeatherError = () => self.send(WeatherError);

        WeatherData.getWeather()
            |> Js.Promise.then_(
                // ...
            )
            |> Js.Promise.catch(
                _err => {
                    handleWeatherError();
                    Js.Promise.resolve();
                }
            )
            |> ignore;

        ReasonReact.NoUpdate;
    },

    reducer: (action, _prevState) => {
        switch action {
            | WeatherLoaded(newWeather) =>
                // ...
            | weatherError =>
              ReasonReact.Update({
                  weather: Error
              })
        }
    },

    // ...
}

```

这些是我们已经使用过的概念，但让用户知道是否出现了问题是很有用的。我们不想让他们挂着“加载”消息！

我们有它，我们的第一个 ReasonReact 网络应用程序。干得不错！我们已经介绍了很多新的概念，但希望您已经看到了使用 Reason 的一些好处。

如果您发现这个有趣的事情，并希望看到另一篇文章，请点击下面的反应让我知道！ ❤️🦄🔖

## 扩展阅读

- [更多的上下文](https://jacklewin.com/2018/getting-started-with-reason)，包括源代码的链接。
- [探索ReasonML和函数式编程](http://reasonmlhub.com/exploring-reasonml) - 一本关于（你猜对了）的免费在线书籍 Reason 和 FP。

## OSS项目

- [bs-jest](https://github.com/glennsl/bs-jest) - Jest 的 BuckleScript 绑定。
- [lwt-node](https://github.com/kennetpostigo/lwt-node) - Node.js API的 Reason 实现
- [reason-apollo](https://github.com/apollographql/reason-apollo)绑定 Apollo 客户端和 React Apollo

## 其他

- [Discord 频道](https://discord.gg/reasonml)
- [论坛](https://reasonml.chat/)
- [Reason Town](https://reason.town/) - ReasonML 语言和社区的播客
- [Redex](https://redex.github.io/) - Reason 包的索引