---
title: OCaml简介第4部分
description: OCaml简介第4部分
created: 2018/07/15
author: lanqy
---

## OCaml简介第4部分

译自：https://qiita.com/zenwerk/items/244b84bee48bf61d2a51

### 模块

它是程序的一部分，但在OCaml中被称为结构。

所有OCaml库都作为模块（结构）提供。

#### 文件名称即是模块名称

文件名 example.ml => 模块名称为 Example

#### 标准模块

OCaml 内置模块

```ocaml
(* 列表 *)
# List.length [1; 2; 3];;
- : int = 3
# let q = Queue.create ();;
val q : '_a Queue.t = <abstr>

(* 队列 *)
# Queue.add "first" q;;
- : unit = ()
# Queue.take q;;
- : string = "first"
# Queue.take q;;
Exception: Queue.Empty.

(* 数组 *)
# Array.make 3 'a';;
- : char array = [|'a'; 'a'; 'a'|]

(* 标准输出 *)
# Printf.printf "%d, %x, %s\n" 10 255 "hoge";;
10, ff, hoge
- : unit = ()
```
#### open 模块

通过 open 打开模块可以省略模块名称

与 python 中 from hoge import * 类似

由于打开到当前源的模块的名称空间已扩展，因此只有在不困惑时才能打开它。

由于容器名称经常与函数名称重叠，因此不打开它们是正常的。

```ocaml
(* 打开模块 *)
# open List;;
# length [1; 2; 3];;
- : int = 3

(* 覆盖函数名称 *)
# let length () = "overload!";;
val length : unit -> string = <fun>
# length ();;
- : string = "overload!"

(* 您可以通过指定模块名称来调用它 *)
# List.length [1; 2; 3];;
- : int = 3
```
顺便说一下，OCaml有一个名为Pervasives的模块，在启动时打开。

像abs和open_in这样的函数属于这个模块。

#### 模块定义

模块名称以大写字母开头

##### module 模块名 = struct 各种定义... end

```ocaml
(* 模块定义 *)
# module Hello = struct
    let message = "Hello"
    let hello () = print_endline message
  end;;
module Hello : sig val message : string val hello : unit -> unit end
(* 调用 *)
# Hello.hello ();;
Hello
- : unit = ()
```

#### 签名

##### 签名
* sig ... end 周围的部分
* 整个模块的类型（类似）
* 表示模块的I / F（可以定义可访问模块的元素）

#### mli文件

Hoge 模块的签名可以在 hoge.mli 文件中定义。

在文件中写一个签名

##### 签名定义
* 定义 => module type 签名名 = sig ... end
* 应用 => module 模块名 : 签名名 = 模块名或 struct ... end

```ocaml
(* message 元素可访问 *)
# Hello.message;;
- : string = "Hello"

(* message 定义未定义的签名 *)
# module type Hello_Sig =
    sig
      val hello: unit -> unit
    end;;
module type Hello_Sig = sig val hello : unit -> unit end

(* 给一个模块签名 *)
# module Hello2 : Hello_Sig = Hello;;
module Hello2 : Hello_Sig

(* 因为它不是签名, message 元素不可访问 *)
# Hello2.hello ();;
Hello
- : unit = ()
# Hello2.message;;
Error: Unbound value Hello2.message

(* 直接定义模块 *)
# module Hello3 : Hello_Sig = struct
    let hello () = print_endline "Hello3!"
  end;;
module Hello3 : Hello_Sig
# Hello3.hello ();;
Hello3!
- : unit = ()
# Hello3.message;;
Error: Unbound value Hello3.message
```
#### 抽象数据类型

在签名定义中，省略 = ...用于类型定义（typ t = ...）

您可以隐藏定义的详细信息。

通过隐藏类型信息和实现，可以将该类型的操作限制为通过模块进行的操作。

防止意外操作。

```ocaml
(* 签名定义 *)
# module type AbstTypeSig = sig
    type t (* 抽象数据类型 *)
    val get_t : int -> t
    val print : t -> unit
  end;;
module type AbstTypeSig =
  sig type t val get_t : int -> t val print : t -> unit end

(* 模块定义 *)
# module AbstTypeInt : AbstTypeSig = struct
    type t = int
    let get_t i = i
    let print t = print_int t
  end;;
module AbstTypeInt : AbstTypeSig

(* 如果返回值是一个抽象数据类型 <abstr> *)
# let t = AbstTypeInt.get_t 0;;
val t : AbstTypeInt.t = <abstr>
# AbstTypeInt.print t;;
0- : unit = ()

(* 
  抽象数据类型不能在外部处理
  AbstTypeInt.t 是一个真正的int，但是因为它隐藏着一个抽象的数据类型
  print_int 即使是作为参数引用
*)
# let () = print_int t;;
Error: This expression has type AbstTypeInt.t
       but an expression was expected of type int
```

### Functor

通过应用参数动态生成参数的函数。

消除了多次定义不同模块的麻烦。

#### 使用 Functor

##### Functor应用程序 => Functor名称（模块化）

##### ※模块化功能应用程序本身

#### 处理集合 Set 模块示例

标准模块的 Set 和 Queue 使用 functor

对于要处理的集合的元素，定义以下模块
* 定义集合元素的模块
* 一个类型 t 表示一个集合的元素
* 比较元素类型t的大小的函数：compare: t -> t -> int

将上述“元素类型模块”应用于 functor ，生成“该类型为元素的模块”。

```ocaml
(* functor 应用 *)
# module IntSet = Set.Make (struct
    type t = int
    let compare i j = i - j
  end);;
module IntSet :
  sig
    type elt = int (* 元素我想作为元素类型对待 elt = int *)
    type t         (* 代表一个集合的类型是 IntSet.t 是一种抽象数据类型 *)
    val empty : t
    val is_empty : t -> bool
    val mem : elt -> t -> bool
    val add : elt -> t -> t
    val singleton : elt -> t
    val remove : elt -> t -> t
    val union : t -> t -> t
    val inter : t -> t -> t
    val diff : t -> t -> t
    val compare : t -> t -> int
    val equal : t -> t -> bool
    val subset : t -> t -> bool
    val iter : (elt -> unit) -> t -> unit
    val fold : (elt -> 'a -> 'a) -> t -> 'a -> 'a
    val for_all : (elt -> bool) -> t -> bool
    val exists : (elt -> bool) -> t -> bool
    val filter : (elt -> bool) -> t -> t
    val partition : (elt -> bool) -> t -> t * t
    val cardinal : t -> int
    val elements : t -> elt list
    val min_elt : t -> elt
    val max_elt : t -> elt
    val choose : t -> elt
    val split : elt -> t -> t * bool * t
    val find : elt -> t -> elt
    val of_list : elt list -> t
  end

(* 使用由 functor 生成的模块 *)
# open IntSet;;
# let s1 = add 2 (add 1 empty)
  and s2 = add 1 (add 3 empty);;
val s1 : IntSet.t = <abstr>
val s2 : IntSet.t = <abstr>
# mem 1 s1;;
- : bool = true
```

#### Functor 的定义

##### module functor名称（参数名称：签名表达式）= 模块化表达式

以下糖衣语法

##### module functor名称 = functor（参数名称：签名表达式） -> 模块化表达式

定义 Set.Make 的一个简单 functor 的例子

```ocaml
(* 签名定义 *)
module type ELEMENT = sig
  type t
  val compare: t -> t -> int
end

(* functor 定义 *)
module MakeSet (Element : ELEMENT) =
  struct
    type elt = Element.t
    type t = elt list

    let empty = []

    let mem x set = List.exists (fun y -> Element.compare x y = 0) set

    let rec add elt = function
    | [] -> [elt]
    | (x :: rest as s) ->
        match Element.compare elt x with
        | 0 -> s
        | r when r < 0 -> elt :: s
        | _ -> x :: (add elt rest)

    let rec elements s = s
  end;;
```

#### 依赖类型
* 上述 functor 的返回值的签名如下
* Functor（Element：ELEMENT） -> 转换的描述在 sig ... end 中查看
* type elt = Element.t 被写入正式参数的值包含在返回类型中
* 换句话说，返回值的类型根据给定的参数值（！= Type）而变化，
* 这被称为依赖类型

```ocaml
(* 返回顶层的值 *)
module type ELEMENT = sig type t val compare : t -> t -> int end
module MakeSet :
  functor (Element : ELEMENT) ->
    sig
      type elt = Element.t
      type t = elt list
      val empty : 'a list
      val mem : Element.t -> Element.t list -> bool
      val add : Element.t -> Element.t list -> Element.t list
      val elements : 'a -> 'a
    end
```

#### functor 的信息隐藏

像普通模块一样，您可以限制函数返回值的签名并隐藏内部实现。

##### module functor名称（参数名称：输入签名表达式）：签名表达式返回 = 模块表达式

通过指定要返回的签名表达式可以隐藏信息

```ocaml
module MakeSet (Element : ELEMENT) :
  (* 签名表达式返回 *)
  sig
    type elt = Element.t
    type t (* 抽象数据类型 *)
    val mem : elt -> t -> bool
    val add : elt -> t -> t
    val elements : t -> elt list
  end
  =
  (* 模块化 *)
  struct
    type elt = Element.t
    type t = elt list

    let empty = []

    let mem x set = List.exists (fun y -> Element.compare x y = 0) set

    let rec add elt = function
    | [] -> [elt]
    | (x :: rest as s) ->
        match Element.compare elt x with
        | 0 -> s
        | r when r < 0 -> elt :: s
        | _ -> x :: (add elt rest)

    let rec elements s = s
  end;;

(* functor 表达式的返回值 *)
module MakeSet :
  functor (Element : ELEMENT) ->
    sig
      type elt = Element.t
      type t
      val empty : t
      val mem : elt -> t -> bool
      val add : elt -> t -> t
      val elements : t -> elt list
    end
```

比较上述函数应用方程的返回值签名公式
* before
```ocaml
# module StringSet = MakeSet(String);;
module StringSet :
  sig
    type elt = String.t
    type t = elt list
    val empty : 'a list
    val mem : String.t -> String.t list -> bool
    val add : String.t -> String.t list -> String.t list
    val elements : 'a -> 'a
  end
```
* After

```ocaml
# module IntSet = MakeSet(struct
    type t = int
    let compare i j = i - j
  end);;
module IntSet :
  sig
    type elt = int
    type t (* 抽象数据类型 *)
    val empty : t
    val mem : elt -> t -> bool
    val add : elt -> t -> t
    val elements : t -> elt list
  end

# Open IntSet;;
# let s1 = add 1 (add 2 empty)
  and s2 = add 3 (add 4 empty);;
val s1 : IntSet.t = <abstr> (* 抽象数据类型 *)
val s2 : IntSet.t = <abstr>

(* 当它是一个字符串 *)
# module StringSet = MakeSet(String);;
module StringSet :
  sig
    type elt = String.t
    type t = MakeSet(String).t (* 有隐藏的实现吗？ *)
    val empty : t
    val mem : elt -> t -> bool
    val add : elt -> t -> t
    val elements : t -> elt list
  end

# open StringSet;;
# let s1 = add "a" (add "b" empty)
  and s2 = add "c" (add "d" empty);;
val s1 : StringSet.t = <abstr> (* 这是一个抽象的数据类型 *)
val s2 : StringSet.t = <abstr>
```

#### 签名的分解和定义（ with type ）

签名表达式 with type 类型名 = 类型定义 and type ...

不要指定你希望 functor 直接用 sig ... end 返回的签名定义，而是要给它定义名称。

type elt = Element.t 部分

签名不能被定义为别名，因为它取决于伪参数名称

```ocaml
module MakeSet (Element : ELEMENT) :
  (* 签名表达式返回 *)
  sig
    type elt = Element.t
    type t (* 抽象数据类型 *)
  end
  =
  struct ... end;;
```

#### with type 语法使用

```ocaml
(* with type 类型定义替换 elt *)
# module type ElementWithType =
    sig
      type elt
      type t
      val empty : t
      val mem : elt -> t -> bool
      val add : elt -> t -> t
      val elements : t -> elt list
    end;;

(* with type 定义 *)
# module type E2 = ElementWithType with type elt = int;;
module type E2 =
  sig
    type elt = int
    type t
    val empty : t
    val mem : elt -> t -> bool
    val add : elt -> t -> t
    val elements : t -> elt list
  end
```

### 批处理编译器和拆分编译

编译器通过file => 批量编译器导出

以文件为单位创建一个目标文件Link => 分割编译

ocaml 的批处理编译器 => ocamlc（bytecode），ocamlopt（本地代码）

#### 统一编译

```ocaml
$ ocamlc -o output.name src1.ml src2.ml ...
```

#### 拆分编译

* 需要安排最后一个链接的 cmo 文件，以便名称解析成为可能
* 非标准库
* 链接时有必要明确指出文件名
* nums.cma 等
* -c 选项输出目标文件而不链接
* -I 允许您指定包含 cmi，cmo 的目录

```ocaml
$ ocamlc -c mod1.ml
$ ocamlc -c mod2.ml
$ ocamlc -o output.name nums.cma mod1.cmo mod2.cmo
```

#### mli文件 

mli 文件描述了相应的 .ml 签名

在编译时使用 ocamlc

用 ocamlc 编译 mli 文件

用 ocamlc -c编译ml文件

链接 cmo 与 ocamlc

```ocaml
$ ocamlc mod.mli  # -c 没有选项要求
$ ocamlc -c mod.ml
$ ocamlc -c mod2.ml
$ ocamlc -o a.out mod.cmo mod2.cmo
```
