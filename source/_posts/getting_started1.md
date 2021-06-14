---
title: OCaml简介第1部分
description: OCaml简介第1部分
created: 2018/07/12
author: lanqy
---

# OCaml 简介第 1 部分

译自 : https://qiita.com/zenwerk/items/3bdf7eef6b7511e11b2c

## 函数式语言

函数式语言 => 执行程序 = 执行表达式

执行表达式 => 计算表达式（eval）以获得值

![函数式语言原理图](/images/68747470733a2f2f71696974612d696d6167652d73746f72652e73332e616d617a6f6e6177732e636f6d2f302f343432382f65373633373633322d393262342d623736342d623063652d6465366161653933396161342e706e67.png)

## 基本类型

### 整型（int）

#### 字首

-   二进制 0b
-   八进制 Oo
-   十六进制 0x

```ocaml
# 351;;
- : int = 351
# 12345;;
- : int = 12345
# 0o523;;
- : int = 339
# 0xffff;;
- : int = 65535
```

### 浮点数（float）

```ocaml
# 3.141;;
- : float = 3.1415
# 1.04e10;;
- : float = 10400000000.
# 25e-15;;
- : float = 2.5e-14
```

### 字符（char）

-   将其用单引号括起来，用单字节字母数字字符（ascii 字符）

```ocaml
# 'a';;
- : char = 'a'
# '\101';;
- : char = 'e'
# '\n';;
- : char = '\n'
```

### 字符串（string）

-   用双引号括起来。
-   一个字符串，一个字符可以用[下标]获取。

```ocaml
# "string";;
- : string = "string"
# "string".[0];;
- : char = 's'
```

### 类型转换

-   从 X 类型转换为 Y 类型的函数的命名规则为 Y_of_X。

```ocaml
# float_of_int 5;;
- : float = 5.
# int_of_float 5.;;
- : int = 5
- # string_of_int 123;;
- : string = "123"
# int_of_string "123";;
- : int = 123
```

### 元组

-   (类型名 \* 类型名...)

```ocaml
# (1, 2);;
- : int * int = (1, 2)
# ('a', 1, "str", 4.3);;
- : char * int * string * float = ('a', 1, "str", 4.3)
# ((1, 2), ('a', "str"));;
- : (int * int) * (char * string) = ((1, 2), ('a', "str"))
```

### 定义变量

#### let 定义（let defenition）

-   let 变量名称 = 表达式

```ocaml
# let hoge = 1;;
val hoge : int = 1
```

#### 可以同时定义

-   let 变量名称 = 表达式 1 and 表达式 2

```ocaml
# let a = 1 and b = 2;;
val a : int = 1
val b : int = 2
```

### 定义函数

#### let 函数名 参数 = 表达式

```ocaml
# let twice s = s ^ s;;
val twice : string -> string = <fun>
```

包含参数（）是可选的。

#### let 表达式（let expression）

-   与 let 定义不同。
-   用于在函数中定义临时变量（局部变量）的表达式。
-   let 变量名 = 表达式 1 in 表达式 2

```ocaml
# let four_times s =
  let twice = s ^ s in
  twice ^ twice;;
val four_times : string -> string = <fun>
```

### 递归定义

#### 用 let rec 定义。

-   使用 rec 可以引用函数定义中定义的函数名称。

阶乘的例子:

```ocaml
# let rec fact x =
    if x <= 1 then 1 else x * fact (x - 1);;
val fact : int -> int = <fun>
# fact 5;;
- : int = 120
```

### 相互递归

-   两个或多个函数相互调用的样式的递归定义。

-   let rec 函数名称 1 参数 1 = 表达式 1 and 函数名称 2 参数 2 = 表达式 2 and ...

```ocaml
# let rec even n =
    match n with
    | 0 -> true
    | x -> odd (x-1)
  and odd n =
    match n with
    | 0 -> false
    | x -> even (x-1);;
val even : int -> bool = <fun>
val odd : int -> bool = <fun>

# even 10;;
- : bool = true
# even 3;;
- : bool = false
# odd 3;;
- : bool = true
# odd 10;;
- : bool = false
```

### 匿名函数

#### fun 参数名 = 表达式

-   let f 参数 = 表达式 是 let f = fun 参数 = 表达式 的语法糖
-   因为 fun 是尽可能地认识到它是一个函数定义，所以最好在必要时用（）分割它。

### 高阶函数

定义一个函数作为参数

```ocaml
# let twice f x = f (f x);;
val twice : ('a -> 'a) -> 'a -> 'a = <fun>
# twice (fun x -> x * x) 3;;
- : int = 81

# let fourth x =
    let square y = y * y in
    twice square x;;
val fourth : int -> int = <fun>
# fourth 3;;
- : int = 81
```

### 柯里化函数

```ocaml
# let concat_curry s1 = fun s2 -> s1 ^ s2 ^ s1;;
val concat_curry : string -> string -> string = <fun>
# concat_curry "a";;  (* 部分適用 *)
- : string -> string = <fun>
# (concat_curry "a") "b";;
- : string = "aba"
```

### 柯里化语法糖

一下这个定义

```ocaml
let concat_curry s1 s2 = s1 ^ s2 ^ s1;;
```

与以下相同

```ocaml
let concat_curry s1 = fun s2 -> s1 ^ s2 ^ s1;;
```

也就是说，当参数排序的时候，以下代码

```ocaml
# let fuga x y z = x + y + z;;
val fuga : int -> int -> int -> int = <fun>
```

实际上可以展开为以下代码

```ocaml
# let hoge x = fun y -> fun z -> x + y + z;;
val hoge : int -> int -> int -> int = <fun>
```

函数是左结合，所以可以扩展如下：

```ocaml
f x y z => (((f x) y) z)
```

函数类型构造函数是右结合的

```ocaml
int - > int - > int - > int = <fun>
解释为
int - >（int - >（int - > int））= <fun>
```

### 运算符定义

#### 中缀运算符被定义为柯里化函数。

```ocaml
# (+);;
- : int -> int -> int = <fun>
# (+) 1 2;;
- : int = 3
```

#### 自己定义操作符时

#### 前缀运算符

-   1、定义为一个参数函数

#### 中缀运算符

-   2、定义为参数（currying）函数
-   请注意，可以用来定义运算符的字符类型是有限的。

### 类型推断

-   比较运算符（=等）可以使用多种类型，如整数类型和实数类型=>多态（多态）。
-   具有多态性的类型系统被称为多态类型系统。

#### 类型方案·参数多态性

-   一种表示一种类型的外观模式的变量。
-   它表示为'a 或'b。

```ocaml
# let twice f x = f (f x);;
val twice : ('a -> 'a) -> 'a -> 'a = <fun>
# let first (x, y) = x;;
val first : 'a * 'b -> 'a = <fun>
```

#### 类型变量的泛化

-   将类型变量（如'\_a）（类型方案，如'a'）泛化的条件之一如下。
-   如果定义右侧的表达式是一个值，则可以概括类型变量
-   作为值处理的事物=>不需要计算的表达式
-   作为一个例子
-   函数声明
-   常量
-   需要计算的表达式不被视为值，因为它们是有价值的。

```ocaml
（*定义身份函数*）
# let id x = x ;;
val id：'a - >'a = <fun>

（*'_a不是一个类型方案
    因为id是一个值，一些实际的参数是必需的，计算是必要的。
*）
# let id'= id id ;;
val id'：'_a - >'_a = <fun>

（*
    这种类型的方案
    通过添加参数x，id'变成了函数定义。
*）
# let id'x = id id x ;;
val id'：'a - >'a = <fun>
```
