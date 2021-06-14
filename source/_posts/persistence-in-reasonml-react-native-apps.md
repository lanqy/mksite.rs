---
title: OCaml 符号ReasonML 中的持久性 React Native Apps
created: 2018/05/28
description: ReasonML 中的持久性 React Native Apps
author: lanqy
---
# ReasonML 中的持久性 React Native Apps

译自： https://jwheatley.co/persistence-in-reasonml-react-native-apps/

> 让我们学习如何通过翻转 `switch` 来保持 ReasonML RN 应用程序中的状态！

## 什么是ReasonML

ReasonML（或作为非SEO友好缩写的 Reason）是来自 Facebook 的开源项目，它使得 OCaml 易于用于 JS 开发人员。它通过为语言提供更友好的语法并提供与 JS 生态系统互操作性如黄油一样平滑的工具。

OCaml 有一个超强的类型系统，它可以取代 Flow 或 TypeScript 提供的任何东西，而且 ReasonML 与工具捆绑在一起，可以替代几乎所有的工具来让 JS 每天都可以编写（ESlint→ReasonML / OCaml 类型系统 ，Flow / TS → ReasonML 语法，Prettier → refmt，JS / ES 模块捆绑器 → ReasonML / OCaml 模块，Babel → BuckleScript 等）。

ReasonML 受 Facebook 支持的重要之处在于它通过 ReasonReact 对 React 项目提供一流的支持，所以如果您厌倦了与 Flow 错误作斗争，并且仍然对 TypeScript 如何适应 React 生态系统感到困惑，那么您应该试一试 ReasonML！

本指南旨在帮助您将状态持久性集成到您的ReasonML React Native 应用程序中，并促进为开发 ReasonReact 应用程序的人创建良好的脱机体验，而无需借助基于 JS 的状态管理解决方案（例如：Redux + Redux-Persist）。

## 入门

要开始编写 ReasonReact Native 应用程序，我建议您使用 `create-react-native-app` 路线：

假设您已经安装了最新版本的 [Node.js](https://nodejs.org/en/)，请在终端中运行这些命令

```ocaml
npm install -g create-react-native-app
    create-react-native-app Switcheroo --scripts-version reason-react-native-scripts
    cd Switcheroo
    npm start
    # 最好是，我会在前面的命令前加上`code。 &&`
    # 在运行启动脚本之前在VS代码中打开它
    # 以防止需要额外努力在编辑器中打开
```

然后，如 CRNA 文档（截至1/1/2018）所详述：

> 在您的 iOS 或 Android 手机上安装 [Expo](https://expo.io/) 应用程序，并使用终端中的 QR 码打开您的应用程序。在应用程序的“项目”选项卡中查找 QR 扫描仪。

## UI 脚手架

现在我们已经启动了开发环境并运行了，现在是时候进入有趣的部分并编写一些代码了!

对于初学者，您需要创建一个名为Switcheroo.re的新文件，该文件将位于App.re旁边的src文件夹中。这是我们将为我们的切换创建逻辑的地方。

在这个文件中，我们为一个简单的 RN Switch 组件设置了基础，处理 [ReasonReact](https://reasonml.github.io/reason-react/docs/en/state-actions-reducer.html) 中提供的 `reducerComponent` 构件块的状态。

```ocaml

/* Switcheroo.re */

open BsReactNative;

type state = {toggled: bool};

type action = 
    | SetSwitchValue(bool);

let component = ReactReact.reducerComponent("Switcheroo);

let make = (_children) => {
    ...component,
    initialState: () => {toggled: false},
    reducer: (action, _state) =>
        switch action {
            | SetSwitchValue(v) => ReasonReact.Update({toggled: v})
        },
    render: (self) =>
        <Switch
            value=self.state.toggled
            onTintColor="#DD4C39"
            onValueChange=((value) => self.reduce(() => SetSwitchValue(value), ()))
        />
};

```

在App.re中，您可以用新编写的 Switcheroo 组件替换 View 的内部（下面的代码）。

```ocaml

/* App.re */

open BsReactNative;

let app = () => 
    <View style=Style.(style([flex(1.), justifyContent(Center), alignItems(Center)]))>
        <Switcheroo />
    </View>;
```

很好，我们有一个很酷的 `switch` ，我们可以来回切换，但是为什么当我重新打开应用程序的时候，它不会保持开启? 哦，是的，我们忘记了在本地保存状态;让我们去做吧!

## 持久性

要在 React Native中 保持状态，您必须使用 AsyncStorage 模块。这使您可以将序列化数据设置为长期数据存储并在以后检索它，即使应用程序已关闭并由用户重新启动，也可使应用程序停留在其数据上。

上一节中的关键词是“序列化”，这意味着您的数据必须转换为字符串格式才能保存并从字符串中恢复并解析回实时数据结构以在您的应用中使用它。

为了在 ReasonML 中做到这一点，我们需要调用 bs-json 的强大功能，它为使用 JSON 结构提供帮助。

- 首先，您需要运行 `npm i -S bs-json` 来安装软件包。
- 接下来，将它添加到你的 bsconfig.json 的 bs-dependencies 数组中。
- 当你在它的时候，改变你的 bsconfig.json 的名字道具为 “Switcheroo”。

完成后，bsconfig.json（支持 ReasonML 开发的 BuckleScript 工具链的配置文件）应如下所示：

```json
{
  "name": "Switcheroo",
  "reason": {
    "react-jsx": 2
  },
  "bsc-flags": ["-bs-super-errors"],
  "bs-dependencies": ["bs-react-native", "reason-react", "bs-json"],
  "sources": [
    {
      "dir": "src"
    }
  ],
  "refmt": 3
}
```

现在，让我们支撑我们的持久性函数，并将其设置为在组件更新状态时运行。

```ocaml
open BsReactNative;
type state = {toggled: bool};
type action =
  | SetSwitchValue(bool);
let persist = state => {
  /* convert state to JSON */
  /* set it in RN's AsyncStorage */
  ()
};
let component = ReasonReact.reducerComponent("Switcheroo");
let make = (_children) => {
  ...component,
  initialState: () => {toggled: false},
  reducer: (action, _state) =>
    switch action {
    | SetSwitchValue(v) => ReasonReact.Update({toggled: v})
    },
  didUpdate: ({newSelf}) => persist(newSelf.state),
  render: (self) =>
    <Switch
      value=self.state.toggled
      onTintColor="#DD4C39"
      onValueChange=((value) => self.reduce(() => SetSwitchValue(value), ()))
    />
};
```

在这个函数中，我们需要使用 bs-json 将我们的状态编码为 JSON 并将其设置为我们的 AsyncStorage 位置，即 “Switcheroo.state”。

```ocaml
/* Switcheroo.re (partial) */
let persist = (state) => {
  /* convert state to JSON */
  let stateAsJson =
    Json.Encode.(object_([("toggled", Js.Json.boolean(Js.Boolean.to_js_boolean(state.toggled)))]))
    |> Js.Json.stringify;
  /* set it in RN's AsyncStorage */
  AsyncStorage.setItem(
    "Switcheroo.state",
    stateAsJson,
    ~callback=
      (e) =>
        switch e {
        | None => ()
        | Some(err) => Js.log(err)
        },
    ()
  )
  |> ignore
};
```

所以，现在，如果您现在查看您的应用程序并翻转开关几次，您会发现没有任何问题（这太棒了！），但是当您刷新应用程序时，您会注意到您的应用程序仍然没有正常工作，捡起它离开的地方。

## Re-hydration（美化或优化）

为了重新维护我们的状态，我们需要：

- 创建一个 re-hydrate `action` 来更新我们 reducerComponent 的状态
- 创建一个 rehydrate 函数，从 AsyncStorage 中检索 JSON 并将其解码为一个 ReasonML 记录
- 设置 Switcheroo 组件在组件激活时调用我们的 Rehydrate 函数

满足上述步骤的代码在这里：

```ocaml
/* Switcheroo.re */
open BsReactNative;
let storageKey = "Switcheroo.state";
type state = {toggled: bool};
type action =
  | SetSwitchValue(bool)
  | Rehydrate(state);
let persist = (state) => {
  /* convert state to JSON */
  let stateAsJson =
    Json.Encode.(object_([("toggled", Js.Json.boolean(Js.Boolean.to_js_boolean(state.toggled)))]))
    |> Js.Json.stringify;
  /* set it in RN's AsyncStorage */
  AsyncStorage.setItem(
    storageKey,
    stateAsJson,
    ~callback=
      (e) =>
        switch e {
        | None => ()
        | Some(err) => Js.log(err)
        },
    ()
  )
  |> ignore
};
let rehydrate = (self) => {
  Js.Promise.(
    /* begin call to AsyncStorage */
    AsyncStorage.getItem(storageKey, ())
    |> then_(
          (json) =>
            (
              switch json {
              | None => ()
              | Some(s) =>
                /* parse JSON, decode it into a ReasonML Record, and reset the state */
                let parsedJson = Js.Json.parseExn(s);
                let state = Json.Decode.{toggled: parsedJson |> field("toggled", bool)};
                self.ReasonReact.reduce(() => Rehydrate(state), ());
                ()
              }
            )
            |> resolve
        )
    |> ignore
  );
  ReasonReact.NoUpdate
};
let component = ReasonReact.reducerComponent("Switcheroo");
let make = (_children) => {
  ...component,
  initialState: () => {toggled: false},
  reducer: (action, _state) =>
    switch action {
    | SetSwitchValue(v) => ReasonReact.Update({toggled: v})
    | Rehydrate(s) => ReasonReact.Update(s)
    },
  didUpdate: ({newSelf}) => persist(newSelf.state),
  didMount: (self) => rehydrate(self),
  render: (self) =>
    <Switch
      value=self.state.toggled
      onTintColor="#DD4C39"
      onValueChange=((value) => self.reduce(() => SetSwitchValue(value), ()))
    />
};
```

现在，你应该有一个很酷的小应用程序，让你切换一个 ReasonML 主题的开关并保持它的状态。甜！

有关 ReasonML 中持久化和重新保持 JSON 状态的更复杂示例，请查看我为我的个人健身追踪器编写的代码。

我希望你从这篇文章中学到了很多东西，并在阅读时享受自己的乐趣！ 欲了解更多内容，你可以在 [Twitter](https://twitter.com/FiberJW) 上关注我！ 这是与我联系并了解我正在做的新事情，我正在探索的新想法或正在学习的新技术的最简单的地方！

祝你有个好的一天！

—--- Juwan