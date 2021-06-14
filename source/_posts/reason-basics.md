---
title: Reason 基础知识
created: 2018/05/23
description: Reason 基础知识
author: lanqy
---
# Reason 基础知识

译自： https://github.com/parkerziegler/reason-basics

> 人们学习 Reason 编程语言的存储库。

## 目的

此存储库是 Reason 社区的新手通过简单示例了解该语言的基础知识的地方。你在这里看到的大部分内容都是从[文档](https://reasonml.github.io/docs/en/overview.html)中压缩或修改的。文档是一个很好的资源，并为语言背后的设计决策提供了高层次的解释。但是，对于初次使用程序员和首次使用 Reason 的用户来说，他们会感到有点压倒性。这个存储库希望能够简化一些语言，并提供非常清晰，简洁的例子来帮助你了解和使用 Reason。如果您发现任何令人困惑，过于复杂或错误的事情，请[提交问题](https://github.com/parkerziegler/reason-basics/issues)或[创建拉取请求](https://github.com/parkerziegler/reason-basics/pulls)。

## 安装

遵循相同的标准步骤克隆和本地安装此存储库：

```ocaml
git clone https://github.com/parkerziegler/reason-basics.git
cd reason-basics
yarn install
```

## 运行代码

该存储库的源文件当前位于 src/index.re 中。要将 Reason 代码编译为 BuckleScript，然后编译为 JS：

```ocaml
yarn run build
```

编译该文件并在保存时监视更改：

```ocaml
yarn run start
```

运行代码并查看终端中的输出：

```ocaml
node src/index.bs.js
```

这期望您的计算机上安装了Node @ 6或更高版本。

## 编辑器

如果使用 vscode，则可以使用 `Cmd + Shift + B`（在 macOS 上）或 `Windows + Shift + B`（在 Windows 上）自动构建。

## 扩展

如果您使用 vscode，我建议安装 [OCaml 和 Reason IDE](https://marketplace.visualstudio.com/items?itemName=freebroccolo.reasonml) 以获得令人敬畏的 Reason 编码体验。


## 让我们学习 Reason

> 注意：这些内容与index.re中的内容相同，只是为了您的欣赏而写在 Markdown 中。

请注意，此存储库使用 Reason 的 v3 语法。

```ocaml
/* 这里是 Reason 中的注释. */
```

要在控制台打印，我们可以使用 BuckleScript 的 JS 模块。

```ocaml
Js.log("Hello, BuckleScript and Reason! Luv BuckleScript");
```

我们也可以使用 Reason 的原生 print_* 模块。

```ocaml
print_endline("Hello, BuckleScript and Reason! Luv Reason");
```

请注意，print_endline 只能打印字符串。要打印数据结构，请使用 Js.log 。未来在语言上对此的支持即将到来。

要打印其他类型，您可以使用 Reason 的类型强制方法或 print_* 模块。

```ocaml
print_endline(string_of_int(1));
print_endline(string_of_bool(true));
```

如果使用 print_* 方法，则需要使用 print_newline() 来获取新行。 print_endline() 为你添加新行。

```ocaml
print_int(1);
print_newline();
print_float(1.0);
print_newline();
```
### 变量

Reason 中的变量使用 let 定义。 没有 JS 中 const 的概念。

```ocaml
let myVar = "myVar";
let myInt = 1;
```

### 字符和字符串

Reason 区分字符( 用单引号 )和字符串( 用双引号 )。


```ocaml
let x = 'x';
let y = 'y';
```

模式匹配字符的示例。

```ocaml
let isXY = char: bool =>
  switch (char) {
    | 'x'
    | 'y'=> true
    | _ => false
  };
```

要将字符转换为字符串，请使用 String.make，将 1 作为第一个参数传递。

```ocaml
let stringFromChar: string = String.make(1, x) ++ String.make(1, y);
print_endline(stringFromChar);
```

字符串使用 ++ 运算符连接。

```ocaml
let greeting = "Hello";
let space = " ";
let name = "P-Doo";
let exclamation = "!";

print_endline(greeting ++ space ++ name ++ exclamation);
```

我们可以使用标准库中的 String 方法（ Reason 的内置方法）对字符串进行操作。

```ocaml
let whitespaceString = "  _trim me_  ";
let trimmedString = String.trim(whitespaceString);

print_endline(trimedString);

let atString =
  String.map(
    c =>
      switch (c) {
        |' ' => '@'
        | _ => c
      },
      whitespaceString
    );
print_endline(atString);
```

字符串中的特殊字符需要使用 `\` 转义。

```ocaml
let slash = "\\";
print_endline(slash);
```

原因还支持多行字符串，类似于 JS 模板文字。他们使用 `{|` 和 `|}` 分隔。

```ocaml
let multilineString = {|Hello
Reasonable
Folks!|};

print_endline(multilineString);
```

字符串方法在多行字符串上工作相同。例如，要将多行字符串转换为单行字符串：

```ocaml
let singlelineString =
  String.map(
    c =>
      swith (c) {
        | '\n' => ' '
        | _ => c
      },
      multilineString
    );

print_endline(singlelineString);
```

要在多行字符串中插入变量，请用 `j| 和 `|j` 包围字符串。

```ocaml
let style = "background-color: papayawhip";
let cssStyle = {j|$style|j};

print_endline(cssStyle);
```

要在 Reason 中的字符串中使用 unicode 字符，请使用 `js| |js` （译者注：或者 `j| |j`）。

```ocaml
let unicodeString = {js|••∆∆••|js};

print_endline(unicodeString);
```

获取字符串长度和截取字符串：

```ocaml
let background = "background-color: aquamarine";
let strLength: int = String.length(background);

print_endline(string_of_int(strLength));

let subStr: string =
  String.sub(background, 0, String.index(background, '-'));

print_endline(subStr);
```

### 条件语句

条件语句的工作原理与 JS 类似。然而，如果没有 `else` 的 `if` 语句，则将其计算为 `else{()}`，即 `unit` 类型。

```ocaml
let displayGreeting = true;

if (displayGreeting) {
  let message = "Enjoying my ridiculous commentary yet?";
  print_endline(message);
}
```

条件是对其正文内容进行评估的——不需要返回语句。

```ocaml
let good = true;
let content =
  if (good){"This tweet is good."} else {
    "This tweet is bad. please make it better."
  };

print_endline(content);
```

在上面的块中，必须有一个 `else` 分支对字符串进行评估。如果没有，内容将被分配到 `unit` 类型，尽管它的目的是作为一个字符串。这会产生编译器错误。还有对三元运算符的支持。

```ocaml
let retweet = good ? "Most certainly." : "Eh, don't think so.";
print_endline(retweet);
```

### 作用域

变量默认为块作用域。让绑定可以使用 `{}` 创建匿名作用域。

```ocaml
let anonymousScope = {
  let name = "Parker";
  let company = "Formidable";
  let place = "Seattle";
  print_endline({j|$name works at $company in $place.|j});
}
```

`nam` ， `company` 和 `place` 在这里是无法访问的，因为它们是由上面创建的匿名范围持有的。试图在范围之外访问它们会导致错误！

### 类型！

Reason 由 OCaml 顶尖的系统支持。我们可以明确地输入变量，虽然这不是必需的。Reason 往往会为我们推断类型。

```ocaml
let petalLength: int = 5;
```

我们也可以使用别名类型。

```ocaml
type sepalLength = int;
```

然后使用它们！

```ocaml
let sepalLength: sepalLength = 20;
```

我们通过注解参数来输入函数返回值。

```ocaml
let flowerLength = (petal: int, sepal: sepalLength): int => petal + sepal;
print_endline(string_of_in(flowerLength(petalLength, sepalLength)));
```

### 布尔

Reason 中的布尔比较类似于 JS 。 `===` 表示引用相等，而 `==` 表示结构相等。小心使用 `==`，Reason 会提醒你这个！

```ocaml
let myTuple = ("Parkie", "is", 1);
let compareBool = tuple: bool => tuple === myTuple;

print_endline(string_of_bool(compareBool(myTuple)));
```
这条线会产生多态比较（可能不安全）。

```ocaml
print_endline(string_of_bool(('M', 23) == ('M', 23)));
```

该行不会产生警告。

```ocaml
print_endline(string_of_bool(('M', 23) === ('M', 23)));
```

### 整数和浮点数

Reason 有整数和浮点数的概念，而不仅仅是 JS 数字。

```ocaml
let githubStars: int = 9;
let squareInt = (num: int): int => num * num;

print_endline(string_of_int(squareInt(githubStars)));
```

要使用整数，请使用 Reason's Pervasives 模块中的方法。这些已经在范围内。

```ocaml
let start: int = 1;
+ start; /* 一元添加 */
- start; /* 一元相减 */
let remainder = 20 mod 5;

print_endline(string_of_int(remainder));
```

浮点数具有唯一的操作数语法，附加 . 到 `+, -, *, /` 操作符上。

```ocaml
let pi: float = 3.1415926;
let circleArea = (radius: float): float => pi *. radius *. radius;

print_endline(string_of_float(circleArea(20.0)));

let radius = sqrt(circleArea(20.0)) /. pi;

print_endline(string_of_float(radius));

```

### 元组

元组在创建时是不可变的、有序的、有限的，并且是异构类型的(尽管它们可以是相同类型的)。Tuple 类型只是模仿它们所代表的元组的形状!

```ocaml
let myTuple: (char, string, int, string) = ('A', "wonderful", 100, "tuple");
```

有一些特殊的方法可以获得长度为 2 的元组的元素元素。这些元素可以在 Pervasives 模块中使用。

```ocaml
let twoTuple: (string, string) = ("It", "Me");
let first: string = fst(twoTuple);
let second: string = snd(twoTuple);

print_endline(first);
print_endline(second);
```

大多数元组元素都是使用解构来访问的。

```ocaml
let (_, _, third: int, _) = myTuple;
print_endline(string_of_int(third));
```

元组对于模式匹配多个参数特别有用。

```ocaml
let rotate = (x, y) =>
  switch (x, y) {
    | (180, (-180)) => print_endline({j|Rotate $x, $y|j})
    | (90, (-90)) => print_endline({j|Turn $x, $y|j})
    | (_, _) => print_endline("Hold steady!")
  };

rotate(180, -180);
rotate(90, -90);
rotate(50, 70);
```

### 记录

记录类似于 JS 对象，但更轻，默认情况下不可变，固定在字段名称和类型中，速度更快，键入更严格。记录类型是必需的 - 如果不写类型，编译器会报错！请务必使用 mutable 关键字预先设置可变属性。

```ocaml
type team = {
  name: string,
  mutable rank: int,
  average: float,
};

let redSox: team = {
  name: "Red Sox",
  rank: 1,
  average: 0.326
};

```

记录的 `keys` 通过点符号访问。

```ocaml
print_endline(redSox.name);
print_endline(string_of_float(redSox.average));
```

使用扩展操作符可以从旧记录中创建新的记录。但是，扩展不能添加新字段，因为记录受到类型限制。

```ocaml
let redSoxUpdate = {...redSox, average: 0.418};
print_endline(string_of_float(redSoxUpdate.average));
```

现有的记录可以使用 `=` 更新可变字段。

```ocaml
redSox.rank = redSox.rank + 1;
print_endline(string_of_int(redSox.rank));
```

在 Reason 中记录也有双关语，类似于ES6对象简写语法。如果值和键匹配，这允许您只提供关键名称。

```ocaml
let capital = "Olympia";
let population = 6000000;

type state = {
  capital: string,
  population: int,
};

let washington: state = {
  capital,
  population
};
```

你也可以用类型双关！

```ocaml
type place = {
  state,
  team,
};

let seattle: place = {
  state: washington,
  team: {
    name: "Mariners",
    rank: 3,
    average: 0.298,
  },
};
```

尽管记录与 JS 对象有些类似，但您必须更加认识它们的类型。将数据记录用于不改变形状的数据。记录被编译为具有数组索引访问权限的 JS 数组，使其快速。更改记录的类型还可以帮助标记需要更新数据结构的位置，从而使调试变得更加简单！

如果你对使用 JS 本地对象感兴趣，Reason 提供了一个简写语法。这涉及用双引号（`""`）来包装 `key` 名称。

```ocaml
type jsObject = {.
  "response": {.
    "data": {.
      "starCount": int,
      "watchers": int
    },
    "code": int
  }
};

let jsObject:jsObject = {
  "response": {
    "data": {
      "starCount": 9,
      "watchers": 2
    },
    "code": 200
  }
};
```
要访问字段，请使用 `##` 符号。

```ocaml
let starCount: int = jsObject##reponse##data##starCount;
print_endline(string_of_int(starCount));
```

### 变种

变体是 Reason 中唯一的数据结构。他们让我们表达这种或那种关系。

```ocaml
type tweetQuality =
  | Dope /* 这些被称为变体的构造函数或标签。 */
  | Sweet
  | NotBad
  | AF
```

变体通常与 Reason 的 switch 语句一起用于模式匹配。

```ocaml
let tweetStatus = status: string =>
  switch (status) {
    | Dope => "That was a dope tweet!"
    | Sweet => "Pretty sweet tweet!"
    | NotBad => "Not great, but not bad!"
    | AF => "Pretty af tweet my friend!"
  };

print_endline(tweetStatus(AF));
```

变体需要明确的定义。通过调用它们所在的模块来导入它们。

```ocaml
let team: Team.seattleVariant = Mariners;
```

变体构造函数也可以带参数。查看 Team.re 中的 seattleVariant，它具有以下形状：

```ocaml
type seattleVariant =
  | Mariners(player)
  | Sonics(player, year)
  | Seahawks
  | Sounders;
```

这看起来很像函数参数！我们可以在模式匹配中使用这个优势！

```ocaml
open Team;
let player: Team.bostonVariant = RedSox("Mookie Betts");

let namePlayer = arg =>
  switch (arg) {
    | RedSox(name) => {j|You chose $name|j}
    | Celtics(name, year) => year < 2008 ? name : "Big 3"
    | Patriots => "Malcolm Butler"
    | Bruins => "Zdeno Chara"
  };

print_endline(namePlayer(player));
print_endline(namePlayer(Celtics("Larry Bird", 2009)));
print_endline(namePlayer(Celtics("Larry Bird",1984)))
```

独立库为您提供了一些很酷的变体。`type option('a) = None | Some('a)`;- 这允许你定义可以为 `null` 或者 `undefined` 的类型。例如，`option(int)` 键入一个变量作为可为空的整数。

```ocaml
let isNull = true;

let possiblyNullInt: option(int) =
  if (isNull) {
    None
  } else {
    Some(5);
  };

let checkNull = (num: option(int)) => {
  switch (num) {
    | Some(int) => false
    | None => true
  };

print_endline(string_of_bool(checkNull(possiblyNullInt)));
}
```

### 列表和数组

列表是同类型的，不可变的，并且在预添加项目时很快。列表看起来很像 JS 中的数组。

```ocaml
let fibList: list(int) = [1, 1, 2, 3, 5, 8, 13, 21];
```

要添加到列表中，请使用延展运算符。这不会改变原始列表。相反，新列表维护一个链接到扩展列表。例如，下面的 fibListHeadZero 与 fibList 共享其元素，使其非常高效。

```ocaml
let fibListHeadZero = [0, ...fibList];
```

需要注意的是，在 Reason 中不允许使用双延展操作符，即不允许类似这样 [a,...b,...c] 。要访问任意列表项，请使用 List 模块中的 List.nth。

```ocaml
let five = List.nth(fibList, 4);
print_endline(string_of_int(five));
```
要获得列表的长度，使用List.length。

```ocaml
let length: int = List.length(fibList);
let lastItem: int = List.nth(fibList, length - 1);
```

List 模块 附带有用于在列表上操作的附加内置方法。

```ocaml
let reverse = List.rev(fibList);
let sum = List.fold_left((acc, el) => acc + el, 0, fibList);
print_endline(string_of_int(sum));

let thriteen = List.find(item => item === 13, fibList);
print_endline(string_of_int(thirteen));
let aboveTen = List.filter(item => item > 10, fibList);

List.iter(item => print_endline(string_of_int(item)), aboveTen);
```

数组就像列表，但是对于随机访问和更新是可变的和优化的。它们的大小是固定的，但在 JS 上是灵活的。数组的两端都用 `[|` 和 `|]` 表示。

```ocaml
let fibArray: array(int) = [|1, 1, 2, 3, 5, 8, 13, 21|];
```
数组访问和更新类似于 JS。

```ocaml
let length: int = Array.length(fibArray);
let lastItem: int = fibArray[length - 1]
fibArray[2] = 500;
```

您也可以使用标准库中的 `Array.get` 和 `Array.set`。

```ocaml
fibArray[2] = 1000;
print_endline(string_of_int(fibArray[2]));
```

要将数组转换为列表，请在 ArrayLabels 模块中使用 `.to_list` 函数。

```ocaml
let fibArrayAsList: list(int) = ArrayLabels.to_list(fibArray);
let fibListAsArray: Array(int) = ArrayLabels.of_list(fibArrayAsList);
```

Reason 还支持多维数组。前两个参数指定数组的维数，而第三个参数提供填充数组的初始值。

```ocaml
let multiDemArray = Array.make_matrix(2, 2, "Initial.");
Js.log(multiDemArray);
```

### 函数

函数使用 => 声明。单行函数是允许的。多行函数应该被 `{}` 包围。在理由中，所有函数都有参数。如果没有显式参数传递，我们传递 `()`，unit 类型。

```ocaml
let noArg = () : unit => print_endline("This is unit!");
let add = x => x + x;
let square = x => x * x;
```

管道操作符的预览！

```ocaml
let addAndSquare = x => x |> add |> square;

print_endline(string_of_int(addAndSquare(4)));
```

Reason 也有标签参数的概念。由于 Reason 支持 currying（柯里化），因此我们可以使用带标签的参数以任意顺序指定参数。

```ocaml
let concatStringInt = (~int: int, ~str: string) =>
  string_of_int(int) ++ " " ++ str;

print_endline(concatStringInt(~str="is an int.", ~int=50));
```

您也可以使用标签参数来在函数中使用。

```ocaml
let calcTriangleArea = (~base as b: float, ~height as h: float): float => 0.5 *. b *. h;
print_endline(string_of_float(calcTriangleArea(~base=2.0, ~height=7.0)));
```

### 柯里

Reason 函数可以自动部分调用。事实上，Reason 中的所有函数都接受一个参数！

```ocaml
let multiply = (x, y) => x * y;
let multiplyByFive = multiply(5);
let result = multiplyByFive(6);

print_endline(string_of_int(result));
```

上面的乘法函数相当于

```ocaml
let multiply = (x, y) => x * y;
```

OCaml 为我们优化了这一点，以避免不必要的函数分配。

### 可选的标签参数

在 Reason 中可以用 `=?` 创建可选的标签参数。

```ocaml
let sayHello = (~greeting as g, ~name=?, ()) => {
  let person =
    switch (name) {
      | None => ""
      | Some(a) => a
    };
  print_endline(g ++ " " ++ person);
};

sayHello(~greeting="Marhaba", ());
sayHello(~greeting="Ahlan ya", ~name="Parker", ());
```

在函数定义的第三个索引中注意到括号 ()，并在上面调用。没有它，Reason 就无法解析函数。`greeting` 和 `name` 都可以柯里化和无序运用，所以目前还不清楚 sayHello(~greeting ="Aloha") 是什么意思。OCaml 解析 () 表示可选标记的 arg 被省略了。否则，它会将该函数解析为正在等待要应用的名称的curried函数。

```ocaml
/* 这里我们调用上面定义的实际函数sayHello。 */
let actualFunction = sayHello(~greeting="Marhaba", ());

/* 这里我们返回一个可以接受〜name参数的函数。 */
let curriedFunction = sayHello(~greeting="Marhaba");

curriedFunction(~name="Parker", ());

```

有时，您不知道您转发给函数的值是 None 还是 Some(val)。在这种情况下，您可以提供一个明确传递的可选类型。

```ocaml
let possibleName: option(string) = Some("Formidable");

sayHello(~greeting="Hi ya", ~name=?possibleName, ());
```

如果 possibleName 具有构造函数 None，那么上面的函数仍然可以工作！

您还可以提供默认值，如 JS。只需在参数定义中使用 `=` 来指定它们即可。下面，除非通过明确的值，否则 Aloha 将成为问候语的值。

```ocaml
let sayHello = (~greeting="Aloha", ~name=?, ()) => {
  let person =
    switch (name) {
      | None => ""
      | Some(a) => a
    };
  print_endline(greeting ++ " " ++ person);
};

sayHello();
```

### 递归函数

要定义递归函数，请使用 `rec` 关键字。

```ocaml
let rec factorial = (num: int) =>
  if (num === 0) {
    1;
  } else {
    num * factorial(num - 1);
  };

print_endline(string_of_int(factorial(5)));
```

### 相互递归函数

函数可以在 Reason 中递归调用对方。使用 `and` 关键字来实现这一点。下面的示例还预览了我们如何在 Reason 中使用异常来创建自定义错误。

```ocaml
exception FactorialArgument(string);

let rec factorialEven = (num: int) =>
  if (num === 0) {
    1;
  } else {
    switch (num mod 2) {
      /* 模式匹配来检查数字是偶数还是奇数。 */
      | 0 => num * factorialOdd(num - 1)
      | 1 =>
        raise(
          FactorialArgument(
            "factorialEven only accepts even-numbered arguments."
            )
          )
      | _ => 1
    };
  }
and factorialOdd = (num: int) =>
  if (num === 0) {
    1;
  } else {
    switch (num mod 2) {
      | 0 =>
        raise(
          FactorialArgument(
            "factorialOdd only accepts odd-numbered arguments."
            )
          )
      | 1 => num * factorialEven(num - 1)
      | _ => 1
    };
  };

print_endline(string_of_int(factorialEven(6)));
print_endline(string_of_int(factorialOdd(5)));

```

以下调用会抛出我们的 FactorialArgument 异常，并说 factorialEven 只接受偶数参数。

```ocaml
print_endline(string_of_int(factorialEven(5)));
```

如果您来自 JS，上述模式的用处就会起作用，因为 Reason / OCaml 不会提升变量或函数声明！考虑如何调用两个函数 a 和 b 来调用另一个函数。这在 JS 中是可行的，因为函数声明被提升到作用域的顶部。既然 Reason 没有提升，相互递归就是解决这个问题的一种技术。

### 更多关于类型

您可以在 Reason 中创建参数化类型，以使类型更具表现力。它们像函数一样，接受参数和返回类型。类型参数前缀为 `'`。

```ocaml
type measurements('a) = ('a, 'a)
type measurementsInt = measurements(int);
type measurementsString = measurements(string);

let modalSize: measurements(int) = (150, 300);
let modalArea = fst(modalSize) * snd(modalSize);

print_endline(string_of_int(modalArea));

let dialogSize: measurements(string) = ("500", "1000");
let (w, h) = dialogSize;
let dialogDescription = {j| This dialog is $w by $h px |j};

print_endline(dialogDescription);

```

大多数情况下，Reason 的类型推断将会为您处理参数类型!

类型也可以使用变体。

```ocaml
type httpResult('a, 'b) =
  | Success('a)
  | Failure('b);

type payload = {
  data: string,
  code: int,
}
```
组合类型 `httpResult` 需要两个参数，并将它们应用于 `Success` 和 `Failure` 构造函数。

```ocaml
let result: httpResult(payload, int) = Success({
    data: "woohoo",
    code: 200
  });

let errResult: httpResult(payload, int) = Failure(404);
```
由于 Reason 的类型系统允许使用类型级别的函数，因此不需要太多类型的样板。例如，我们可以使用 `list(int)` 和 `list(string)`，而不需要为每个都创建一个新的基本类型，即 listOfInt 和 listOfString。这使得 Reason 的类型系统具有超凡的表现力，同时仍然提供了坚如磐石的系统的脚手架。请享用！

### 相互递归类型

类型如函数，可以是相互递归的。

```ocaml
type professor = {courses: list(course)}
and course = {
  name: string,
  professor,
};
```

### 解构

解构是 Reason 中的常见模式，对于从结构中提取数据非常有用。

```ocaml
let teams = ("Mariners", "Red Sox", "Astros", "Twins");
let (ms, bosox, stros, twins) = teams;

print_endline(
  {j|$ms, $bosox, $stros, $twins === Parkie-Doo's playoff picks.|j}
  );
```

以下是如何解构 Reason 记录。它看起来很像 JS 中的对象解构。

```ocaml
type album = {
  name: string,
  artist: string,
  year: int,
};

let myFavoriteAlbum: album = {
  name: "Illinois",
  artist: "Sufjan Stevens",
  year: 2004,
};

let {name, artist, year} = myFavoriteAlbum;

print_endline({j|$artist wrote $name in $year.|j});
```

当您解构它们的时候也可以给变量起别名，全部在一行中！这与 ES6 解构相似。

```ocaml
let {name: n, artist: a, year: y} = myFavoriteAlbum;

print_endline({j|$a wrote $n in $y.|j})
```

你甚至可以解构和别名函数参数！

```ocaml
type exclamation = {
  phrase: string,
  volume: float,
};

let exclaim = (~exclamation as {phrase} as exclamation) =>
/* 您可以访问惊叹号（记录）和 短语属性作为一个解构变量。 */
print_endline(
  {j|And lo, Parkie-Doo shouted, $phrase at $exclamation.volume DB.|j}
  );

exclaim(~exclamation={phrase: "Breathtaking, this Reason!", volume: 120.7});
```

### 模式匹配

模式匹配是将数据与一组值相匹配的好方法。模式匹配最好与变体一起使用 - 何时这样做，我们可以从类型系统获得很好的全面帮助，从而检查不匹配的案例。

```ocaml
type victory =
  | NailBiter(int)
  | BlowOut(int)
  | OT(string, int);

let myVictory: victory = OT("8:03", 1);
```

模式匹配允许我们解构变体，分别处理每个案例。要查看编​​译器警告你一个不匹配的情况，请尝试在下面注释掉 BlowOut。

```ocaml
let myOTVictory =
  switch (myVictory) {
    | NailBiter(margin) => {j|yeesh, close game. Nice win by $margin.|j}
    | BlowOut(margin) => {j|Damn, what a blowout. $margin run is impressive|j}
    | OT(time, margin) => {j|It took $time to win by $margin. But a win's a win.|j}
  };

print_endline(myOTVictory);
```

我们也可以用其他数据结构来切换其他情况。例如，一个 `array(int)`。

```ocaml
let arr = [|500, 600|];

let handleArray = (array: array(int)) =>
  switch (array) {
    | [|500, 600|] => print_endline("This is a very specific case.")
    | [|500, _|] => print_endline("You have two items in this array, and the first is 500.")
    | [|_, _|] => print_endline("You have two items in this array.")
    | _ => print_endline("This is the default.")
  };

handleArray(arr);
handleArray([|500, 601|]);
handleArray([|101, 102|]);
handleArray([|1s|]);
```

你甚至可以模式匹配一​​组结果到一个特定的结果。例如，让我们将服务器上的错误映射到特定的结果。

```ocaml
type httpResultWithCode =
  | Success(int, list(string))
  | Failure(int);

let handleResult = (res: httpResultWithCode) =>
  switch (res) {
    | Success(200, data) =>
      let f = (acc, el) => acc ++ " " ++ el;
      /* 我们在这里使用 fold_left 来连接字符串。 fold_left 类似于 reduce，但用于列表！ */
      let resString = ListLabels.fold_left(~f, ~init="", data);
      print_endline({j|data: $resString|j});
    | Failure(500)
    | Failure(502) => print_endline("Server error.")
    | Failure(404) => print_endline("Not found.")
    | _ => print_endline("we don't know what happened, sorry!")
  };

handleResult(Failure(500));
handleResult(Failure(501));
handleResult(Success(200, ["You", "Rock"]));
handleResult(Success(201, ["You", "Are", "Still", "Great"]));
```

您也可以使用 `when` 子句来检查案例的特定条件。这就像在你的模式匹配逻辑中添加一点点 `if` 语法糖。扩展我们上面的例子：

```ocaml
let isServerError = err => err === 500;
let isBadGateway = err => err === 502;

let handleResult = (res: httpResultWithCode) =>
  switch (res) {
    | Success(200, data) =>
      let f = (acc, el) => acc ++ " " ++ el;
      let resString = ListLabels.fold_left(~f, ~init="", data);
      print_endline({j|data: $resString|j});
    | Failure(errCode) when isServerError(errCode) =>
      print_endline("Server error.")
    | Failure(errCode) when isBadGateway(errCode) =>
      print_endline("Bad gateway. Getaway? Who knows?")
    | Failure(404) => print_endline("Not found.")
    | _ => print_endline("we don't know what happened, sorry!")
  };

handleResult(Failure(500));
handleResult(Failure(502));
```

您也可以嵌套模式，这意味着我们可以对数据结构的嵌套属性进行模式匹配。

```ocaml
type composer = {
  name: string,
  concertos: int,
};

let evaluateComposer = (composer: composer) =>
  switch (composer) {
    | {name: "Beethoven" | "Mozart" | "Debussy"} => "What high class. How fancy!"
    | composer when composer.concertos <= 7 => "Not too bad, but nothing special."
    | _ => "Just another composer"
  };

print_endline(evaluateComposer({name: "Debussy", concertos: 57}));
print_endline(evaluateComposer({name: "Jerry", concertos: 7}));
```

### 变动

`let` 绑定默认情况下是不可变的。如果你需要改变一个 `let` 绑定,使用 `ref` 关键字的包起它的值。

```ocaml
let mutableVar = ref("mutable");
```

要访问它，请使用 `^` 运算符。

```ocaml
let mutableReference = mutableVar^;
```
要重新分配一个可变变量，请使用 `:=` 运算符。

```ocaml
mutableVar := "mutated";
print_endline(mutableVar^);
```

然而，你也可以简单地通过用另一个let绑定来映射它们来改变变量。

```ocaml
let shodow = "First I am this!";

print_endline(shadow);

let shadow = "But now I've been shadowed!";

print_endline(shadow);
```

### 命令式循环

for 循环在 Reason 中迭代从一个开始直到包括一个结束值。

```ocaml
let repeatCapitalize = (~time: int, ~str: string) => {
  let result = ref("");
  for (time in 1 to times) {
    switch (time mod 2) {
      | 0 => result := result^ ++ String.capitalize(str)
      | 1 => result := result^ ++ str
      | _ => result := result^ ++ str
    };
  };
  result^;
};

print_endline(repeatCapitalize(~time=5, ~str="reason"));
```

您也可以使用 `downto` 操作符从一个数字开始迭代。

```ocaml
let factorialSum = (~num: int): int => {
  let result = ref(0);
  for (n in num downto 0) {
    result := result^ + n;
  };
  result^;
};

print_endline(string_of_int(factorialSum(~num=5)));
```

总的来说，`Array` 和 `List` 方法可能会让你更接近你想要的东西，而不是 for 循环，但是它们很好知道。您也可以像在 JS 中那样使用 while 循环。

```ocaml
let count = ref(1);

while (count < ref(5)) {
  print_endline("We are while looping!");
  count := count^ + 1;
};
```
使用上面的可变绑定来跳出循环。Reason 没有像 JS 这样的 `break` 关键字的概念。

### JSX

Reason 本身支持 JSX 。调用如下：

```ocaml
<div foo={bar}> child1 child2 </div>
```

变成：

```ocaml
([@JSX] div(~foo=bar, ~children=[child1, child2], ()));
```

[@JSX] 语法将一个函数标记为想要格式化为 JSX。这在任何 Reason 库中都是允许的，而不仅仅是 ReasonReact，来利用JSX。

### 外部对象

外部是理性与其他语言（通常是 JS ）的交互方式。使用外部最常用的方法之一是加载模块。例如，让我们从这个 repo 中的 findS 模块加载 findS 函数。

```ocaml
[@bs.module "./findS"] external findS: string => int = "";

let twos = findS("strings");

print_endline(string_of_int(twos));
```

优雅！我们从 Reason 中调用了一个 JS 函数！上面的注释允许我们从 findS 模块中键入函数 findS，并返回一个类型安全引用到该模块。让我们看看我们如何访问一个作用域函数，比如 Math.random。

```ocaml
[@bs.val] [@bs.scope "Math"] external random : unit => float = "random";
[@bs.val] [@bs.scope "Math"] external floor : float => int = "floor";
```

在上面的定义中，我们说在 Math 作用域中有一个方法叫做 `random`。它将 unit 类型作为参数并返回一个浮点数。我们将它别名命名为 `random` ，以便在 Reason 中使用。现在我们可以使用它！

```ocaml
let getRandomInt = (~max as m: float) =>
  floor(random() *. float_of_int(floor(m)));

print_endline(string_of_int(getRandomInt(~max=100.0)));
```

### 对象

大多数情况下，您将使用记录来存储名称和值。但有时候，你会想要一个对象。请注意，Reason 中的对象与 JS 对象不同。使用 `pub` 关键字将对象的公共值前缀。 `val` 可用于定义不可从外部访问的对象上的值。 `pri` 可以定义私有方法。

```ocaml
let anUntypeReasonObject = {pub city = "Burlington"; val state = "Vermont"};

print_endline(anUntypeReasonObject#city);
```

Reason 对象不需要类型定义。如果我们确定它们，则该对象必须具有所提供的类型的形状。

```ocaml
type locale = {
  .
  city: string,
  state: string,
  population: int,
};

let burlington: locale = {
  pub city = "Burlington";
  pub state = "Vermont";
  pub population = 56000
};

print_endline(string_of_int(burlington#population));
```

两个点（称为elision）表示此对象已打开，并且可以具有除原始类型属性之外的属性。开放对象是多态的，需要一个类型参数。

```ocaml
type localeOpen('a) = {.. getplace: unit => string} as 'a;
```

Reason 中的对象有一个 `this` 上下文，它指向对象本身。

```ocaml
let vt: localeOpen({. getPlace: unit => string}) = {
  val address = "100 Church St";
  val city = "Burlington";
  val zipCode = "05712";
  pub getPlace = () =>
    address ++ ", " ++ city ++ " " ++ zipCode ++ ". " ++ this#addOn();
  pri addOn = () => "Didn't ya know?"
};

print_endline(vt#getPlace());
```

大多数情况下，您将使用记录或 JS 对象来实现您在 JS 中使用的内容。Reason 有一个很好的定义 JS 对象的语法。这涉及到将对象键包装在 `""` 中并使用 `##` 进行访问。

```ocaml
let reason = {"color": "orange", "language": "reason", "users": 5000};
print_endline(reason##language);
```

改变 JS 对象涉及让 BuckleScript 知道我们有一个可变的键 - 值对。为此，请使用 `@bs.set` 注释类型。然后，使用 `#=` 改变它。

```ocaml
type language = {
  .
  [@bs.set] "color": string,
  "language": string,
  "users": int,
};

[@bs.module "./findS"] external myJSObject : language = "";

print_endline(myJSObject##language);

myJSObject##color#="orange";

print_endline(myJSObject##color);
```

### 模块

Reason 中的模块可以包含类型定义，`let` 绑定，嵌套模块，几乎任何东西。使用 module 关键字来创建一个。

```ocaml
module Earth = {
  let continents = [|
    "Africa",
    "Antarctica",
    "Asia",
    "Australia",
    "Europe",
    "North America",
    "South America",
  |];

  let pickContinent = (idx: int) => continents[idx];
};

let aussie = Earth.pickContinent(3);

print_endline(aussie);
```

我们也可以访问嵌套模块！查看 `Team` 模块，它有一个嵌套的 `Boston` 模块。

```ocaml
print_endline(Team.Boston.team);
```

虽然 Reason 有很好的模块推理 - 没有 `import` 概念！ - 使用 open 关键字明确地打开模块可能很有用。这将模块纳入作用域内。我们可以在本地为一个特定的函数打开一个模块，如下所示：

```ocaml
let fact =
  Team.Boston.(
    switch (team) {
      | "Red Sox" => "The Red Sox are DOPE."
      | _ => "Eh, don't really care."
    }
  );

print_endline(fact);
```

全局打开对于在另一个模块中获取所有内容非常有用。但是要小心，如果模块中有共享名称的成员，这可能会引入不必要的命名冲突/阴影。

```ocaml
open Team.Boston;
```

模块也可以相互扩展，在传统的 OOP 语言中履行继承或混合的角色。

```ocaml
module ExtendedBoston = {
  include Team.Boston;
  let basketball = "Celtics";
};

print_endline(ExtendedBoston.team);

print_endline(ExtendedBoston.basketball);
```

### Promises

Reason 中的 Promises 通过使用 Js.Promise.* 方法的 BuckleScript 处理。注意。`.` 作为第一个参数传递到 `resolve` 。这可以让 BuckleScript 知道这是一个非柯里化函数。这是在 BuckleScript 2.2.2 中引入的。如果使用旧版本的 bs-platform ，则可以使用 `[@bs] resolve(100)` 获得相同的效果。

```ocaml
let promise = Js.Promise.make((~resolve, ~reject as _) => resolve(. 100));

exception Failure(string);
let failedPromise = Js.Promise.make((~resolve as _, ~reject) => reject(. Failure("Rejected!")));

promise
|> Js.Promise.then_(res => {
    Js.log(res);
    Js.Promise.resolve(res);
})
|> Js.Promise.then_(res => {
    Js.log("That's all folks!");
    Js.Promise.resolve(res - 100);
})
|> Js.Promise.catch(err => {
    Js.log2("Failure!!", err);
    Js.Promise.resolve(-1);
});

failedPromise
|> Js.Promise.then_(res => {
    Js.log(res)
    Js.Promise.resolve(res);
})
|> Js.Promise.then_(res => {
    Js.Promise.resolve(res - 100);
})
|> Js.Promise.catch(err => {
    Js.log2("Failure!!", err);
    Js.Promise.resolve(-1);
});
```

### 异常

异常是一种特殊的变体，在 Reason 中不常使用。

下面的函数展示了如何使用 `raise` 关键字使用 `Not_found`、`exception`。

```ocaml
let aFunkyList = ["Parliament", "Funkadelic", "George Clinton"];
  if (List.exists(item => item ==="Stevie Wonder", aFunkyList)) {
    print_endline("Yay Stevie!");
  } else {
    /* 在这里，我们引发 Exception 变体的 Not_found 构造函数。 */
    raise(Not_found);
  }
```
