---
title: OCaml简介第6部分
description: OCaml简介第6部分
created: 2018/07/17
author: lanqy
---

## OCaml简介第6部分

译自：https://qiita.com/zenwerk/items/97d370d457008d8f01de

### 标记的参数

#### 〜标签名称：
* 你可以命名这个参数。
* 如果你给一个标签名称，你可以改变你喜欢的参数的顺序。

```ocaml
(* 使用带标签的参数定义函数 *)

# let rec range ~first: a  ~last: b = 
  if a > b then []
  else a :: range ~first: (a + 1) ~last: b;;
  
(* 函数类型上的标签类型 *)
val range : first:int -> last:int -> int list = <fun>

(* 指定标签名称函数应用程序 *)

# range ~first: 1 ~last: 10;;

- : int list = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] 

# range ~last: 10 ~first: 1;;

- : int list = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] 

(* 除非指定了标签名称，否则按标签名称定义应用 *) 

# range 1 10;;
- : int list = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]

# range 10 1;;
- : int list = []
```

* 〜hoge：○○ ○○ 是可选的

```ocaml
# let rec range ~first ~last = 
  if first > last then []
  else first :: range (first + 1) last;;
val range : first:int -> last:int -> int list = <fun>
```

#### 可选参数

?标签名称：（pattern =表达式）

与Python的家伙相同的功能。

```ocaml
(* 默认值1给出步骤值 *)

# let rec range ?(step = 1) a b = 
  if a > b then []
  else a :: range ~step (a + step) b;;
val range : ?step:int -> int -> int -> int list = <fun>

(* 函数应用 *)
# range 1 10;;

- : int list = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]

(* 因为它在调用时指定了标签 *)

# range 1 10 ~step:2;;
- : int list = [1; 3; 5; 7; 9]
# range 1 ~step:3 10;;
- : int list = [1; 4; 7; 10]

# range 2 1 10;;
Error: The function applied to this argument has type ?step:int -> int list
This argument cannot be applied without label
```
#### 关于可选参数的说明

##### 在选项参数后面准备一个无标签的参数

如果选项参数在函数的结尾，则它变成了不能省略的选项参数，所以没有任何意义。

```ocaml
(**
 * 我想定义一个函数，返回常量1 ....。
 * 如果选项参数在最后，我会收到警告
 *)
# let f ?(x=1) = x;;
Warning 16: this optional argument cannot be erased.
val f : ?x:int -> int = <fun>
(* 1函数返回我以为它返回一个函数，接收*选项参数返回正在currying
 *)
# f;;
- : ?x:int -> int = <fun>

(* 最后这么糟糕，不要添加无标签的参数 *)
# let f ?(x=1) () = x;;
val f : ?x:int -> unit -> int = <fun>
# f;;
- : ?x:int -> unit -> int = <fun>
# f();;
- : int = 1
```
#### 选项参数实体

可选参数是用 'a option 实现的

如果你没有指定默认值，并写入它，尝试执行，你会得到一个错误，如下所示

```ocaml
# let rec range ?step a b =
    if a > b then []
    else a :: range ~step (a+step) b;;
(* 'a option type error is occurring  *)
Error: This expression has type 'a option
       but an expression was expected of type 'a
       The type variable 'a occurs inside 'a option
```

因此，在这种情况下，None 加 'a Some 来模式匹配

```ocaml

(* 与选项类型匹配的模式 *)

# let rec range ?step a b =
    let s = match step with None -> 1 | Some s -> s in
    if a > b then [] else a :: range (a + s) b;;
val range : ?step:int -> int -> int -> int list = <fun>

# range 1 10;;
- : int list = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
```

#### 多态变种

##### `Constructor

一种机制，可以为多种变体类型使用通用的构造函数。

消除 “1构造函数<=> 1变体” 的限制。

```ocaml
# `Hoge;;
- : [> `Hoge ] = `Hoge
# `Hoge 2;;
- : [> `Hoge of int ] = `Hoge 2
# `Hoge `Fuga;;
- : [> `Hoge of [> `Fuga ] ] = `Hoge `Fuga
```

#### 函数返回多态变种

```ocaml
# let f b = if b then `Hoge else `Fuga;;
val f : bool -> [> `Fuga | `Hoge ] = <fun>
```
```ocaml
# let hoge = function
  | `Hoge -> "hoge"
  | `Fuga -> "fuga"
  | `Piyo -> "piyo";;
val hoge : [< `Fuga | `Hoge | `Piyo ] -> string = <fun>
```

#### 多态变种的类型方案

[> ...] , [<...] 是类型方案 (同 `a)

[> ..] 和 [< ..] 它被视为有限制的类型变量。

在 [> ...] 的情况下
[> 可以解释为 “包含多态变种” 或更高。

```ocaml
(* 
 * 可以接受任何东西的多相变体列表
 *)
# let  a : [>] list = [`Fuga; `Piyo];;
val a : [> `Fuga | `Piyo ] list = [`Fuga; `Piyo]

# a @ [`Asdf];;
- : [> `Asdf | `Fuga | `Piyo ] list = [`Fuga; `Piyo; `Asdf]
```

在 [<...] 中

[< 可以被解释为“在所包含的多相变体类型中”。

```ocaml
(* `Hoge,`within Fuga *)
# let f = function
  | `Hoge -> "hoge"
  | `Fuga -> "fuga";;
val f : [< `Fuga | `Hoge ] -> string = <fun>

(* `Hoge, type within Fuga *)
# type type_A = [`Hoge | `Fuga];;
type type_A = [ `Fuga | `Hoge ]
# let a:type_A = `Hoge;;
val a : type_A = `Hoge
# f a;;
- : string = "hoge"

(* `Hoge,`Fuga or more types *)
# type type_B = [`Hoge | `Fuga | `Piyo];;
type type_B = [ `Fuga | `Hoge | `Piyo ]

(* `I'm Hoge type_B is `Hoge, `Fuga or more types *)
# let b:type_B = `Hoge;;
val b : type_B = `Hoge
# f b;; (* type error　*)
Error: This expression has type type_B but an expression was expected of type
         [< `Fuga | `Hoge ]
       The second variant type does not allow tag(s) `Piyo
```

#### 多态变体类型的定义

[> ..] [<..] 无法使用的模式

type hoge = [`Hoge |`Fuga |...]

声明一个类型变量 [> ..] 和 [<..] 定义一个模式

type 'a hoge = [> `Hoge ...] as' a

```ocaml
# type hoge = [`Hoge | `Fuga];;
type hoge = [ `Fuga | `Hoge ]

# type 'a hoge = [> `Hoge] as 'a;;
type 'a hoge = 'a constraint 'a = [> `Hoge ]

# type 'a hoge = [< `Hoge] as 'a;;
type 'a hoge = 'a constraint 'a = [< `Hoge ]

(* 类型定义可以重用 *)
# type hoge' = [hoge | `Piyo];;
type hoge' = [ `Fuga | `Hoge | `Piyo ]
```

#### 递归定义多相变量

##### type 'a mylist = Nil | Cons of 'a * 'a mylist 我们将其定义为多相变量
* 首先按顺序定义它，并检查Valiant类型发生了什么

```ocaml
（*通过定义来查看价格类型的变化*）
# let l1 = `Nil;;
val l1 : [> `Nil ] = `Nil
# let l2 = `Cons(1, `Nil);;
val l2 : [> `Cons of int * [> `Nil ] ] = `Cons (1, `Nil)

（*你不能获得泛型类型定义*）
# let l3 = `Cons(2, `Cons(1, `Nil));;
val l3 : [> `Cons of int * [> `Cons of int * [> `Nil ] ] ] =
  `Cons (2, `Cons (1, `Nil))
```
* 如果你写一个执行任意列表处理的函数，你应该能够获得列表的通用术语类型定义...

```ocaml
(* 递归Variant类型的类型定义是...？ *)
# let rec length = function
  | `Nil -> 0
  | `Cons (a, l) -> 1 + length l;;
(*
* [...]因为'a已被赋予变体类型定义的别名
*这个别名'a出现在[...]中，原来是一个递归的定义
*'由b给出的类型变量是一个也可以看出哪个是固定的类型
*)

val length : ([< `Cons of 'b * 'a | `Nil ] as 'a) -> int = <fun>
```

```ocaml
(* 函数在列表中选择*的最大值 *)
# let rec max_list = function
  | `Cons(x, `Nil) -> x
  | `Cons(x, `Cons(y, l)) ->
    if x < y then max_list (`Cons(y, l)) else max_list (`Cons(x, l));;

(*
* [.. [..]] 是因为它的形式，可以看出长度大于或等于1
*)
val max_list : [ `Cons of 'a * ([< `Cons of 'a * 'b | `Nil ] as 'b) ] -> 'a = <fun>
```

#### 注意多相变异型

##### 注意 "&"

如果多态变体＆出来，它表明，它输入失败

例如，int＆float指示表示int和float，不能实现

那么，在这种情况下，您应该查看函数定义等

```ocaml
# let f = function `A x -> x+1 | `B -> 2;;
val f : [< `A of int | `B ] -> int = <fun>
# let g = function `A x -> int_of_float x+1 | `B -> 2;;
val g : [< `A of float | `B ] -> int = <fun>

(*
*返回不正确的类型[int＆float]
* hoge＆fuga是一个不可行的类型，所以下面的类型定义是真实的[<`B] - > Int
*)

# let f_or_g b = if b then f else g;;
val f_or_g : bool -> [< `A of int & float | `B ] -> int = <fun>
```

#### 使用点

练习多相变量的正确使用所必需的。

在很多情况下，应该使用正常的变体来完成。

#### 奖励：你有多态记录吗？

如果有多相变量，是否还有多态记录（如``hoge：int'）？ 我想，但没有这样的事情。

相反，OCaml对象可以提供等效的功能。

然而，对象没有模式匹配，所以说它用得不多。
