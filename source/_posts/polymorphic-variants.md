---
title: Reason 中的多态性与普通变体
created: 2018/05/25
description: Reason 中的多态性与普通变体
author: lanqy
---
# Reason 中的多态性与普通变体

译自： http://blog.klipse.tech/reason/2018/03/12/blog-reason-types.html

## 介绍

变体是 Reasonml 最酷的特性之一。

在[官方文档](https://reasonml.github.io/docs/en/variant.html)中，他们提到了变体的限制（也称为普通变体）：

> 一个函数不能接受由两个不同变体共享的任意构造函数。

他们还提到，使用多态变体可以克服这个限制。

本文的目的是揭示普通变体的局限性，并看看多态性变体如何克服这一限制。我们希望我们带着狗和郁金香带来的例子会让这篇文章的阅读有些愉快。

![狗和郁金香](/images/dog_tulip.jpg)

## 普通变体 - 简要回顾

假设你有一个 `animal` 变种

```ocaml
# type animal =
| Dog
| Cat

type animal = Dog | Cat
```

并且你想写一个函数来将 `animal` 字符串化。

```ocaml
# let string_of_animal = x =>
switch (x) {
  | Dog => "dog"
  | Cat => "cat"
};

val string_of_animal : animal => string = <fun>
```

现在，一只 `Dog` 是一只 “狗” ，一只 `Cat` 是一只“猫”：

```ocaml
# "The " ++ string_of_animal(Dog) ++ " bites the " ++ string_of_animal(Cat);
- : string = "The dog bites the cat"
```

到现在为止还挺好。

现在让我们对鲜花做同样的事情：

```ocaml
# type flower =
| Rose
| Tulip;

type flower = Rose | Tulip

# let string_of_flower = x =>
switch (x) {
  |Rose => "rose"
  |Tulip => "tulip"
};

val string_of_flower : flower => string = <fun>

# let a = "The " ++ string_of_flower(Rose) ++ " is more beautiful than the " ++ string_of_flower(Tulip);

val a : string = "The rose is more beautiful than the tulip"
```

## 变体的局限性

现在如果你尝试写一个函数使花和动物字符串化会发生什么？

```ocaml
# let string_of_flower_or_animal = x =>
switch (x) {
  |Rose => "rose"
  |Tulip => "tulip"
  |Dog => "dog"
  |Cat => "cat"
};

File "", line 5, characters 4-7:
Error: This variant pattern is expected to have type flower
The constructor Dog does not belong to type flower
/* 文件“”，第5行，字符4-7：
错误：这种变体模式预计会有花类型
构造函数Dog不属于花的类型 */
```

构造函数 `Dog` 不属于 `flower` 类型，在这种情况下，ocaml 不会立即创建 `flower_or_animal` 类型！

普通变体的另一个限制是，你不能在列表或数组中混合 `animal` 和 `flower` 类型元素：

```ocaml
# let a = [Dog, Cat, Rose, Tulip];

File "", line 1, characters 19-23:
Error: This variant expression is expected to have type animal
The constructor Rose does not belong to type animal

/* 文件“”，第1行，字符19-23：
错误：预计此变体表达式具有 Dog 类型
构造函数 Rose 不属于 Dog 类型 */
```

欢迎来到多态变体的世界！

## 多态变体

在语法上，多态变体通过反向撇号区别于普通变体：

```ocaml
# let myDog = `Dog;
val myDog : [> `Dog] = `Dog
```

请注意，与普通变体不同，多态变体可以在没有显式类型声明的情况下使用。他们的类型是自动推断的。

当然，它也适用于参数化的变体：

```ocaml
# let myNumber = `Int(4);
val myNumber : [> `Int of int ] = `Int 4
```

现在，让我们看看如何使用多态类型来编写我们的 `string_of_flower_or_animal` 函数：

```ocaml
# let string_of_flower_or_animal = x =>
switch (x) {
  |`Rose => "rose"
  |`Tulip => "tulip"
  |`Dog => "dog"
  |`Cat => "cat"
};

val string_of_flower_or_animal : [< `Cat | `Dog | `Rose | `Tulip ] => string = <fun>
```

请注意，系统已经自动推断函数参数的类型：它是[< \`Cat | \`Dog| \`Rose | \`Tulip ]。你可能想知道 `<` 符号 的含义是什么。

在回答这个问题之前，让我们看看多态变体如何让我们在列表中混合不同类型的元素：

```ocaml
# let myNatrue = [`Dog, `Cat, `Rose, `Tulip];
val myNatrue : [> `Cat | `Dog | `Rose | `Tulip] list = [`Dog; `Cat; `Rose; `Tulip]
```
现在，列表的类型是：`[> \`Cat | \`Dog | \`Rose | \`Tulip] list 。

## 上限和下限

现在是时候解释多态变体中 `<` 和 `>` 的含义了。

`>` 在变体类型的开头标记类型 a 是**开放的**以便与其他变体类型组合。我们可以解读类型  [> \`Cat | \`Dog | \`Rose | \`Tulip ] 为描述一种变体，其标签包括 \`Cat，\`Dog，\`Rose和 \`Tulip，但也可能包含更多标签。

换句话说，你可以大致翻译 `>` 来表示：“这些标签或更多”。

事实上，我们可以连接动物列表和花名单：

```ocaml
let myAnimals = [`Dog, `Cat];
let myFlowers = [`Rose, `Tulip];
let myThings = List.concat([myAnimals, myFlowers]);

val myAnimals : [> `Cat | `Dog] list = [`Dog; `Cat]
val myFlowers : [> `Rose | `Tulip] list = [`Rose; `Tulip]
val myThings : [> `Cat | `Dog | `Rose | `Tulip] list = [`Dog; `Cat; `Rose; `Tulip]
```

`<` 变体类型的开始部分表示 “这些标签或更少”。例如，在我们上面定义的 `string_of_flower_or_animal` 函数中，参数被推断为类型  [< \`Cat | \`Dog | \`Rose| \`Tulip]。

事实上，这个函数没有办法处理具有除 \`Cat，\`Dog，\`Rose 和 \`Tulip 之外的标签的值。

## 结论

你现在可能会问自己为什么不总是使用多态变体。

答案是多态变体的灵活性是有代价的。

- 它们比普通变体更复杂
- 他们不太可能捕捉那些普通变体的错误 - 正是由于它们允许的灵活性
- 它们比普通变体重一点，性能较差

请务必阅读 Real World Ocaml 的 [这一章](https://realworldocaml.org/v1/en/html/variants.html#polymorphic-variants)，深入了解普通和多态变体。在本章最后，他们详细解释了多态变体相对于普通变体的优点和缺点。
