---
title: ReasonML - React 为首要目标
created: 2018/05/28
description: ReasonML - React 为首要目标
author: lanqy
---
# ReasonML - React 为首要目标

译自：https://www.imaginarycloud.com/blog/reasonml-react-as-first-intended/

ReasonML 是 [Facebook](https://www.facebook.com/) 用来开发 React 应用程序并将其作为 JavaScript 的未来版本推广的新技术（[他们说](https://reasonml.github.io/docs/en/what-and-why.html)ES2030）。在这篇文章中，我将简要介绍一下这项技术。

## 简而言之，ReasonML是：

- 一种编写 React 应用程序的新方法;
- OCaml语义的JavaScript友好语法;
- 静态类型 - 带有类型推断;
- 函数式，但不是纯粹的;
- 主要编译为JavaScript;
- 由 Facebook 和 Bloomberg。

## 以前的 React 与现在的 React 的不同

React 的编程风格比面向对象编程更接近函数式。因此，发现[第一个 React 原型没有在 JavaScript 中实现，而是在 Standard ML中实现](https://reasonml.github.io/docs/en/what-and-why.html)，这并不奇怪。

但是，随着原型开始成熟，作者决定将其移植到 JavaScript 并从那里继续，因为没有成熟的 JavaScript 转换器，也因为当时世界还没有准备好接受这种非主流的编程语言和风格。

因此，React作为与JavaScript编程语言相关的技术而广受欢迎。

尽管在 [JavaScript 生态系统](https://www.imaginarycloud.com/blog/a-javascript-ecosystem-overview/)中取得了这些成功，但有些人开始觉得幕后还有其他相关项目正在发生 - 例如 Redux，[Elm](https://www.imaginarycloud.com/blog/elm-javascript-reinvented-1-overview/) 和 Purescript，开始受到欢迎，从而推动社区的思维更接近 React 最初函数式和静态类型的根源。

这使得 Facebook 相信将 React 本身拉近根本是可行和方便的。如果他们没有那么多已经完成的工作，他们不会这么做。

#### Bucklescript

一些公司正在开发这样的任务关键用户界面, 使用动态或渐进式语言可能会造成无法承受的损失。

[Bloomberg](https://www.bloomberg.com/) 就是这样的公司之一，对 Bloomberg 而言，[张宏波正在尝试 JavaScript 运行时](https://www.techatbloomberg.com/blog/release-1-7-story-behind-bucklescript/)，他意识到将 OCaml 编译器移植到 JavaScript 并在浏览器上运行并不困难。

```ocaml

(* Bucklescript / O'Caml 中的阶乘实现 *)

let rec factorial n = 
    if n <= 0 then
        1
    else
        n * fact(n - 1)

```
现实情况是，***OCaml编译器已经非常模块化***，并且用JavaScript生成的后端替换它的本地代码生成的后端并不是很难。有了这样的后端，甚至可以将OCaml编译器编译为JavaScript，从而***[自行托管Bucklescript编译器](https://bucklescript.github.io/bucklescript-playground/#Quick_Sort)***并在浏览器中运行它。

Bucklescript 诞生并且更好，它由 Bloomberg 以开源软件的形式发布。

```ocaml
(* Bucklescript / O'Caml 中的 FizzBu​​zz 实现 *)

let fizzbuzz i = 
    match i mod 3, i mod 5 with
    | 0, 0 -> "FizzBuzz"
    | 0, _ -> "Fizz"
    | _, 0 -> "Buzz"
    | _ -> string_of_int i

let _ = for i = 1 to 100 do
    print_endline(fizzbuzz i)
done

```

需要注意的是，原始的 OCaml 编译器已经由 [Institut National de Recherche en Informatique et Automatique（INRIA）](https://en.wikipedia.org/wiki/French_Institute_for_Research_in_Computer_Science_and_Automation)进行了数十年的开发和优化，并且它是用于如此严重的类型检查语言的最快编译器之一。

## ReasonML

所以，***如果 Facebook 打算让静态类型的 React 生态系统被打入，Bucklescript 肯定是一个很好的候选***，因为他们似乎相信 JavaScript 以其流行的花括号语法对 React 的成功负有主要责任。

```ocaml

// ReasonML 中的阶乘实现

let rec factorial = (x) => 
    if(x <= 0) {
        1;
    } else {
        x * factorial(x - 1);
    };

```

然而，他们并不足够简单地将 Bucklescript 与其 OCaml 语法相结合。他们相当保守 OCaml 的语义; Bucklescript 后端和尽可能多的 JavaScript 语法。

***为了保持 JavaScript 语法，他们创建了一个额外的解析器***，处理一种叫做 ReasonML 的新语言，它简直就是 OCaml，带有类似 JavaScript 的花括号语法。

```ocaml

// ReasonML 中的 FizzBuzz 实现

let fizzbuzz = (i) =>
    switch ([i mod 3, i mod 5]) {
        | [0, 0] => "FizzBuzz"
        | [0, _] => "Fizz"
        | [_, 0] => "Buzz"
        | _ => string_of_int(i)
    };

    for(i in 1 to 100) {
        print_endline(fizzbuzz(i));
    };

```

***其结果与 JavaScript 非常相似***，以至于某些 JavaScript 代码可以被编译器直接处理，就好像它是 ReasonML 一样，立即享受静态类型编译器带来的好处，而无需更改任何代码。

```ocaml

// 有效的 ReasonML 和 Javascript 代码

let add = (a, b) => a + b;
add(4, 6);

```

## ReasonReact

除了语言和编译器本身的工作之外，Facebook 还致力于开发围绕其 React 框架的 ReasonML 包装以及一些附加功能。

它被称为 [Reason React](https://reasonml.github.io/reason-react/) 并且已经开发，因此可以很[容易地将 JavaScript React 组件](https://reasonml.github.io/reason-react/docs/en/reason-using-js.html)与 Reason 组件在同一个 ReactJS 或 Reason 应用程序中混合使用。

应该注意的是，[Reason React] 不仅仅是 React 的包装，还提供了外部函数库，如 [Redux](https://redux.js.org/introduction) 和 [Immutable](https://facebook.github.io/immutable-js/)。

## Redux 有什么用？

*** Redux 是一名在 React 项目中非常流行的状态管理器***。 简而言之，它允许将应用程序域逻辑组织为一组组合的 reducer 函数，这些函数旨在表示应用程序的状态应该如何转换为外部事件（如用户交互）。

***使用 ReasonML 时，我们不再需要 Redux。*** ReasonReact 无状态组件已经提供了 reducer 中构建的概念，旨在解决 Redux 用于解决的问题。

```ocaml
/*  
    * ReasonReact 中的简单递增计数器  
    * 尝试：http://bit.ly/counter-button-sample  
*/

type state = {count: int};

type action = 
    | Click;

let component = ReasonReact.reducerComponent("Counter");

module Counter = {
    let make = _children => {
        ...component,

        initialState: () => {count: 0},

        reducer: (action, state) => 
            switch (action) {
                | Click => ReasonReact.Update({count: state.count + 1})
            },
        
        render: self => {
            let message = 
                "Clicked"
                ++ string_of_int(self.state.count)
                ++ " times(s)";
                <div>
                    <button onClick=(_event => self.send(Click))>
                    (ReasonReact.string(message))
                    </button>
                </div>;
        },
    };
};


```

## 如何不可变？

以前由 Immutable 提供的功能是在语言级别实现的。 ReasonML（和 OCaml ）操作默认是不可变的，因此避免了使用外部库的认知和性能开销。

## 如何将 Reason 与 elm 进行比较？

前一段时间，我写了一系列关于 [elm 语言 的文章](https://www.imaginarycloud.com/blog/elm-javascript-reinvented-1-overview/)，他们彼此没有什么不同。

分析它们之间的深度差异超出了本文的预期范围，但总而言之，***它们源于对函数纯度的不同立场以及两个项目的不同成熟度水平***。

下面你可以找到他们的特征如何匹配的表格摘要：

共同的特征:

- 函数式编程;
- 编译成 JavaScript ;
- 安全;
- 短反馈回路;
- 易于测试和反应堆;
- 全覆盖，推断的静态类型。

差异：

特性 | Reason | Elm 
------------ | ------------- | -------------
函数纯度 | 不纯粹 | 纯粹
语法 | 基于Javascript的 - 可选 ML 为基础 | 更简洁;基于 ML 的语法
JS 交互 | 更简单 - 不太安全 | 安全 - 更多的模板
测试难易度 | 由于最终缺乏函数纯度，因此可能有些代码用 Reason 可能更难以测试 | 由于其函数纯度，始终易于测试
React 兼容 | 是 | 否
处理JS的副作用 | 通过编写命令式代码很容易处理副作用 | 有时很难优雅地处理
多态性 | 参数化和 OO 式 Ad-hoc | 参数型和行型
编译速度 | 非常快 | 较慢
目标平台 | Javscript、OCaml Bytecode; Native code(AMD; INTEl ; ARM & PowerPC) | Javascript

行业 / 学术支持 | Facebook;Blooberg; INRIA | Evan Czaplicki(作者)；Prezi NoRedInk

## 编译为本地代码

正如您在上面的表格中注意到的那样，它提到了 ReasonML 可以编译为本地代码。这可以通过将 ReasonML语法层与剩余的原始 OCaml 编译器（包括原始本机代码后端）一起使用来完成。

***这里有很多潜力***，最终允许在后端和前端之间共享原因代码，并将后端编译为本机代码。

## 现实世界的 Reason

ReasonML 的旗舰应用程序是 Facebook Messenger，最初是一个使用 ReactJS 的应用程序，已逐步迁移到 ReasonML。此外，Reason 文档页面中列出了大量[其他项目和公司](https://reasonml.github.io/en/users-of-reason.html)。

## 结论

***ReasonML似乎是对 Elm 框架所探讨的相同概念的一种新尝试***。即便如此，从营销和技术观点来看，这个项目及其支持者所采取的选择似乎更有希望。

虽然 Elm 看起来是一个建立在创新理念基础上的美丽原型，但 ***ReasonML 似乎是企业级的实现***，恰到好处地站在巨人的肩膀上，并吸引主流的品味。