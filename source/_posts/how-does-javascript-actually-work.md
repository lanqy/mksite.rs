---
title: JavaScript 的工作原理：引擎，运行时和调用堆栈的概述
description: JavaScript 的工作原理：引擎，运行时和调用堆栈的概述
created: 2018/10/22
author: lanqy
---

# JavaScript 的工作原理：引擎，运行时和调用堆栈的概述

随着 JavaScript 变得越来越流行，团队正在利用其在堆栈中的多个级别的支持 - 前端，后端，混合应用程序，嵌入式设备等等。

这篇文章旨在成为系列中的第一篇，旨在深入挖掘 JavaScript 及其实际工作方式：我们认为通过了解 JavaScript 的构建块以及它们如何一起发挥，您将能够编写更好的代码和应用。我们还将分享我们在构建 [SessionStack](https://www.sessionstack.com/?utm_source=medium&utm_medium=source&utm_content=javascript-series-post1-intro) 时使用的一些经验法则，这是一个轻量级JavaScript应用程序，必须具有强大且高性能才能保持竞争力。

如 [GitHut](https://githut.info/) 统计数据所示，JavaScript 在 GitHub 中的 Active Repositories和 Total Pushes 方面处于领先地位。它也不会落后于其他类别

![console](/images/1_Zf4reZZJ9DCKsXf5CSXghg.png)
(Check out up-to-date GitHub language stats).

如果项目越来越依赖于JavaScript，这意味着开发人员必须利用语言和生态系统提供的所有内容，对内部进行更深入和更深入的了解，以便构建出色的软件。

事实证明，有很多开发人员每天都在使用JavaScript，但却不了解幕后发生的事情。

## 概貌

几乎每个人都已经听说过 V8 引擎作为一个概念，大多数人都知道 JavaScript 是单线程的，或者它使用的是回调队列。

在这篇文章中，我们将详细介绍所有这些概念，并解释 JavaScript 实际运行的方式。通过了解这些详细信息，您将能够编写更好的，非阻塞的应用程序，这些应用程序正确地利用了所提供的 API。

如果您对 JavaScript 比较陌生，那么这篇博文将帮助您理解为什么 JavaScript 与其他语言相比如此“奇怪”。

如果您是一位经验丰富的 JavaScript 开发人员，希望它会为您提供一些关于您每天使用的 JavaScript 运行时实际工作方式的新见解。

## JavaScript 引擎

JavaScript 引擎的一个流行示例是 Google 的 V8 引擎。例如，V8 引擎用于 Chrome 和 Node.js。这是一个非常简化的视图：

![](/images/1_OnH_DlbNAPvB9KLxUCyMsA.png)

引擎包含两个主要组件：

- 内存堆 - 这是内存分配发生的地方
- 调用堆栈 - 这是您的代码执行时堆栈帧的位置

## 运行时

浏览器中有几乎所有 JavaScript 开发人员都使用过的 API（例如“setTimeout”）。但是，引擎不提供这些 API。

那么，他们来自哪里？

事实证明，现实有点复杂。

所以，我们有引擎，但实际上还有很多。我们有一些叫做 Web API 的东西，它们是由浏览器提供的，比如 DOM，AJAX，setTimeout 等等。

然后，我们有如此受欢迎的事件循环和回调队列。

## 调用堆栈

JavaScript 是一种单线程编程语言，这意味着它只有一个调用堆栈。因此，它可以一次做一件事。

调用栈是一种数据结构，它基本上记录了程序中的位置。如果我们进入函数，我们将它放在堆栈的顶部。如果我们从函数返回，我们会弹出堆栈的顶部。这就是所有堆栈都可以做到的。

我们来看一个例子吧。看一下下面的代码：

```javascript
function multiply(x, y) {
    return x * y
}

function printSquare(x) {
    var s = multipy(x, x);
    console.log(s);
}

printSquare(5);
```

当引擎开始执行此代码时，调用堆栈将为空。之后，步骤如下：

![](/images/1_Yp1KOt_UJ47HChmS9y7KXw.png)

调用堆栈中的每个条目称为堆栈帧。

这正是抛出异常时堆栈跟踪的构造方式 - 它基本上是异常发生时调用堆栈的状态。看一下下面的代码：

```javascript
function foo() {
    throw new Error('SessionStact will help you resolve crashes :)')
}

function bar() {
    foo();
}

function start() {
    bar();
}

start();
```

如果在 Chrome 中执行此操作（假设此代码位于名为 foo.js 的文件中），则将生成以下堆栈跟踪：

![](/images/1_T-W_ihvl-9rG4dn18kP3Qw.png)

“吹掉堆栈” - 当达到最大调用堆栈大小时会发生这种情况。这很容易发生，特别是如果你使用递归而不是非常广泛地测试你的代码。看看这个示例代码：

```javascript
function foo() {
    foo();
}

foo();
```

当引擎开始执行此代码时，它首先调用函数“foo”。但是，此函数是递归的，并且在没有任何终止条件的情况下开始调用自身。因此，在执行的每个步骤中，相同的函数一次又一次地添加到调用堆栈中。它看起来像这样：

![](/images/1_AycFMDy9tlDmNoc5LXd9-g.png)

但是，在某些时候，调用堆栈中的函数调用数量超过了调用堆栈的实际大小，并且浏览器决定通过抛出错误来执行操作，该错误看起来像这样：

![](/images/1_e0nEd59RPKz9coyY8FX-uw.png)

在单个线程上运行代码非常简单，因为您不必处理多线程环境中出现的复杂场景 - 例如，死锁。

但是在单个线程上运行也是非常有限的。由于JavaScript只有一个调用堆栈 (Call Stack)，当事情变慢时会发生什么？

## 并发和事件循环

如果在调用堆栈中有函数调用需要花费大量时间才能处理，会发生什么？例如，假设您想在浏览器中使用 JavaScript 进行一些复杂的图像转换。

你可能会问，为什么这甚至是一个问题?问题是，虽然调用堆栈有函数要执行，但浏览器实际上不能做任何其他事情——它被阻塞了。这意味着浏览器不能呈现，不能运行任何其他代码，它只是卡住了。如果你想在应用中使用流畅的 UI，这就会产生问题。

这不是唯一的问题。一旦您的浏览器开始在调用堆栈中处理如此多的任务，它可能会在相当长的时间内停止响应。大多数浏览器通过引发错误来采取行动，询问您是否要终止网页。

![](/images/1_WlMXK3rs_scqKTRV41au7g.jpeg)

现在，那不是最好的用户体验，是吗？

那么，如何在不阻止 UI 并使浏览器无响应的情况下执行繁重的代码呢？好吧，解决方案是异步回调。

这将在“如何实际运行JavaScript”教程的第2部分中进行更详细的解释：“[在V8引擎内部+有关如何编写优化代码的5个技巧](https://blog.sessionstack.com/how-javascript-works-inside-the-v8-engine-5-tips-on-how-to-write-optimized-code-ac089e62b12e)”。与此同时，如果您在JavaScript应用程序中难以复制和理解问题，请查看 [SessionStack](https://www.sessionstack.com/?utm_source=medium&utm_medium=blog&utm_content=Post-1-overview-outro) 。SessionStack 记录 Web 应用程序中的所有内容：所有 DOM 更改，用户交互，JavaScript 异常，堆栈跟踪，网络请求失败和调试消息。

使用 SessionStack，您可以将网络应用中的问题作为视频重播，并查看用户发生的所有事情。

有一个免费的计划，不需要信用卡。现在就[开始](https://www.sessionstack.com/?utm_source=medium&utm_medium=blog&utm_content=Post-1-overview-getStarted)。

![](/images/1_kEQmoMuNBDfZKNSBh0tvRA.png)
