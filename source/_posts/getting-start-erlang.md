---
title: Erlang 入门
description: Erlang 入门
created: 2018/07/31
author: lanqy
---

# Erlang 入门

 译自：https://qiita.com/oskimura/items/87910c2e76cac075ee7f

## Erlang 入门

Erlang 是一种支持语言和 VM 并行处理的函数式语言。此外，Erlang 受到 Prolog 及其并行逻辑语言的强烈影响，后者是其继承者，其效果可见于 VM 的语法和结构。

### 安装

#### Windows

从 http://www.erlang.org/downloads 下载安装程序并安装它。

#### Mac

```text
brew install erlang
```

你可以安装

#### Linux

如果是 Debian

```text
apt install erlang
```

### 对话环境

#### Windows

```text
"Start" -> "All Programs" -> "Erlang OTP" -> "Erlang"
```

#### Mac & Linux

通过从终端输入来建立对话环境。我们来试试吧。

```text
$ erl
```

```erlang
$ erl
Erlang R16B03 (erts-5.10.4) [source] [64-bit] [async-threads:10] [kernel-poll:false]

Eshell V5.10.4  (abort with ^G)
1> 1+2+3.
6
2>
```

6 它将显示。这是 1 + 2 + 3. 计算公式的结果。Erlang 使用 . 代替 ;。

### 编译

用 c 命令编译 。

由于 Erlang 在名为 BEAM 的 VM 上运行，因此它被编译为在 VM 上运行的二进制文件。 

我们将此文件称为梁文件。

创建的文件是

```shell
$ erl - pz. - noshell - noinput - s module - name function - s init stop
```

您可以在终端上键入并运行。

但是，如果您想轻松运行它，它会更方便使用。

### escript

在escript中，Erlang的程序可以像Perl脚本一样执行。

```erlang
#!/usr/bin/env escript
%% -*- erlang -*-
%%! -smp enable -sname factorial -mnesia debug verbose
main([String]) ->
    try
        N = list_to_integer(String),
        F = fac(N),
        io:format("factorial ~w = ~w\n", [N,F])
    catch
        _:_ ->
            usage()
    end;
main(_) ->
    usage().
usage() ->
    io:format("usage: factorial integer\n"),
    halt(1).

fac(0) -> 1;
fac(N) -> N * fac(N - 1).
```

```text
$ chmod u+x factorial
```

```text
$ ./factorial 5
factorial 5 = 120
```


```text
$ ./factorial
usage: factorial integer
```



```text
$ ./factorial
usage: factorial integer
```

```text
$ ./factorial five
usage: factorial integer
```

```text
$ escript factorial 5
```

等等。

有关详细信息，请参阅 [escript manual](http://erlang.org/doc/man/escript.html)。

### 整数和实数

```erlang
4> 2#0101.
5
5> 16#deadbeaf.
3735928495
```

作为 # 你可以通过在前面放置一个数字来将 N-ary 符号提高到 36 位小数。

Erlang 是一个多精度整数。

IEEE 754格式的浮点数在 1.0e−323 到 1.0e+308 范围内

### 文字列

字符前面是 ASCII 字符 $ 。用双引号括起字符串。这是字母列表的糖涂层语法。列表将描述以下内容。

它需要以后面描述的二进制表示。

```erlang
7> $a.
97
8> "a".
"a"
9> [$a,$b].
"ab"
```

### 变量

Erlang 变量名以大写字母开头。

```erlang
10> X = 1.
1
11> X.
1
```

这是变量 X 在 1 中的值，通过代入 X 的值。

我们将用后面描述的模式匹配来解释赋值。

### 操作符

#### 算术运算符

- `+`
加
- `*`
乘
- `-`
减
- `/`
除
- `div`
整数除法
- `rem`
整数除法的余数

它基本上与C语言和Java相同。div和rem分别是除数时的商和余数。

#### 比较运算符

- `>`
大于
- `<`
小于
- `>=`
大于等于
- `=<`
小于等于
- `==`
等于
- `/=`
不等于
- `=:=`
相等（带类型检查）
- `=/=`
不等于（带类型检查）

#### 算术运算符

- `not`
否定
- `and`
和
- `andalso`
和顺便（短路）
- `or`
或
- `orelse`
或者其他（短路）
- `xor`
独家理论和

### 条件分支

#### if

```erlang
if
    <conditional expression> -> <form 1>;
    <Conditional> -> <Formula 2>
end
```

因为 Erlang 有一个模式匹配，所以 if 使用不多。

对于条件分支，我们经常使用模式匹配，如稍后所述。

### 原子

原子代表一个标识符。小写字母如果包含以字母或符号开头的字母数字字符，则必须将它们括在反引号中。

```erlang
1> abc.
abc
```

当然，您可以将其分配给变量。

```erlang
2> X = abc.
abc
3> X.
abc
```

Atom 经常用于 Erlang 编程，因为它便于后面描述的元组和消息的模式匹配。

### 输出

在 Erlang 中，输出到控制台 io:format 是使用通常调用的函数完成的。

#### io:format

Erlang 的格式化输出是通过 io:format完成。printf它对应于C语言，但格式化的输出格式类似于 Common Lisp 或者来自 C 的 Common Lisp。

```erlang
io:format("~p~n", [Term]).
```

- `~p` 漂亮打印
- `~s` 文本列
- `~n` 更改行数
- `~d` 整数（十进制数）

Erlang 使用列表，因为没有像 C 语言这样的可变参数。

### 模式匹配

#### 案例部分

```erlang
case <type> of
    <表达式1> -> <式1>;
    <表达式2> -> <式2>
end
```

case 子句是最简单的模式匹配形式。

```erlang
2> A = 3.
3
3> case A of 1 -> a; 2 -> b; 3 -> c end.
c
```

将返回 c ，它也可以像这样的C语言 switch 句子。

如果我们从此表达式中排除最后一次模式匹配的处理，会发生什么？

```erlang
4> case A of 1 -> a; 2 -> b end.
** 异常错误：没有 case 子句匹配3
5>
```

像这样发生异常。这是一个例外，意味着没有条件匹配模式匹配。

#### 通配符

然后如何更改它如下？它是通配符，并且在每种情况下都匹配。

```erlang
5> case A of 1 -> a; 2 -> b; _ -> c end.
c
```

#### 单赋值

```erlang
6> X = 1.
```
实际上，替换实际上是一种模式匹配。

`=` 操作符在左侧表达式的右侧执行模式匹配。如果左边的变量尚未绑定，则计算右边表达式的值将被绑定。`=` 当然，如果表达式左侧的模式匹配失败，则会引发异常。

在这个例子中，因为它只是一个像数字类型的简单类型，我认为它的用处很难理解。通过结合后面描述的列表，记录，元组等，您可以以非常多样化的方式使用它。

模式匹配是 Erlang 的核心功能。

### 函数

```erlang
<function name> (参数...) -> 
函数主体<body>.
```

例如，下面是一个函数，用于递增指定为参数的变量。函数定义是这样的。

```erlang
inc(X) -> X + 1.
```

### 函数和模式匹配

Erlang 还可以对函数参数执行模式匹配。

```erlang
signal(red) -> stop;
signal(blue) -> do;
signal(yellow) -> carefull;
signal(_) -> error.
```

#### 递归

由于 Erlang 没有循环控制语法(如 C 语言中的语句)，因此循环处理是递归进行的。

我将编写一个函数来查找阶乘。

```erlang
pow(1) -> 1;
pow(X) -> X * pow(X - 1).
```

#### 尾递归

可以通过使用尾递归的形式来优化循环。

```erlang
pow_tail(1, Ret) -> Ret;
pow_tail(X, Ret) -> pow_tail(X - 1, X * Ret).
```

简单地说，尾递归形式不计算递归函数的返回值，因此不需要每次都将函数的返回点堆栈在堆栈帧上，因此可以像循环一样在内部处理它

有兴趣的人请自己研究。

### 记录

这是一个所谓的结构。

```erlang
-record(<记录名>, {<元素名1>, <元素名2>, ...}).
```

还提供了一种称为定义记录的函数的 rd 语法糖。

```erlang
rd(item, {id, name, price}).
```

```erlang
7> rd(player, {name, hp}).
player
```

#### 初始化

```erlang
# <记录名> {<元素名1> = 初始值1, <元素名1> = 初始值2, ...}
```

可以以这种形式进行记录初始化。

```erlang
9> P = #player{name="hoge", hp=3}.
```

#### 引用

```erlang
Variable#<record name>.<Element>
```

上面提到它。

```erlang
10>P#player.name
"hoge"
```

#### 更改

```erlang
Variable#<record name>{<element> = "hage"}.
```

通过这样做，它返回更改记录元素的记录。

```erlang
11> P#player{name="hage"}.
#player{name = "hage", hp = 2}
```

#### 记录模式匹配

在左侧

```erlang
# <Record name> {<element name 1> = value, <element name 2> = variable name, ..}
```

你可以用搭配的形式进行模式匹配。

```erlang
2> P = #item{id = 1, name="test", price=100}.
3> #item{id=1} = P.
#item{id = 1, name = "test", price = 100}.
4> #item{id = 1, name = Name} = P.
#item{id = 1, name = "test", price = 100}
5> Name.
"test"
```

### 列表

```erlang
[<element 1>, <element 2>...]
```

```erlang
1> [1,2,3].
[1,2,3]
```

这与 Lisp 等列表相同。内部缺点，最后一个是递归结构，单元格类似于Lisp nil。这将在后面描述。

#### 列表处理

该列表可以通过模式匹配来处理。

接下来，我们以一个计算列表长度的函数为例。

```erlang
len([]) ->
    0;
len([Head|Tail]) ->
    1 + len(Tail).
```

[] 匹配空列表。

[Head|Tail] Head 标识列表头, 例如, [1,2,3] Head 匹配 1， Tail 匹配 [2,3].

处理列表的函数是组织成称为标准库的模块的列表。

### 元组

```erlang
{<Element 1>, <element 2>...}
```

```erlang
1> {1,2,3}.
{1,2,3}
```

在 Erlang 中经常使用元组而不是 Lisp 的 S 表达式。

### 二进制

```erlang
<< <Value 1>: <Size 1> / <Type>, <Value 2>: <Size 2> / <Type> ... >>
```
您可以为每个尺寸指定一个大小的值。如果省略大小规范，则默认为 8 位。按此大小的大小划分的块称为段。

可以指定类型，类型，代码，字节序，多种类型 - 可以连接。 可能的类型如下所列。

类型

- `integer` 整型
- `float` 浮点数
- `binary` 二进制类型
- `bytes` 字节型
- `bitstring` 位串
- `bits` 位类型
- `utf8` UTF-8 位字符串
- `utf16` UTF16 的位串
- `utf32` UTF32 位字符串

符号

- `signed` 有符号的
- `unsigned` 无符号

尾数法

- `big` 大端字节序
- `little` 小端字节
- `native` CPU 原生端
- `unit` 段的大小

#### 二元运算符

- `bsl` 左移位
- `bsr` 右移位
- `band` 比特理论
- `bor` 比特理论和
- `bxor` 比特独家论证和
- `bnot` 比特论证否定

#### 二进制模式匹配

作为二进制模式匹配的示例，我将展示 RGB 颜色的示例。

```erlang
1> Color = <<16#FF00FF:(8*3)>>.
<<255,0,255>>
```

`,` 您可以将每个 8 位 RGB 定义为相隔。

```erlang
2> <<R,G,B>> = Color.
<<255,0,255>>
```

这样，它可以通过模式匹配每 8 位进行分解。

如果只想要第一个 R ，可以使用通配符通过以下方式编写来获得它。

```erlang
2> <<R:8,_/binary>> = Color.
<<255,0,255>>
3> R.
255
```

顺便说一句，在 Erlang 中处理像日语这样的多字节字符

```erlang
io:format("~ts~n",[<<"お"/utf8>>]).
```

必须以二进制格式指定UTF-8，如下所示 有关详细信息，请参阅 [Erlang 文档](http://erlang.org/doc/programming_examples/bit_syntax.html)。

### 理解符号

有两种类型的理解符号：列表理解符号和二进制理解符号。

最初，理解符号是表示一组与集合论中的条件 P 一致的元素的符号。

试图表达这一点的符号被介绍给米兰达。这种表示法后来被合并到 Haskell 和 Python 中。

#### 列表理解符号

 进入 `erl`

 ```erlang
Erlang R16B03 (erts-5.10.4) [source] [64-bit] [async-threads:10] [kernel-poll:false]

Eshell V5.10.4  (abort with ^G)
1> [ X * 2 || X <- [1,2,3,4,5], X < 4 ].
[2,4,6]
 ```

 这个 X < - [1,2,3,4,5] 产生一个列表。

 接下来，X < 4 它匹配条件 1 2 3 但被过滤，

 最后列表 X * 2 由 1 * 2 , 2 * 2 , 3 * 2 创建公式。

 概括如下：

 ```erlang
[Expression || pattern <- list, condition]
 ```

 二元理解是理解符号的二进制版本。

 ```erlang
<< expression || pattern <= binary, condition >>
 ```

 ### 并行

 Erlang 中的并行处理是通过轻量级进程（进程）和消息实现的。

 Erlang 的过程类似于 OS 过程，但它由 Erlang 的 VM 执行，因此操作系统内的上下文切换没有完成。

 Erlang 的进程通过消息队列进行通信，而不共享内存区域。

 切换速度很快，因为 Erlang 的进程不涉及复杂的上下文切换，例如临时保存寄存器和切换到内核模式。

 #### Actor 模型

 我将简要介绍作为 Erlang 并行模型的 Actor 模型。

 一个叫做 actor 的对象，能够发送和接收消息 通过异步发送和接收消息并行执行计算处理的计算模型。在 Erlang 中，该过程相当于一个 actor ，我们正在相互发送和接收消息。 除了 Erlang 之外，这个 actor 模型的概念被采用为 Scala 的框架等

#### spawn

spawn 通过以下方式调用它来生成轻量级进程。

```erlang
Pid = spawn (<module name>, <function name>, <list of arguments>)
```

spawn 从参数函数创建一个进程，并为每个进程返回一个唯一的数值，称为进程 ID。

进程 ID 和进程是链接的，此进程 ID 用于在进程之间发送和接收消息。

#### 发送消息

要发送消息，请将消息以消息的形式发送到您想要发送的流程的进程 ID ，如下所示。

```erlang
Pid! Message
```

#### 接收消息

通过以下方式接收消息并通过模式匹配执行与消息相对应的处理是常见的。

```erlang
receive
    <Pattern 1> ->
        <Process 1>;
    <Pattern 2> ->
        <Process 2>;
    
    .........

    <Pattern N> ->
        <Process N>
    after n ->
        < Timeout processing>
end
```

after 将进程在 n 毫秒后超时。

#### 进程注册

spawn 对程序员来说每次都要保存变量生成的进程 ID 很麻烦。特别是当多个程序创建程序来发送消息时，管理变得非常麻烦。

Erlang 有一种通过指定原子而不是进程 ID 来发送消息的方法。

这是流程注册。 register 使用该函数注册进程。

由于它超出了本文档的范围，因此不对其进行详细描述，但是进程注册功能通常用于分布式环境中。

```erlang
register(Atom,Pid)
```

通过 Atom ! the message 您可以在进程中发送已经以所述形式向 atom 注册的消息。

### spawn_link

spawn_link 基本上是 spawn 相同，但是当发生异常时行为会有所不同。 该进程的父进程会在生成的子进程中创建一个异常。 spawn，父进程不会收到异常。


### 异常

我这样写异常。

```erlang
try <evaluated expression> of
    <pattern 1> guard 1 -> <process 1>;
    <pattern 2> guard 2 -> <process 2>;
catch
    <exception type 1>: <pattern 1> guard 1 -> <Exception processing 1>;
    <exception type 2>: <pattern 2> guard 2 -> <Exception processing 2>
after
    <Processing executed even if an exception occurs but not occurring>
end
```

try ... of 当在封闭的表达式中出现异常时，将在其后执行一个捕捉模式匹配，并在执行异常处理后执行处理。

以下是引发异常的函数。

#### 抛出异常

您可以按如下方式引发异常。

```erlang
throw(<表达式>)
```

#### 退出

它引发了如下异常并终止了该过程。

```erlang
exit(<表达式>)
```
#### 错误

我们将提出一个严重的例外，并终止如下过程。

错误主要用于 Eralng 的 VM 生成的运行时错误。

```erlang
error(<表达式>)
```

### 模块

当以描述源的文件为单位进行编译时，模块是最小的单元。除非属于某个模块，否则无法编译源代码。

要创建模块，您需要在.erl 文件中描述模块属性，例如模块声明 。

#### 模块属性

```erlang
-module(<module name>).
```

#### 导出声明

声明导出声明函数。请连接 函数名称的参数个数如下 `/`。您可以按如下方式导出模块创建的所有功能。

```erlang
-export([<函数名1>/<参数1>>, <函数名2>/<参数2>>])

-export_all
```

#### 编译选项声明

为编译器指定[此选项](http://erlang.org/doc/man/compile.html)。

#### 宏定义

宏可以定义如下。

```erlang
-define{<macro>，<expression>}
```

#### 记录声明

记录可以声明如下。

```erlang
-record(<记录名>, {<元素名1>, <元素名2>,...}).
```

#### include

头文件可以包括如下。

```erlang
-include("<header file>").
```

头文件扩展名为.hrl。
