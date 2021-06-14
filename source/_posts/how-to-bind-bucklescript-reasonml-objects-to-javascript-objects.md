---
title: 如何将 BuckleScript / ReasonML 对象绑定到 JavaScript 对象
description: 如何将 BuckleScript / ReasonML 对象绑定到 JavaScript 对象
created: 2018/06/15
author: lanqy
---

# 如何将 BuckleScript / ReasonML 对象绑定到 JavaScript 对象。

> 注1：当我在本文中说 BuckleScript 时，我指的是 OCaml。
> 注2：JavaScript对象与 BuckleScript / ReasonML中的 Object 不同。

尽管它们本身支持面向对象的风格，但 BuckleScript / ReasonML 是函数式语言，这意味着它们不鼓励使用类的概念。相反，要在 BuckleScript / ReasonML 中创建 JavaScript 对象，可以使用 Js.Dict 或 Record 类型。当需要可变数量的 keys 时，应使用前者。后者用于键被固定并且其类型被预先确定的情况。

## 使用 Js.Dict 创建一个对象


```ocaml
let myDict = Js.Dict.empty ()
let _ Js.Dict.set myDict "myKey1" 100
let _ Js.Dict.set myDict "myKey2" 200

let myDict2 = Js.Dict.empty ()
let _ Js.Dict.set myDict2 "myKey1" 100
let _ Js.Dict.set myDict2 "myKey2" "hello" (* 错误，值必须是相同的类型 *)
```
\*用 BuckleScript Js.Dict 创建 JavaScript 对象\*

```ocaml
let myDict = Js.Dict.empty();
Js.Dict.set(myDict, "myKey1", 100);
Js.Dict.set(myDict, "myKey2", 200);

let myDict2 = Js.Dict.empty();
Js.Dict.set(myDict2, "myKey1", 100);
Js.Dict.set(myDict2, "myKey2", "hello"); /* 错误，值必须是相同的类型 */

```

\* 上面代码片段的 ReasonML 对应部分。\*

Js.Dict 是一个 JavaScript 字典，我们可以将任何值作为值，但是，强类型系统要求其值必须是相同的类型。我们创建的字典将直接转换为普通的 JavaScript 对象。

如果我们想要在 Js.Dict 中存储多个类型的值，我们可以使用嵌套的 Js.Dict 结构或变体类型来实现。这看起来可能很麻烦，但这是对型号安全的一种折衷。

现在，让我们看看如何在 Js.Dict 中存储变量类型的值。

```ocaml
type myVariantType = 
    | Nothing
    | Something of int
    | LotOfThing of int array
    [@@bs.deriving accessors]

let myDict = Js.Dict.empty ()
let _ = Js.Dict.set myDict "foo" Nothing
let _ = Js.Dict.set myDict "bar" ((Something (1)))
let _ = Js.Dict.set myDict "foobar" ((LotOfThing ([|1;2;3|])))
```
\* 代码片段显示了如何在Dictionary对象中存储变体类型 \*

```ocaml
[@bs.deriving accessors]
type myVariantType = 
    | Nothing
    | Something(int)
    | LotOfThing(array(int));

let myDict = Js.Dict.empty();
Js.Dict.set(myDict, "foo", Nothing);
Js.Dict.set(myDict, "bar", Something(1));
Js.Dict.set(myDict, "foobar", LotOfThing([|1,2,3|]));
```
\* 上述代码片段的ReasonML对象 \*

Variant 类型的值不会按原样存储在 JavaScript 对象中，但它们会被转换并转换为普通的 JavaScript 对象，如下所示：

```ocaml
var myDict = { };

myDict["foo"] = /* Nothing */0;
myDict["bar"] = /* Something */Block.__(0, [1]);
myDict["foobar"] = /* LotOfThing */Block.__(1, [/* array */[
      1,
      2,
      3
    ]]);
```

`Nothing` 变成 `0`, `Something` 变成 `Block.__(0, [1]);` 和 `LotOfThing` 变成 `Blocks.__(1, [[1,2,3]]);`

这意味着我们在运行时不会有 Variant 类型的值。通过不查看注释来运行代码，我们无法取回 Variant 的名称。这就是注解 `accessors` 在那里的原因。它将根据每种变体类型绑定变量，以便我们可以自然地在 JavaScript 中使用 Variant，例如 `myDict["foo"] = nothing;` 代替 `myDict["foo"] = /* Nothing */0;`

## 使用 Record 创建一个对象

虽然我们使用 Js.Dict 来存储相同类型的键和值（称为字段），并且它可能具有可变数量的字段。[一个对象可以被定义为一个记录](https://bucklescript.github.io/docs/en/object.html#object-as-record)

- 具有已知数量的字段
- 可能包含或可能不包含异构类型的值

与 Variant 略有不同，可以使用 type 关键字使用[以下语法](https://realworldocaml.org/v1/en/html/records.html)定义 Record：

```ocaml
type <记录名称> =
  { <字段> : <类型> ;
    <字段> : <类型> ;
    ...
  }
```

例如，

```ocaml
type person = {
    name: string;
    age: int;
    job: string;
}

let p = {name = "John"; age = 20; job = "jobless"}
let _ = Js.log p
```

上面的代码产生了一个 ["John", 20, "jobless"] 的数组，这并不是我们想要的。要保留键，我们必须使用对象语法将我们的 Record 包装在 JavaScript 对象类型 Js.t 中。

```ocaml
type person = <name: string; age: int; job: string> Js.t
```

这里的角括号是在 BuckleScript 中创建对象。请注意，这与典型的基于类的面向对象语言不同，它不需要 Class 的概念。对于来自其他世界的人来说，这可能看起来很奇怪，但是[在 OCaml 类型中并不等于类](https://realworldocaml.org/v1/en/html/objects.html#ocaml-objects)。一个对象可以在没有类的情况下创建。

为了创建，我们可以使用 person 类型的对象

```ocaml
let p = object
    method name = "John"
    method age = 20
    method job = "jobless"
end;;
```

不，这不是一个错字。我们想要暴露给外部世界的属性被定义为方法。这是未包装在 Js.t 中的对象的原始 OCaml 语法。但是，BuckleScript 将所有 JavaScript 对象解除为 Js.t. 它通过提供 bs.obj s注释帮助我们避免语法负担。所以，上面的代码将成为

```ocaml
let p = [%bs.obj {name="John";age=20;job="jobless"}]
```

ReasonML 将语法糖带到另一个层面。要定义一个 JavaScript 绑定：

```ocaml
type person = {
    .
    "name": string,
    "age": int,
    "job": string,
};
```

并在 ReasonML 中创建 JavaScript 对象：

```ocaml
let p = {"name": "John", "age": 20, "job": "jobless"};
```

## 类

在 ES6 中引入的 JavaScript 类仅仅是使用基于原型的继承和函数闭包的功能。

> [类语法不会为 JavaScript 引入新的面向对象的继承模型。](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes)

他们[不鼓励使用](https://bucklescript.github.io/docs/en/class.html#bind-to-js-classes)：

> 通常，更喜欢使用先前的对象部分的特征来绑定到 JS 对象。

最后，本文旨在讨论如何将 BuckleScript / ReasonML 对象绑定到 JavaScript 对象，但到目前为止，我们所做的仅仅是定义使 OCaml 类型系统识别我们要使用的对象的结构。

`external` 是我们想要将值绑定到 JavaScript 值时使用的关键字。例如

```ocaml
external johb : person = "john" [@@bs.val]
```

这意味着我们将 john 绑定到 JavaScript 变量名称 john 并且它有一个 person 类型。

## 总结

使用 JavaScript 对象时，可能会试图将其与（Js.t）框一起使用。但是，为了获得 OCaml 类型系统的全部好处，我们宁愿将 Js.t 转换为本地结构。
