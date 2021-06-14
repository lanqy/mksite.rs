---
title: ES6：var，let和const - 函数作用域和块范围之间的争斗
created: 2018/11/21
description: ES6：var，let和const - 函数作用域和块范围之间的争斗TypeScript
author: lanqy
---
# ES6：var，let和const - 函数作用域和块范围之间的争斗

译自：http://www.deadcoderising.com/2017-04-11-es6-var-let-and-const-the-battle-between-function-scope-and-block-scope/

在 ES6 之前的时代，只有一种方法可以在 JavaScript 中声明变量 - 即使用 var。

var 总是有这种特殊的误解光环 - 这可能是因为用 var 声明的变量的行为与大多数其他编程语言的区别。话虽如此，整个事情有一个非常自然的解释 - 作用域。

问题是，var 是函数作用域。这种类型的作用域与更常用的块范围略有不同。

让我们来看看这意味着什么。

##  var - 函数作用域

如前所述，使用 var 声明的变量将是函数作用域，这意味着它将存在于其声明的函数作用域内。

```javascript
function myFunc() {
  var name = 'Luke'
  console.log(name); // 'Luke'
}

myFunc();

console.log(name); // name is not defined
```

如您所见，函数内部使用 var 声明的变量无法从函数外部访问。

话虽如此，其他类型的块 - 如 if 语句，循环等 - 将不被视为作用域。

```javascript
if(true) {
  var name = 'Luke'
}

console.log(name); // 'Luke'
```

使用 var，变量名在其声明的 if 语句之外可用。这是因为它们属于同一作用域。

然而，随着 ES6 的引入，引入了两种声明变量的新方法。

## let 和 const - 块作用域的介绍

在ES6中，let 和 const 被引入作为声明变量的替代方法 - 两者都块作用域。

如果你习惯了除 JavaScript 之外的任何其他语言，这可能会更好地与你产生共鸣。

在块作用域内，任何块都是范围。这将提供更一致的行为。

这意味着函数仍然是有效作用域，就像使用 var 一样。

```javascript
function myFunc() {
  let name = 'Luke'
  console.log(name); // 'Luke'
}

myFunc();
console.log(name); // name is not defined
```

但在这种情况下，其他类型的块也可以作为作用域 - 如 if 语句。

```javascript
if(true) {
  let name = 'Luke'
}

console.log(name); // name is not defined
```

## 当函数作用域变得混乱时

现在我们知道了功能作用域和块范围之间的区别 - 让我们看看为什么这很快就会让人感到困惑。

在作用域内部具有与外部作用域中的变量同名的局部变量是完全正确的。

```javascript
var name = 'Luke';

const func = () => {
  var name = 'Phil';
  console.log(name); // 'Phil'
}

func();

console.log(name); // 'Luke'
```

正如预期的那样，即使在 func (包含一个同样命名的局部变量)执行之后，外部作用域中的 name 仍然保持初始声明值“Luke”。

然而问题是，由于函数作用域只包含函数而不是其他类型的块，我们会得到与其他块完全不同的行为。

```javascript
var name = 'Luke';

if(true) {
  var name = 'Phil';
  console.log(name); // ‘'Phil'
}

console.log(name); // 'Phil'
```

在这种情况下，'Phil' 将在两个地方打印。这是因为两个变量都在相同的作用域内，导致 'Phil' 覆盖第一个变量声明。


可以想象，随着复杂性的增加，这很快就会变成一个真正的头痛。

## 与块作用域保持一致

如果我们看看 let  - 这是块作用域 - 这将对所有块保持一致。

```javascript
let name = 'Luke';

const func = () => {
  let name = 'Phil'
  console.log(name); // 'Phil'
}

func();

console.log(name); // 'Luke'
```

```javascript
let name = 'Luke';

if (true) {
  let name = 'Phil';
  console.log(name); // 'Phil'
}

console.log(name); // 'Luke'
```

##  循环怎么样？

让我们看看另一个例子来真正理解不同的行为。


假设我们想要创建一个将惰性函数推送到数组的循环。这些函数中的每一个都将打印当前索引。

让我们首先看看如果我们使用 var 会发生什么。

```javascript
var printsToBeExecuted = [];

for (var i = 0; i < 3; i++) {
  printsToBeExecuted.push(() => console.log(i));
}

printsToBeExecuted.forEach(f => f());
// Output: 3, 3, 3
```

再说一次，如果你习惯于块作用域，那会觉得有点奇怪。你会期望 0, 1, 2 对吗？

解释只是在使用 var 时循环不是作用域。因此，不是为每个增量创建局部变量 i ，而是最终为所有函数打印变量的最终值。

```javascript
var printsToBeExecuted = [];

for (var i = 0; i < 3; i++) {
  printsToBeExecuted.push(
    ((ii) => () => console.log(ii))()
  )
}

printsToBeExecuted.forEach(f => f());

// Output: 0, 1, 2
```

太棒了，我们得到了我们预期的输出，但它有点冗长吧？

如果我们现在看一下使用块作用域的解决方案来获取迭代变量，我们将获得第一个示例的简单性以及预期的结果。

```javascript
var printsToBeExecuted = [];

for (let i = 0; i < 3; i++) {
  printsToBeExecuted.push(() => consolt.log(i));
}

printsTBeExecuted.forEach(f => f());
// Output: 0, 1, 2
```
