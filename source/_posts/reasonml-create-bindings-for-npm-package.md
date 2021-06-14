---
title: ReasonML：为 NPM 包创建绑定
created: 2018/06/15
description: ReasonML：为 NPM 包创建绑定
author: lanqy
---
# ReasonML：为 NPM 包创建绑定

译自：https://medium.com/@DmytroGladkyi/reasonml-create-bindings-for-npm-package-b8a3c6d0703e

ReasonML 正在上升。最新的 https://www.reason-conf.com/ 表明，很多人都对 Facebook 这种语言感兴趣。您可以非常容易地将 ReasonML 添加到现有的 JavaScript / TypeScript 项目中，并获得强类型语言的全部好处，但很多库都是用 JS 编写的，并发布到 NPM 。要从 ReasonML 使用它们，您必须提供绑定到包。为不同的库创建了很多绑定，例如：[MomentJS 的绑定](https://github.com/reasonml-community/bs-moment)。

在这篇文章中，我将向您展示如何从头开始创建绑定，以及如何在您的 ReasonML 项目中使用它们：

```ocaml
[@bs.module "semver"] [@bs.val]
external clean : string => Js.nullable(string) = "clean";

let clean = a => clean(a) |> Js.Nuellable.toOption;

/***** COMPARISON START *****/
[@bs.module "semver"] [@bs.val] external gt : (string, string,) => bool = "gt";

let gt = (a, b) => gt(a, b);
[@bs.module "semver"] [@bs.val]
external gte : (string, string) => bool = "gte";

```

## 入门

ReasonML 提供了一个从 ReasonML 到 JavaScript 世界的非常薄的绑定层。官方的 BuckleScript 文档是一个很好的起点。

我们将为 NPM 官方的 semver 软件包编写绑定。

这个包暴露了不同的函数，我们也可以实例化 Semver 类，就像在这个 JavaScript 例子中一样：

```ocaml
const semver = require('semver')
semver.valid('1.2.3') // '1.2.3'
semver.valid('a.b.c') // null
semver.clean('  =v1.2.3   ') // '1.2.3'
semver.satisfies('1.2.3', '1.x || >=2.5.0 || 5.0.0 - 7.2.3') // true
```

#### 创建新的 NPM 包

运行命令

```ocaml
npm init
```

这将启动一个向导，创建一个新的 NPM 包。提供你想要的任何信息，只要给它一个带有前缀 'bs'（ BuckleScript ）的好名字。 `bs`是 BuckleScript 或 ReasonML 绑定的社区约定。

创建一个文件夹 'src'：

```ocaml
mkdir src
```

它将包含我们绑定的所有源文件。

创建一个文件夹'__tests__'：

```ocaml
mkdir __tests__
```

它将包含由 [bs-jest 库](https://github.com/glennsl/bs-jest) 执行的绑定测试。

#### 添加 bsconfig.json

为了使我们的包与 ReasonML 编译器一起工作，我们必须添加一个 bsconfig.json。

```ocaml
{
  "name": "@gladimdim/bs-semver",
  "version": "0.2.0",
    "sources": [
	{
	    "dir" : "src",
	    "subdirs" : true
	},
	{
	    "dir": "__tests__",
	    "type": "dev"
	}
    ],
  "package-specs": {
    "module": "commonjs",
    "in-source": true
  },
  "suffix": ".bs.js",
  "bs-dependencies": [
  ],
    "bs-dev-dependencies": ["@glennsl/bs-jest"],
  "warnings": {
    "error" : "+101"
  },
  "refmt": 3
}
```

最重要的导入属性：

```ocaml
name: '@gladimdim/bs-semver'
```

必须与 package.json 中的完全相同。

```ocaml
sources: [...src....__tests__....]
```

指定要编译为 JavaScript 代码的文件夹。测试文件夹的类型为 “`dev`”，所以它不会出现在建议中，也不会被编译进软件包中。

#### 编辑 package.json

现在打开package.json，我们必须为它添加一些绑定特定的属性

```ocaml
"script": {
    "clean": "bsb -clean-world",
    "build": "bsb -make-world",
    "watch": "bsb -make-world -w",
    "test": "jest"
},
```

这些是脚本，用于构建，编译，测试和运行监视器。

提供 dev 依赖：

```ocaml
"devDependencies": { 
  "bs-platform": "^3.0.0",  
  "jest": "22.1.2", 
  "@glennsl/bs-jest": "0.3.2" 
},
```

请注意，您必须提供像 'jest' 这样的'真正' JavaScript NPM 包装代码，因为它们包含真实代码，在测试或编译任务期间，这些代码将被来自 'bs-jest' 的绑定使用。

现在告诉 NPM 包含哪些文件：

```ocaml
"files": [
    "src/semver.re",
    "bsconfig.json"
]
```

这是应该发布给 NPM 注册管理机构的内容。包含 bsconfig.json 非常重要，因为它已被最终用户的构建过程所使用。

#### 指定目标NPM软件包的 Peer Dependencies

当我们为 semver 包创建绑定时，我们必须告诉 NPM 使其成为对等关系。我们软件包的最终用户将不得不为我们提供这种对等关系。

```ocaml
"peerDependencies": {    "semver": "^5.5.0"  },
```

## 如何编写绑定

在 src 文件夹中，创建一个名为 semver.re 的文件。这将是我们的主要和唯一的绑定文件。

让我们为函数 'clean' 编写绑定，它作为 semver 包中的独立函数公开。

在我们编写 `clean` 函数的主体之前，我们需要深入到可怕的JS世界：

***您必须始终在运行时检查这些函数在实际中返回什么***。

每个 npm 软件包页面都有一个 “Test With RunKit” 按钮，您可以在不安装软件包的情况下使用它来调用函数：

![1_j4hBNwdBskluPQKGKLPKUA.png](/images/1_j4hBNwdBskluPQKGKLPKUA.png)

函数 'clean' 的问题如下：它可能会返回一个有效的 semver 字符串;如果无法解析输入的 semver 版本，则返回 null。所以，从 ReasonML 的角度来看，这个函数的结果是一个 Option。它要么返回字符串，要么无返回（ None ）。

我们使用指令 @bs.module 和 @bs.val 来指示下一个函数没有 ReasonML 主体。相反，它将从 JavaScript 世界中获取。欲了解更多信息，请阅读官方文档：

https://bucklescript.github.io/docs/en/intro-to-external.html

```ocaml
[@bs.module "semver"] [@bs.val]
external clean : string => Js.nullable(string) = "clean";
let clean = a => clean(a) |> Js.Nullable.toOption;
```

第二行中的类型签名意味着以下内容：函数 'clean' 接受一个字符串作为输入参数并输出一个字符串或 null。指令 @bs.module “semver” 和 “clean” 将把它转换成 JavaScript：

```ocaml
semver.clean()
```

我们可以保持原样，但我们希望使这个函数的返回类型更具有 ReasonML -规范，并使用 Option 类型。这就是为什么在＃3 行我们有这个函数的主体。它以下面的方式读取：函数 'clean' 将参数 a，发送到 'clean'（来自 semver 包的 JavaScript 函数），然后将结果传送到 toOption 转换器。

ReasonML 将继承“ external clean ”声明中的类型定义，因此您不必重复它们。

我们的 ReasonML 函数 'clean' 的输出将是一个 String 类型的 Option 。

我们来写一下绑定测试。使用以下内容在`__tests__`文件夹中创建一个文件 semver_spec.re：

```ocaml
open Jest;

let () = 
    describe(
        "semver",
        ExpectJs.(
            () => {
                test("#clean", () =>
                    expect(
                        Semver.clean("  =1.5.0   ")
                        |> (
                            result =>
                                switch (result) {
                                | Some(v) => v
                                | None => raise(Not_found)
                                }
                        ),
                    )
                    |> toBe("1.5.0")
                );
            }
        ),
    );

```

semver 模块将从我们的 semver.re 文件中自动加载。请记住，我们不测试 Semver 功能，我们测试绑定。

所以我们只需要验证一下，我们的绑定返回的是可选类型，结果是字符串。

我们可以继续覆盖从官方文档到 semver 的其他简单方法：https://github.com/npm/node-semver

## 如何为 'string' 枚举创建类型

函数 semver.cmp(a, c, b) 接受3个参数：第一个版本，操作（字符串），第二个版本。

绑定到它看起来像这样：

```ocaml
[@bs.module "semver"] [@bs.val]
external cmp : (string, string, string) => bool = "cmp";
```

但是，名为“操作”的第二个参数可以是仅来自特定“操作”集的字符串。例如：“<，>，≤，≥，==，！==”等等。

ReasonML 中的用法如下所示：

```ocaml
Semver.cmp("1.5.0", "<", "2.3.5");
```

通过定义第二个参数“<”，作为一个字符串，它使我们有可能犯下以下错误：

```ocaml
Semver.cmp("1.5.0", "hello", "2.3.5");
```

我们可以把它作为一个字符串类型，但是在 ReasonML 中，我总是喜欢为这些重要的参数设置类型。

我们必须引入一个只对'cmp'方法字符串有效的类型：

```ocaml
type comparator = 
    | LooseEqual 
    | LooseNotEqual 
    | Equal 
    | Empty 
    | NotEqual
    | Gt
    | Gte
    | Lt
    | Lte;
```

编写一个将这些类型转换为字符串的函数，因为JavaScript需要一个字符串作为输入：

```ocaml
let comparatorToString = comparator : string =>
    switch (comparator) {
        | LooseEqual => "=="
        | LooseNotEqual => "!=="
        | Equal => "==="
        | Empty => ""
        | NotEqual => "!=="
        | Gt => ">"
        | Gte => ">="
        | Lt => "<"
        | Lte => "<="
    };
```

现在，增强我们的绑定：

```ocaml
[@bs.module "semver"] [@bs.val]
external cmp : (string, string, string) => bool = "cmp";

let cmp = (a: string, c: comparator, b: string) =>
    cmp(a, c |> comparatorToString, b);
```

这个 ReasonML 代码将返回一个编译错误：

```ocaml
Semver.cmp("1.5.0", "hello", "2.3.0");
```

我们必须重用提供的类型 Semver.Gt  ：

```ocaml
Semver.cmp("1.5.0", Semver.Gt, "2.3.0");
```

该绑定将将  Semver.Gt  转换为 “>” 并将其发送到外部“真实” JavaScript 函数。

## 为 Semver 类创建类型

Semver 包还提供了一个实例化 Semver 类的可能性：

```ocaml
const s = new semver("1.5.0");
s.minor(); // 5
```

我们可以在 ReasonML 中定义一个类类型来覆盖所有的 'semver' 对象属性：

```ocaml
class type semverInstance = 
    [@bs]
    {
        pub inc: tRelease => semverInstance;
        pub version: string;
        pub major: int;
        pub minor: int;
        pub patch: int;
        pub raw: string;
        pub build: array(string);
        pub prerelease: array(string)
    };
```

然后，我们添加 'createSemver' 函数，这将帮助我们使所有类型安全：

```ocaml
type tSemver = Js.t(semverInstance);

[@bs.new] [@bs.module] external createSemver : string => tSemver = "semver";
```

用法：

```ocaml
let a = Semver.createSemver("1.5.0");
Js.log(a##minor); // 5
```

## 总结

我希望这篇文章能够帮助你为其他软件包创建自己的类型。有很多很好的绑定 https://github.com/reasonml-community/bs-moment， https://github.com/glennsl/bs-jest 您可以查看它们的源代码，以获取关于如何编写绑定的更多见解。这实际上是我就是这么做的:-)

## ReasonML 周报

要获取有关ReasonML的最新消息，您可以按照我们的 twitter：https://twitter.com/@WeeklyReason 并订阅我们的每周新闻简报：https://news.reasonml.online。

GitHub repo bs-semver 绑定： https://github.com/gladimdim/bs-semver
