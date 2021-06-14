---
title: OCaml 操作符备忘单
created: 2018/09/17
description: OCaml 操作符备忘单
author: lanqy
---
# OCaml 操作符备忘单

译自：https://www.brendanlong.com/ocaml-operator-cheatsheet.html

学习 OCaml 最困难的部分之一就是弄清楚中缀运算符的作用，因为它们只是一串符号而你无法通过谷歌搜索找到它们。这是我试图制作一个备忘单，无论何时你想知道一系列随机符号是什么意思。在此页面上搜索应该找到有关任何常见 OCaml 运算符的基本信息。请注意，某些库定义了自己的运算符，例如 Jane Street 的[Command.Spec](https://ocaml.janestreet.com/ocaml-core/latest/doc/core/Core/Command/Spec/index.html#val-(++)) 如何定义 ++，+> 和 +<。

## 关于中缀函数的一般信息

在 OCaml 中，如果函数名称以以下字符之一开头，则函数为中缀：

```ocaml
= @ ^ | & + - * / $ %
```

其次是零个或多个这些字符：

```ocaml
! $ % & * + - . / : ? @ ^ | ~
```

定义中缀函数时，需要在 “name” 周围加上 ()。 例如，在 [utop](https://github.com/diml/utop) 中：

```ocaml
# let (=<>@^|&~+-*/$%!?:.) a b = a + b ;;
val ( =<>@^|&~+-*/$%!?:. ) : int -> int -> int = <fun>

# 1 =<>@^|&~+-*/$%!?:. 2;;
- : int = 3
```

此外，您可以通过再次将函数名称包装在括号中来查看 utop 中的中缀运算符的类型：

```ocaml
# (=<>@^|&~+-*/$%!?:.);;

val ( =<>@^|&~+-*/$%!?:. ) : int -> int -> int = <fun>
```

这里的[官方文档在这里](https://caml.inria.fr/pub/docs/manual-caml-light/node4.9.html)，虽然这个[博客](https://haifengl.wordpress.com/2014/07/02/ocaml-functions/)有一个更容易理解的解释。

## 内置中缀运算符

内置运算符在 [Pervasives](https://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html) 中定义：

请参阅文档，了解涉及多种类型（=，<>，<，>等）的函数所涉及的魔力。


操作符 | 描述
------------ | -------------
=	| 结构相等 [1](https://stackoverflow.com/a/13596236/212555)
<>	| 结果不等 [1](https://stackoverflow.com/a/13596236/212555)
<	| 小于
>	| 大于
<=	| 小于等于
>=	| 大于等于
==	| 物理相等（同一对象）[1](https://stackoverflow.com/a/13596236/212555)
!=	| 物理不相等（不是同一个对象）[1](https://stackoverflow.com/a/13596236/212555)
&&	| 布尔和
&	| (已弃用) 布尔和
\|\|	| 布尔或
\|	| （已弃用）布尔或
\|>	| 反函数应用（x \|> f 与 f x 相同）。也称管道运算符
@@	| 功能应用（f @@ x与 f x 相同）
~-	| 整数否定（与一元相同 - ）
~+	| 被描述为 “一元加法” 但似乎没有做任何事情。
+	| 整数加法
-	| 整数减法
*	| 整数乘法
/	| 整数除法
~-.	| 浮动否定（与一元相同 -.）
~+.	| 被描述为 “一元加法” 但似乎没有做任何事情。
+.	| 浮点数加法
-.	| 浮点数减法
*.	| 浮点数乘法
/.	| 浮点数除法
**	| 浮点数幂
^	| 字符串连接
@	| 列表连接
!	| 获取 ref 的值
:=	| 设置（修改） ref 的值
^^	| 格式化字符串连接

## Jane Street

### Numbers

Jane Street 通常在模块中定义有意义的算术运算符，因此您可以执行以下操作：

```ocaml
Bigint.(of_int 1 + of_int 3 / of_int 5)
```

此接口的文档位于 [Int_intf.S_common](https://ocaml.janestreet.com/ocaml-core/latest/doc/base/Base/Int_intf/module-type-S_common/index.html) 下，尽管它们中的大多数也是针对浮点数定义的。

操作符 | 描述
------------ | -------------
+	| 特定模块的添加（即 float.(+) 是浮动添加）
-	| 特定模块的减法
*	| 特定模块的乘法
/	| 特定模块的除法
//	| 整数除法返回浮点数
%	| 中缀 [mod](https://en.wikipedia.org/wiki/Modulo_operation) (结果总是整数)
/%	| 中缀 [mod](https://en.wikipedia.org/wiki/Modulo_operation) (如果输入为负，则结果为负)

### Monads

Jane Street 的库（ Core，Async，Base 等）在 [Monad_infix 模块](https://ocaml.janestreet.com/ocaml-core/latest/doc/base/Base/List/Monad_infix/index.html)下一致地定义了中缀运算符。

操作符 | 描述
------------ | -------------
>>=	| 中缀版本的 [bind](https://en.wikipedia.org/wiki/Monad_(functional_programming)#Overview)。打开 Async 将此设置为 [Deferred.bind](https://ocaml.janestreet.com/ocaml-core/latest/doc/async_kernel/Async_kernel/Deferred/index.html#val-bind)
>>\|	| 中缀版本的 map. 打开 Async 将此设置为 [Deferred.map](https://ocaml.janestreet.com/ocaml-core/latest/doc/async_kernel/Async_kernel/Deferred/index.html#val-map)
>>=? |	与 Or_error 混合的 bind。打开 Async 将此设置为 [Deferred.Or_error.bind](https://ocaml.janestreet.com/ocaml-core/latest/doc/async_kernel/Async_kernel__/Deferred_or_error/index.html#val-bind)
\>>\|? |	与 Or_error 混合的 map。打开异步将此设置为 [Deferred.Or_error.map](https://ocaml.janestreet.com/ocaml-core/latest/doc/async_kernel/Async_kernel__/Deferred_or_error/index.html#val-map)

假设您熟悉 monad，可以记录 map 和 bind ，如果您需要更多信息，可能会发现此 [StackOverflow 答案](https://stackoverflow.com/questions/29851449/what-is-the-use-of-monads-in-ocaml/29852213#29852213)很有用。

>>= 和 >>| 最常出现在 Async 中，但它们也可以与 Option，List，Result 等一起使用。

### Lwt

请参阅 [Lwt文档](http://ocsigen.org/lwt/3.1.0/api/Lwt)。

操作符 | 描述
------------ | -------------
>>=	| 中缀版本的 [bind]()
=<<	| 与反转的参数 bind
\>\|=	| 中缀版 map。与 Jane Street 代码中的 >>| 相同 
=\|<	| 与反转的参数 map

Lwt 没有 Async 的>>=? 或 >>|? 因为 Lwt.t 可以包含错误而没有单独的 Or_error 模块。

如果您需要有关 map 和 bind 的信息，请参阅上面的 Jane Street Monad 部分。