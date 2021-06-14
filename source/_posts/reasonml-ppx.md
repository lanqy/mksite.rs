---
title: ReasonML PPX
created: 2018/06/15
description: ReasonML PPX
author: lanqy
---
# ReasonML PPX

译自：https://blog.hackages.io/reasonml-ppx-8ecd663d5640

自从我第一次见到 [Reason](https://reasonml.github.io/) 后，我看到了 “PPX” 这个词。我想知道 PPX 是什么，它做了什么以及如何构建 PPX 。我努力了一下收集方式和原因，所以我想和你分享我的经验。

## 什么是 PPX

PPX 重写器是一个接受序列化抽象语法树（AST）并输出另一个可能被修改的 AST 的程序。

所以，PPX 的目标只是修改 AST ，例如，我们可以在这个 [Github](https://github.com/jaredly/ppx_autoserialize) 项目中看到，添加一个通用的方法：

```ocaml
type myThing = {
    something: int
};

let myValue = {something: 10};
/* 哦，神奇的 myThing__to_devtools 可用! */

Js.log(myThing__to_devtools myValue);
```

你可以真正做到一个或多个 PPX 链接。他们非常强大。例如，您可以使用它将 Reason JSX 翻译成 ReactJS 能够理解的内容。这就是 reactjs_jsx_ppx 所做的。

## 构建一个 PPX

为了更好地理解 PPX 如何工作，我们来构建一个 PPX。我们将其称为 ppx_test，它会将 [%test] 转换为 42。

## 配置 BuckleScript 构建

您应该先配置 [BuckleScript](https://bucklescript.github.io/bucklescript/Manual.html#_first_example_by_hand_without_samples) 构建，方法是更新 bsconfig.json 并指定在构建过程中使用哪个 PPX ：

```json
{
  "name" : "build-a-ppx",
  "ppx-flags": ["ppx_test/ppx"],
  "sources": "src"
}
```
正如 bsconfig.json 的文档所述，您应该传递一个 package_name / binary 并且 BuckleScript 将在您的 node_modules 中查找它。所以，通过这个配置，我们应该在 node_modules / ppx_test / ppx 中有一个二进制 PPX。只需在你的 node_modules 中创建一个 ppx_test 目录。我们将在其中编写我们的 ppx_test.re 文件。

## 写 PPX 

PPX 是类型为 Ast_mapper.mapper 的映射器。这个映射器类型只是一个包含 ParseTree 数据类型的很多映射器函数签名的记录。它看起来像这样：

```ocaml
type mapper = {
    attribute: mapper => Parsetree.attribute => Parsetree.attribute,
    case: mapper => Parsetree.case => Parsetree.case,
    cases: mapper => list Parsetree.case => list Parsetree.case,
    /* 还有很多其他的mapper函数类型 */
}
```

Ast_mapper 模块已经为我们提供了一个默认映射器：

Ast_mapper.default_mapper（类型为 Ast_mapper.mapper ）。

```ocaml
let Ast_mapper.default_mapper: Ast_mapper.mapper;
```

我们将继承这个默认的映射器来创建我们的 PPX ，并且只覆盖将 [%test] 转换为 42 所需的内容。

```ocaml
/* ppx_test.re */
open Asttypes;
open Parsetree;
open Ast_mapper;

let test_mapper  argv => {
    ...default_mapper, /* 我们扩展了 default_mapper */
    /* 并覆盖 'expr' 属性 */
    expr: fun mapper expr => 
    /* 如果表达式是 [%测试] */

    switch expr {
        | {pexp_desc: Pexp_extension {text: "test"} (PStr []) [@implicit_arity]} =>
        /* 然后换成 42 */
            Ast_helper.Exp.constant (Const_int 42)
        | other => default_mapper.expr mapper other
    }
};

let () = register "ppx_test" test_mapper;

```

我们扩展 default_mapper 并覆盖 'expr'属性。如果参数中给出的表达式匹配 [%test] ，则返回 42 .否则，返回 default_mapper 实现。

## 构建 PPX

BuckleScript 需要一个二进制文件：node_modules / packages_name / binary_file。在我们的例子中：node_modules / ppx_test / ppx。

关于 ReasonML 编译如何在这里有一个很好的介绍。让我们看看我们如何在二进制文件中逐步构建我们的 .re 文件：

```ocaml
// 1. 构建 OCaml 文件

ocamlc -o outputfile yourOcamlFile.ml

/*
2. 构建 ReasonML 文件

如果你想构建 ReasonML 文件，你需要通过带有 pp 标志的预处理器来传递它。
refmt 是 Reason reformat（它随 [reason-cli](https://github.com/reasonml/reason-cli) 提供）。在这种情况下，它将打印ReasonML文件的二进制文件。-impl 告诉编译器 .re 应该被认为是一个普通的 .ml（ OCaml 文件）
*/
ocamlc -pp "refmt --print binary" -o outputFile -impl yourReasonFile.re

/*
3.使通用模块可用 我们需要 OCaml Common 模块。 -I 搜索依赖关系，'+' 使其相对于 OCaml 目录
*/

ocamlc -pp "refmt --print binary" -o ppx -I +compiler-libs ocamlcommon.cma -impl ppx_test.re

```

用最后一条命令，我们生成我们的 ppx 二进制文件。所以我们现在有了 node_modules / ppx_test / ppx

## 使用魔法

```ocaml
render: fun _ => {
    <div>
        (ReasonReact.stringToElement (string_of_int [%test]))
    </div>
}
```

我们拥有它。当我们用 bsb -make-world 运行我们的项目时。 BuckleScript 将读取 bsconfig.json 中的构建配置，然后通过在我们的 node_modules 中查找来注册我们的 PPX。

> 您可以在我们的网站上找到更多关于 Hackages 的信息。我们是一家社区驱动的公司，提供围绕 JavaScript 的最新框架和技术的高级培训。我们也乐于[贡献开源资源](https://github.com/hackages)，并组织全欧各地的免费社区活动！通过https://hackages.io 查看我们即将举办的活动。