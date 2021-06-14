---
title: 理解 JavaScript 调用堆栈
created: 2018/10/22
description: 理解 JavaScript 调用堆栈
author: lanqy
---
# 理解 JavaScript 调用堆栈

译自：https://medium.freecodecamp.org/understanding-the-javascript-call-stack-861e41ae61d4

JavaScript 引擎（在浏览器等托管环境中找到）是一个包含堆和单个调用堆栈的单线程解释器。浏览器提供 DOM，AJAX 和 Timers 等 Web API。

本文旨在解释调用堆栈是什么以及为什么需要它。对调用堆栈的理解将清楚地说明“函数层次结构和执行顺序”在 JavaScript 引擎中的工作原理。

调用堆栈主要用于函数调用（调用）。由于调用堆栈是单个的，所以函数执行一次一个地完成，从上到下。这意味着调用堆栈是同步的。

对调用堆栈的理解对异步编程至关重要（我们将在后面的文章中介绍）。

在异步 JavaScript 中，我们有一个回调函数，一个事件循环和一个任务队列。在通过事件循环将回调函数推送到堆栈之后，调用堆栈在执行期间对回调函数起作用。

在最基本的层面上，调用堆栈是一种数据结构，它使用后进先出（LIFO）原则来临时存储和管理函数调用（调用）。

让我们分解我们的定义：

LIFO：当我们说调用堆栈由 Last In，First Out 的数据结构原理操作时，这意味着当函数返回时，最后一个被推入堆栈的函数是第一个被弹出的函数。

让我们看一下代码示例，通过向控制台打印堆栈跟踪错误来演示 LIFO。

```javascript
function firstFunction() {
    throw new Error('Stack Trace Error');
}

function secondFunction(){
    firstFunction();
}

function thirdFunction() {
    secondFunction();
}

thirdFunction();
```

代码运行时，我们收到错误。打印堆栈显示函数如何堆叠在彼此之上。看一下图表。

![console](/images/1_LIuELJ2RTtwWExRWGdu_Hw.png)

你会注意到函数作为一个堆栈的排列开始于 `firstFunction()`（这是进入堆栈的最后一个函数，并弹出以抛出错误），然后是`secondFunction()` ，然后是 `thirdFunction()`（这是执行代码时第一个被推入堆栈的函数）。

暂时存储：当调用（called）函数时，函数，其参数和变量被推入调用堆栈以形成堆栈帧。此堆栈帧是堆栈中的内存位置。当函数从堆栈中弹出时，内存被清除。
![stack](/images/1_PPkrowy4n_Pyehb_NdhLrg.png)

管理函数调用（called）：调用堆栈维护每个堆栈帧的位置记录。它知道要执行的下一个函数（并将在执行后将其删除）。这就是使 JavaScript 中的代码执行同步的原因。

想象一下你自己站在一个杂货店的现金点排队。只有在你面前的人被照顾后才能照顾你。这是同步的。

这就是我们所说的“管理函数调用”。

## 调用堆栈如何处理函数调用？

我们将通过查看调用另一个函数的函数的示例代码来回答这个问题。这是示例代码：

```javascript
function firstFunction(){
 console.log("Hello from firstFunction");
}
function secondFunction(){
 firstFunction();
 console.log("The end from secondFunction");
}
secondFunction();
```

![console output](/images/1_9iSkoJoXM0Ok8iQ5mOHl5Q.png)

这是代码运行时发生的情况：

1.当执行 secondFunction() 时，会创建一个空堆栈帧。它是该程序的主要（匿名）入口点。
2. secondFunction() 然后调用 firstFunction()，将其推入堆栈。
3. firstFunction() 返回并将“Hello from firstFunction”打印到控制台。
4. firstFunction() 从堆栈中弹出。
5. 执行顺序然后移动到 secondFunction()。
6. secondFunction() 返回并打印“The end from secondFunction”到控制台。
7. 从堆栈中弹出 secondFunction()，清除内存。

## 是什么导致堆栈溢出？

当存在递归函数（一个自己调用的函数）而没有退出点时，会发生堆栈溢出。浏览器（托管环境）具有最大堆栈调用，它可以在抛出堆栈错误之前容纳。

这是一个例子：

```javascript
function callMyself(){
  callMyself();
}
callMyself();
```

callMyself() 将一直运行，直到浏览器抛出“Maximum call size exceeded”。那是堆栈溢出。

![Maximum call stack error](/images/1_JFRlgLp2uvbdVrh7WdmMrQ.png)

## 综上所述

调用堆栈的关键点是：

- 它是单线程的。这意味着它一次只能做一件事。
- 代码执行是同步的。
- 函数调用会创建占用临时内存的堆栈帧。
- 它作为LIFO - Last In，First Out 数据结构。

我们使用了调用堆栈文章为我们将在异步 JavaScript 上看到的系列奠定了基础（我们将在另一篇文章中看到）。

所有代码示例都可以在此 [github仓库](https://github.com/charlesfreeborn/JS-CallStack-CodeSamples/blob/master/codesamples.md) 中找到。 