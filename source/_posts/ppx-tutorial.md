---
title: OCaml -ppx 语言扩展教程
created: 2018/09/17
description: OCaml -ppx 语言扩展教程
author: lanqy
---
# OCaml -ppx 语言扩展教程

译自：https://victor.darvariu.me/jekyll/update/2018/06/19/ppx-tutorial.html

> 简要介绍 OCaml 中的 ppx 扩展机制，并附有示例和进一步的指示 - 我希望在准备硕士论文时我知道的事情。

你如何扩展编程语言？老式的方法是编写自己的预处理器，它接受用扩展语法编写的程序，并将它们转换为普通的编程语言(这是早期 c++ 编译器的工作原理)。稍微不那么苛刻的选择是依赖编程语言生态系统提供的工具;OCaml 是为数不多的提供开箱即用功能的d语言之一。在这篇文章中，我尝试从程序员的角度补充现有的集体智慧，了解 OCaml ppx 语言扩展机制的工作原理。

## ppx 基础知识

OCaml 语法支持 4.02 版本中的扩展节点，如 [OCaml 手册](https://caml.inria.fr/pub/docs/manual-ocaml/extn.html#sec261)中所述。扩展节点允许以任意复杂的方式扩展语言 - 它们可以表示表达式，类型表达式，模式等。让我们看一个扩展节点的一个愚蠢的例子：一个将 “+ 1” 项附加到任何代数表达式的节点。它可能看起来如下：

```ocaml
[%addone 1 + 2]
```

扩展节点由两部分组成：属性 id（上面的 addone ）和有效负载（表达式 1 + 2）。属性 id 标识它所代表的扩展节点的类型，以便由适当的重写器处理，而有效负载是需要根据语言扩展的逻辑重写的表达式的主体。在我们的例子中，在作者展开之后，上面的术语应该被阅读

```ocaml
(1 + 2) + 1
```

扩展节点是语法树中的通用占位符，类型检查器拒绝它，并打算由 ppx 重写器扩展它。ppx rewriter 是一种二进制文件，它接收解析器生成的 AST，执行一些转换，并输出经过修改的 AST。

## OCaml AST

那么这个 AST 是什么样的？通过解析生成的 AST 数据类型是 compiler-libs 包的一部分。您可以在 [Parsetree](https://caml.inria.fr/pub/docs/manual-ocaml/libref/Parsetree.html) 和 [Asttypes](https://caml.inria.fr/pub/docs/manual-ocaml/libref/Asttypes.html) 模块中找到类型的定义。要检查解析器在特定表达式上生成的 AST，可以通过运行以下命令来使用 [ppx_tools](https://github.com/ocaml-ppx/ppx_tools) 包中的 dumpast 工具：

```text
ocamlfind ppx_tools/dumpast -e "[%addone 1 + 2]"
```

它生成了下面的语法树片段，其中使用了 OCaml 版本4.05中的 Parsetree/Asttypes 模块中的数据类型。我们可以直观地看到，解析树包含一个扩展节点，带有 addone 属性 id 和一个包含表达式的有效负载。这个表达式是对两个子表达式的加法函数的应用，这两个子表达式仅仅是常量。尝试按原样编译这个小程序将导致由于扩展节点未解释而引起错误——这是 ppx 重写程序的工作。

```ocaml
{pexp_desc =
  Pexp_extension
   ({txt = "addone"},
    PStr
     [{pstr_desc =
        Pstr_eval
         ({pexp_desc =
            Pexp_apply ({pexp_desc = Pexp_ident {txt = Lident "+"}},
             [(Nolabel,
               {pexp_desc = Pexp_constant (Pconst_integer ("1", None))});
              (Nolabel,
               {pexp_desc = Pexp_constant (Pconst_integer ("2", None))})])},
         ...)}])}
```

## AST Helpers

ppx 重写器需要与 AST 片段（例如上面的片段）进行模式匹配并执行转换。这些转换还需要生成有效的 OCaml AST 片段。手工构建这些非常麻烦。为此，[Ast_helper 模块](https://caml.inria.fr/pub/docs/manual-ocaml/libref/Ast_helper.html)提供了用于构造片段的辅助函数。为了构造表达式1 + 2，我们可以使用 Exp.apply 和 Exp.constant 助手 (helpers)，如下所示：

```ocaml
Exp.apply   (Exp.ident {txt = Lident "+"; loc=(!default_loc)}) 
            [(Nolabel, Exp.constant (Pconst_integer ("1", None)));
             (Nolabel, Exp.constant (Pconst_integer ("2", None)))];
```

## AST Mapper

我们任务的 ppx 重写器将使用 [Ast_mapper API](https://caml.inria.fr/pub/docs/manual-ocaml/libref/Ast_mapper.html)，它提供编译器和 ppx 重写器之间的标准接口。它还提供了一个默认的映射器，它只不过是一个深度的身份映射器 - 因此我们只能修改我们感兴趣的语法树部分。使用必要的管道，我们的 addone 重写器将如下所示：

```ocaml
open Ast_mapper
open Ast_helper
open Asttypes
open Parsetree
open Longident

let expr_mapper mapper expr = 
   begin match expr with
      | { pexp_desc =
          Pexp_extension ({ txt = "addone"; loc }, pstr)} ->
        begin match pstr with
        | PStr [{ pstr_desc =
                  Pstr_eval (expression, _)}] -> 
                            Exp.apply  (Exp.ident {txt = Lident "+"; loc=(!default_loc)})
                                        [(Nolabel, expression);
                                         (Nolabel, Exp.constant (Pconst_integer ("1", None)))]
        | _ -> raise (Location.Error (Location.error ~loc "Syntax error"))                       
        end
      (* Delegate to the default mapper. *)
      | x -> default_mapper.expr mapper x;
  end

let addone_mapper argv =
  { 
    default_mapper with
    expr = expr_mapper;
  }
 
let () = register "addone" addone_mapper
```
让我们详细研究这个片段。在第 23 行，我们定义了自定义映射器，它使用我们自己的 expr_mapper 替换默认映射器中的 expr 字段。这意味着我们自己的 expr_mapper 只处理表达式;模式和其他 AST 类型将保持不变。第 7 行的 expr_mapper 的定义将表达式与具有标识符 addone 的扩展节点匹配，其他标识符不应由此映射器处理。然后我们对第 13 行的表达式进行模式匹配，并使用 AST 助手添加另一个函数应用程序 - + 应用于原始表达式和常量1。

为了构建重写器，我们可以使用标准的  ocamlbuild 工具，指定对 compiler-libs 的依赖，其中必要的模块位于：

```text
ocamlbuild -package compiler-libs.common addone_ppx.native
```


要检查重写器是否符合我们的要求，我们可以使用ppx_tools包中的重写器工具。假设 [%addone 1 + 2] OCaml 代码在文件 addone.ml 中：

```text
ocamlfind ppx_tools/rewriter ./addone_ppx.native addone.ml
```

输出 (1 + 2) + 1，这正是我们想要的。该工具还允许将重写的源代码输出到文件而不是stdout。


## 递归呢？

上面给出的重写器不会递归，因此我们不能有嵌套的 addone 节点，例如 [%addone 1 + [%addone 2]]。支持递归是任何有意义的语言添加的关键要求，正是使扩展功能强大的原因。以下将让我们首先在AST的外部节点上应用+1加法;注意第16行的映射器的递归调用。然后重写[%addone 1 + [%addone 2]]表达式将给出（1 +（2 + 1））+ 1。

```ocaml
open Ast_mapper
open Ast_helper
open Asttypes
open Parsetree
open Longident

let rec expr_mapper mapper expr = 
   begin match expr with
      | { pexp_desc =
          Pexp_extension ({ txt = "addone"; loc }, pstr)} ->
        begin match pstr with
        | PStr [{ pstr_desc =
                  Pstr_eval (expression, _)}] -> 
                              Exp.apply  (Exp.ident {txt = Lident "+"; loc=(!default_loc)})
                                          [(Nolabel, (expr_mapper mapper expression));
                                           (Nolabel, Exp.constant (Pconst_integer ("1", None)))]
        | _ -> raise (Location.Error (Location.error ~loc "Syntax error in expression mapper"))                       
        end
      (* Delegate to the default mapper. *)
      | x -> default_mapper.expr mapper x;
  end

let addone_mapper argv =
  { 
    default_mapper with
    expr = expr_mapper;
  }
 
let () = register "addone" addone_mapper
```

## 构建和打包

假设您对 ppx 重写器感到满意。 到目前为止的直观工作流程（使用 ppx 重写器重写文件，编译重写文件）对于玩具问题是可接受的，但对于较大的项目是不可行的。 理想情况下，我们希望能够使用扩展语言编写源文件，而不必担心调用预处理步骤。

这就是打包的地方：将重写器作为包安装（例如，addone.ppx），您只需将其指定为依赖项，编译器将为您处理中间步骤：

```text
ocamlfind ocamlc -package addone.ppx -package decml -linkpkg addone.ml
```

我个人发现 [oasis](http://oasis.forge.ocamlcore.org/) 工具最容易使用，以便在更复杂的场景下构建扩展。whitequark 的教程提供了一个很好的示例配置;然后，您可以使用 [oasis2opam](https://github.com/ocaml/oasis2opam) 等工具将其转换为 [opam](http://opam.ocaml.org/) 格式，在本地固定，甚至发布！您可能还希望查看更复杂的项目（例如 [SLAP](http://akabe.github.io/slap/) ）以查看示例 oasis 配置和项目布局，因为 oasis 文档简短且不完整。值得一提的是，OCaml 社区似乎正在向 [dune](https://github.com/ocaml/dune) （jbuilder）转变为事实上的构建工具，尽管我认为它更难以作为一个完整的初学者使用，特别是对于 ppx 重写器。

## Parsetree 版本

使用 ppx 重写器时的一个警告是 AST 数据类型在不同版本之间略有不同;因此，为 OCaml 版本 4.02 编写的扩展名与例如版本 4.05 的 OCaml 编译器不兼容。社区已经提出了一种自动方式，用于在 [ocaml-migrate-parsetree](https://github.com/ocaml-ppx/ocaml-migrate-parsetree) 库中的不同 AST 版本之间转换扩展。对于本博客中提供的示例，您需要使用 OCaml 4.05 版。如果您只打算支持有限数量的版本，则可能需要避免开销并手动将重写器转换为适当的版本。

```ocaml
{pexp_desc =
  Pexp_apply ({pexp_desc = Pexp_ident {txt = Lident "+"}},
   [(Nolabel, {pexp_desc = Pexp_constant (Pconst_integer ("1", None))});
    (Nolabel, {pexp_desc = Pexp_constant (Pconst_integer ("2", None))})])}
```
AST for 1 + 2 in OCaml version 4.05.

```ocaml
{pexp_desc =
  Pexp_apply ({pexp_desc = Pexp_ident {txt = Lident "+"}},
   [("", {pexp_desc = Pexp_constant (Const_int 1)});
    ("", {pexp_desc = Pexp_constant (Const_int 2)})])}
```
AST for 1 + 2 in OCaml version 4.02.

## 更多资源

- [ppx_tools](https://github.com/ocaml-ppx/ppx_tools) 库在编写 ppx 重写器时提供了有用的功能。 本教程提到了重写器和 dumpast; 作者还提供了 metaquot，它为您提供了一种简单的方法来获取重写器代码中特定表达式的 OCaml 语法树。 例如，使用它，您可以通过编写 [%expr 1 + 2] 来获取 1 + 2 的 AST，而不是使用详细的 parsetree s数据类型来构造它。 这在编写测试时很方便。
- [whitequark 的教程](https://whitequark.org/blog/2014/04/16/a-guide-to-extension-points-in-ocaml/)是关于 ppx 重写器的原始教程，是我的初始起点。
- Shayne Fletcher 编写了一个[优秀的教程](http://blog.shaynefletcher.org/2017/05/preprocessor-extensions-for-code.html)，它使用了多种类型的映射器（不是表达式，而是结构和类型/构造函数声明）。您可能会发现它可用作重写器的另一个用例。


编辑26/06/2018

[这里](https://www.reddit.com/r/ocaml/comments/8sus7f/a_tutorial_to_ocaml_ppx_language_extensions/)有关于 OCaml reddit 的帖子的讨论。 OCaml 社区有帮助指出包装部分已经过时。 您可以在 [Rudi Grinberg 的教程](http://rgrinberg.com/posts/extension-points-3-years-later/)中查看如何使用 dune / jbuilder（较新的构建工具）设置配置，包括运行测试以将语法树与 diff 工具进行比较的巧妙方法。 他提供了一个可以在[这里](https://github.com/rgrinberg/ppx_getenv2)克隆的初学者项目。 有些库已经重新组织，因此您必须更改导入才能使其开箱即用。