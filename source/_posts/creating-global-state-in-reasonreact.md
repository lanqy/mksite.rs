---
title: 在 ReasonReact 中创建全局状态
description: 在 ReasonReact 中创建全局状态
created: 2018/06/21
author: lanqy
---
# 在 ReasonReact 中创建全局状态

译自：https://medium.com/@Hehk/creating-global-state-in-reasonreact-f84701c6ab6

默认情况下，ReasonReact 为通过使用 Reducer 组件管理有状态组件提供了一个解决方案。Reducer 组件对于管理整个系统中的小部分状态非常好，但在更大的应用程序中遇到一些严重问题。具有更持久的全局状态，使用减速器体系结构并将状态提升到组件之外将提供两全其美。

## 设置一个基本的 Reducer 组件

ReasonReact 减速器组件是管理 ReasonReact 应用程序中的状态的基本方式。他们通过建立一个 state ，一系列 actions 和一个 reducer 来实现。

- state：组件的当前状态
- action：一组可以修改状态的动作
- reducer：一个函数，它需要一个动作和一个状态来计算一个新的状态。

```ocaml
type state = {count: int};

/* the actions */
type action = 
    | Increment
    | Decrement;

let component = ReasonReact.reducerComponent("Counter");

let make = (_children) => {
    ...component,
    /* the state */
    initialState: () => {
        count: 0
    },

    /* the reducer */

    reducer: (action, state) => 
        switch action {
            | Increment => ReasonReact.Update({count: state.count + 1})
            | Decrement => ReasonReact.Update({count: state.count - 1})
        },
    render: ({state, send}) =>
        <div>
            <h1> (ReasonReact.string(string_of_int(state.count))) </h1>
            <button onClick=((_e) => send(Increment))> (ReasonReact.string("+")) </button>
            <button onClick=((_e) => send(Decrement))> (ReasonReact.string("-")) </button>
        </div>
}
```

这个组件是一个简单的计数器，它通过reducer组件体系结构进行变异。

1. 用户点击“+”按钮，发送增量动作
2. 减速器在动作和当前状态下被触发
3. Reducer以新状态返回ReasonReact.Update

就是这样，（除了背景布线）这个减速器组件是如何经历变异的。

一个更复杂的例子是待办事项列表。

```ocaml
type filter = 
    | Completed
    | UnCompleted
    | None;

type item = {
    name: string,
    completed: bool
};

type state = {
    input: string,
    items: list(item),
    filter
};

let initialState = () => {
    input: "test",
    items: [{name: "initial item", completed: true}, {name: "kewl item", completed: false}],
    filter: None
}

type action = 
    | AddItem(item)
    | RemoveItem(item)
    | ChangeInput(string)
    | ToggleItem(string)
    | ChangeFilter(filter);
    
let reducer = (action, state) =>
    switch action {
        | AddItem(item) => ReasonReact.Update({...state, items: [item, ...state.items], input: ""})
        | RemoveItem(item) =>
            ReasonReact.Update({...state, items: List.filter((elem: item) => elem.name != item.name, state.items)})
        | ChangeInput(input) => ReasonReact.Update({...state, input})
        | ToggleItem(name) => ReasonReact.Update({
            ...state,
            items: List.map((item) => name == item.name ? {...item, completed: !item.completed }: item, state.items)
        })
        | ChangeFilter(filter) => ReasonReact.Update({...state, filter})
    };

let createItem = (~name, ()) => {name, completed: false};

let component = ReasonReact.reducerComponent("App");

let make = (_children) => {
    ...component,
    render: ({state, send}) => 
        <div>
            <input value=state.input onChange=((e) => send(ChangeInput(Obj.magic(e)##target##value))) />
            <button onClick=((_e) => send(AddItem(createItem(~name=state.input, ()))))>
                (ReasonReact.string("add"))s
            </button>
        </div>
}

```

虽然这个例子中有很多代码，但系统实际上相当简单。所有操作都是 actions，它们通过 reducer 映射到简单的状态更改。如果你感到困惑，我会建议你走进一个动作的生命周期，这些部分应该是有意义的。

我强烈建议您阅读文档，他们会详细解释不同类型的更新以及与 reducer 组件交互的更有趣方式。https://reasonml.github.io/reason-react/docs/en/state-actions-reducer.html

## 减速器组件的缺点

上述示例的最大问题是状态本质上与组件绑定。例如，如果删除组件，则状态将丢失，如果丢失状态，组件将从其初始状态重新启动。在处理路由等问题时，这种短暂的状态成为一个巨大的问题，因为无法在URL更改之间维护您的状态。

## 你怎么解决这个问题

全局状态，通过将状态从这些短暂的组件中移出，我们可以创建一个可以轻松处理被删除和重新创建的应用程序。

怎么样？简单，减速器组件！ ：P

上面的体系结构可以通过创建处理状态的父组件向上移动到组件树中，并且仅向下传递我们获得两全其美所需的内容。减速器组件模型和状态与短暂组件分离。

```ocaml
type element =
  | Todo
  | Counter;

/* 将 state 合并为一个全球 state */
type state = {
  todo: Todo.state,
  counter: Counter.state,
  activeElement: element
};

/* 将来自 todo 和 counter 的动作组合到全局动作包装器中 */
type action =
  | Todo(Todo.action)
  | Counter(Counter.action)
  | ChangeElement(element);

let component = ReasonReact.reducerComponent("App");

let make = (_children) => {
  ...component,
  /* 结合所有初始状态 */
  initialState: () => {todo: Todo.initialState(), counter: Counter.initialState(), activeElement: Todo},
  /* 让组件减速器处理状态部分 */
  reducer: (action, state) =>
    switch action {
    | Todo(todoAction) => ReasonReact.Update({...state, todo: Todo.reducer(todoAction, state.todo)})
    | Counter(counterAction) =>
      ReasonReact.Update({...state, counter: Counter.reducer_2(counterAction, state.counter)})
    | ChangeElement(element) => ReasonReact.Update({...state, activeElement: element})
    },
  render: ({state, send}) =>
    <div>
      <button onClick=((_e) => send(ChangeElement(Todo)))> (ReasonReact.stringToElement("todo")) </button>
      <button onClick=((_e) => send(ChangeElement(Counter)))> (ReasonReact.stringToElement("counter")) </button>
      <h1>(ReasonReact.stringToElement("Component"))</h1>
      (
        /* 将一段状态传递给组件 */
        switch state.activeElement {
        | Todo => <Todo todoState=state.todo dispatch=((action) => send(Todo(action))) />
        | Counter => <Counter counterState=state.counter dispatch=((action) => send(Counter(action))) />
        }
      )
    </div>
};
```

这确实需要对原始组件进行微小修改。

```ocaml
type state = {count: int};

/* 将初始状态移出组件 */
let initialState = () => {count: 0};

type action =
  | Increment
  | Decrement;

/* 将 reducer 移出组件 */
let reducer = (action, state) =>
  switch action {
  | Increment => {count: state.count + 1}
  | Decrement => {count: state.count - 1}
  };

let component = ReasonReact.statelessComponent("Counter");

let make = (~dispatch, ~counterState, _children) => {
  ...component,
  render: (_self) =>
    <div>
      <h1> (ReasonReact.stringToElement(string_of_int(counterState.count))) </h1>
      /* 使用传入的调度函数而不是self.send */
      <button onClick=((_e) => dispatch(Increment))> (ReasonReact.stringToElement("+")) </button>
      <button onClick=((_e) => dispatch(Decrement))> (ReasonReact.stringToElement("-")) </button>
    </div>
};
```
如果你想看到 todo 组件，它在最后链接的源代码中，应用程序托管在http://hehk.github.io/example-reason-react-global-state

正如您所看到的，这些更改非常小，我所做的只是创建一个具有子状态并集的父组件，并让它负责更新该新状态。我们也只是改变原始组件中的一些函数调用来使用新的调度函数，我们是金色的。

![示例演示截图](/images/1__kAXzPNzq0y1fSyyOjOzOA.gif)

管用!

## 优点

- 这将状态与组件分离，您实际上并不需要将 reducer 和初始状态放在同一模块中。它们可能完全不同，处理方式不同。例如，您可以创建一个包含状态，操作和缩减器的语言模块，但没有特定的语言组件，其状态只会传递给组件
- 您现在可以通过删除和重新创建元素来持续存在

## 这个方法有问题

- 这个解决方案相当简单，可能会很快变得难看。
- 您必须通过 props 将状态显式传递给组件。 **您可以在州周围创建一个 observable，并将所有子组件订阅为一种可能的解决方法。 **
- 除了通用更新之外的任何更改都需要重写，目前只有子减速器返回状态。你可以让他们返回一个状态和更新类型的元组，但这也只是一个更多的样板。

我希望在 Reason / OCaml 上有一个更好的人最终会推出一个优雅的解决方案来打破这个代码，但是，就目前而言，我认为这是一种创建全局状态的简单方法。

## 关闭笔记

感谢阅读，我是新来写博客文章，所以希望它并不可怕。我之所以写这篇文章，是因为我是工作范例的忠实粉丝，我找不到更有趣的状态管理。如果您认为有什么不对或可以更好地解释，请告诉我。

如果您想了解有关使用 reducer 组件的更多信息，请访问https://reasonml.github.io/reason-react/docs/en/state-actions-reducer.html 查看文档。


此帖子的所有代码均来自以下示例应用：https://github.com/Hehk/example-reason-react-global-state
