---
title: OCaml简介第2部分
description: OCaml简介第2部分
created: 2018/07/13
author: lanqy
---

## OCaml简介第 2 部分

译自：https://qiita.com/zenwerk/items/603bd383fe5c6b8cace3

### 递归多层数据结构。

列表类型

```ocaml
# [1;2;3;4;5];;
- : int list = [1; 2; 3; 4; 5]
# ["a"; "b"; "c"];;
- : string list = ["a"; "b"; "c"]
(* 不同的类型不能存在于同一个列表 *)
# [1; "a"];;
Error: This expression has type string but an expression was expected of type int
```

### 将值添加到列表的开头

使用consus操作符(::)。

右结合

```ocaml
# 1 :: [2; 3; 4];;
- : int list = [1; 2; 3; 4]

(* 右結合 *)
# 1 :: 2 :: [3; 4];;
- : int list = [1; 2; 3; 4]
```

### 列表合并

使用 @ 。

```ocaml
# [] @ [];;
- : 'a list = []
# [1] @ [2; 3];;
- : int list = [1; 2; 3]
# ["asdf"; "hoge"] @ ["fuga"];;
- : string list = ["asdf"; "hoge"; "fuga"]
```

### 模式匹配

match 表达式

match 表达式 with 模式1 - >表达式 | 模式2 - >表达式...

找到整数列表的总和的例子:

```ocaml
# let rec total l =
    match l with
      [] -> 0
    | h :: rest -> h + (total rest);;
val total : int list -> int = <fun>
# total [1; 2; 3; 4; 5];;
- : int = 15
```
反转列表的函数示例:

```ocaml
# let reverse l =
    let rec innerReverse l1 l2 =
      match l1 with
      | [] -> l2
      | h :: rest -> innerReverse rest (h :: l2)
    in
    innerReverse l [];;
val reverse : 'a list -> 'a list = <fun>
# reverse [1; 2; 3; 4];;
- : int list = [4; 3; 2; 1]
```

### function 表达式

fun 和 match 通过组合定义一个匿名函数

function 模式1 - > 表达式 | 模式2 - > 表达式...

上面整数列表的总和的例子可以改写如下：

当使用最后一个参数进行模式匹配时方便，并且该参数仅用于模式匹配

```ocaml
# let rec total = function
    [] -> 0
  | h :: rest -> h + (total rest);;
val total : int list -> int = <fun>
# total [1; 2; 3; 4; 5];;
- : int = 15
```

### map 函数的例子

```ocaml
# let rec map fn = function
    | [] -> []
    | h :: rest -> fn h :: map fn rest;;
val map : ('a -> 'b) -> 'a list -> 'b list = <fun>
# map (fun x -> x + 1) [1; 2; 3; 4];;
- : int list = [2; 3; 4; 5]
```
### fold (折叠)函数例子

```ocaml
(* 左fold *)
# let rec foldl fn acc l =
    match l with
    | [] -> acc
    | h :: rest -> foldl fn (fn acc h) rest;;

(* 用于查找列表长度的 fold 示例 *)
# foldl (fun acc x -> acc + 1) 0 [1; 2; 3];;
- : int = 3

(* 右fold *)
# let rec foldr fn l acc =
    match l with
    | [] -> acc
    | h :: rest -> fn h (foldr fn rest acc);;
val foldr : ('a -> 'b -> 'b) -> 'a list -> 'b -> 'b = <fun>
```

### 模式匹配守卫子句

match 表达式 with 模式1 when 真/假表达式 - >表达式| ...

使用 when。

### 注意：match，function 中没有关闭符号

在match with 和 function 上没有结束符号。

因此，当模式匹配嵌套时，必须用（）等括起来

### 数据结构

#### 记录（record）

C语言结构，数据结构等同于 Python 字典。

命名元素名称。

#### 记录定义

* type name = {field name：type; ...}
* 字段 - > 名称和值对
* 请注意，字段名称不能与其他记录重复

```ocaml
# type student = {name: string; id: int};;
type student = { name : string; id : int; }
```
#### 创建记录

* {Field name = value; ...}

```ocaml
# let s = {name = "hoge"; id = 1};;
val s : student = {name = "hoge"; id = 1}
```

### 记录转移

创建一个新的记录值，而不是覆盖现有的值。

{记录 with 字段名称 = 值; ...}

```ocaml
# let s2 = {s with name = "fuga"};;
val s2 : student = {name = "fuga"; id = 1}
```
### 变体

数据结构代表案例分类。 

```ocaml
type name =
  | 构造函数 [ of <type> [* <type>]... ]
  | 构造函数 [ of <type> [* <type>]... ]
  | ...
```
* 构造函数以英文大写字母开头
* of 是构造函数所需的参数类型
* of int * int 参数不是一组 int ，两个 int
* of (int * int) 参数是一对 int

以下是四种数字的变体类型：

```ocaml
# type figure =
  | Point
  | Circle of int
  | Rectangle of int * int (* 两个 int 类型的参数，不是元组 *)
  | Square of int;;
type figure = Point | Circle of int | Rectangle of int * int | Square of int

# let c = Circle 4;;
val c : figure = Circle 4
# let figs = [Point; Rectangle (1, 1); c];;
val figs : figure list = [Point; Rectangle (1, 1); Circle 4]
```

### 变体的模式匹配

#### function | 构造函数的参数 -> | ...

省略参数部分意味着没有参数构造函数。

```ocaml
(* 计算图形面积的例子 *)
# let area = function
  | Point -> 0
  | Circle r -> r * r * 3
  | Rectangle (x, y) -> x * y
  | Square x -> x * x;;
val area : figure -> int = <fun>
# area c;;
- : int = 48
```

### 多态变体类型

* 可以使用类型变量（ 'a 等）来定义变体类型，
* 它也被称为带参数的参数。
* 例如，'a list 可以使用多态变量来表示。

```ocaml
# type 'a mylist =
  | Nil
  | Cons of 'a * 'a mylist;;
type 'a mylist = Nil | Cons of 'a * 'a mylist
# Nil;;
- : 'a mylist = Nil

(* 表示一个整数列表 *)
# Cons(1, Nil);;
- : int mylist = Cons (1, Nil)

(* 表示字符列表 *)
# Cons('a', Cons('b', Nil));;
- : char mylist = Cons ('a', Cons ('b', Nil))
```

### Optional

通常说可选，表明这些情况下没有值

```ocaml
# type 'a option =
  | None
  | Some of 'a;;
type 'a option = None | Some of 'a
```
使用可选的示例

```ocaml
# let fact n =
    let rec fact' n = if n = 0 then 1 else n * fact' (n - 1) in
    if n < 0 then None else Some (fact' n);;
val fact : int -> int option = <fun>
# fact 3;;
- : int option = Some 6
# fact (-10);;
- : int option = None
```

### 递归变体类型

在构造函数中 of 下面是它自己的类型出现的变体类型

#### 二叉树的例子

```ocaml
# type 'a btree =
  | Leaf
  | Node of 'a * 'a btree * 'a btree;;
type 'a btree = Leaf | Tree of 'a * 'a btree * 'a btree

# Node(1, Node(1, Leaf, Leaf), Node(1, Node(1, Leaf, Leaf), Leaf));;
- : int btree =
Node (1, Node (1, Leaf, Leaf), Node (1, Node (1, Leaf, Leaf), Leaf))
```
查找树的元素数量和高度的函数示例

```ocaml
# let tr = Node(1, Node(1, Leaf, Leaf), Node(1, Node(1, Leaf, Leaf), Leaf));;  
val tr : int btree =
  Node (1, Node (1, Leaf, Leaf), Node (1, Node (1, Leaf, Leaf), Leaf))

(* 用于查找高度的函数 *)
# let rec height = function
  | Leaf -> 0
  | Node(_, left, right) -> 1 + max (height left) (height right);;
val height : 'a btree -> int = <fun>
# height tr;;
- : int = 3

(* 用于查找元素数目的函数 *)
# let rec size = function
  | Leaf -> 0
  | Node (_, left, right) -> 1 + size left + size right;;
val size : 'a btree -> int = <fun>
# size tr;;
- : int = 4
```

#### 二叉搜索树的例子

* 添加元素

```ocaml
# let rec insert x = function
  | Leaf -> Node(x, Leaf, Leaf)
  | Node(k, left, right) ->
      if x < k then Node(k, insert x left, right)
      else Node(k, left, insert x right);;
val insert : 'a -> 'a btree -> 'a btree = <fun>
```

* 搜索元素

```ocaml
# let rec mem x = function
  | Leaf -> false
  | Node(k, left, right) ->
      if x < k then mem x left
      else if x = k then true
      else mem x right;;
val mem : 'a -> 'a btree -> bool = <fun>
```

* 使用例子

```ocaml
# let tr = Leaf;;
val tr : 'a btree = Leaf
# tr;;
- : 'a btree = Leaf
# insert 10 tr;;
- : int btree = Node (10, Leaf, Leaf)
# let tr = insert 10 tr;;
val tr : int btree = Node (10, Leaf, Leaf)
# let tr = insert 5 tr;;
val tr : int btree = Node (10, Node (5, Leaf, Leaf), Leaf)
# let tr = insert 20 tr;;
val tr : int btree = Node (10, Node (5, Leaf, Leaf), Node (20, Leaf, Leaf))
# mem 5 tr;;
- : bool = true
# mem 15 tr;;
- : bool = false
```

### 玫瑰树的一个例子

玫瑰树是元素数量未知的树。

它可以被认为是与 UNIX 相同的目录结构。

* 类型定义 

```ocaml
(* 由于子元素的数量是不确定的，节点的元素是列表 *)
# type 'a rosetree = RLeaf | RNode of 'a * 'a rosetree list;;
type 'a rosetree = RLeaf | RNode of 'a * 'a rosetree list
```
XML作为玫瑰树

* 类型定义

因为它是 XML ，叶子也没有价值 - >（'b option）

```ocaml
(* 'a 标记, 'b 值 *)
# type ('a, 'b) xml = XLeaf of 'b option | XNode of 'a * ('a, 'b) xml list;;
type ('a, 'b) xml = XLeaf of 'b option | XNode of 'a * ('a, 'b) xml list
```
对XML数据结构进行字符串化的函数

递归XML（Rose Tree）包含一个递归数据结构的列表

它是相互递归的定义
```ocaml
# let rec string_of_xml = function
  | XNode(tag, xml_list) -> "<" ^ tag ^ ">" ^ string_of_xmllist xml_list ^ "</" ^ tag ^ ">"
  | XLeaf None -> ""
  | XLeaf(Some s) -> s
  and
  string_of_xmllist = function
  | [] -> ""
  | xml :: rest -> string_of_xml xml ^ string_of_xmllist rest;;
val string_of_xml : (string, string) xml -> string = <fun>
val string_of_xmllist : (string, string) xml list -> string = <fun>
```

### 无限的列

#### 整数的无限列的例子

```ocaml
# type intseq = Cons of int * (int -> intseq);;
type intseq = Cons of int * (int -> intseq)
```
* 无限列的示例递增

```ocaml
# let rec f x = Cons(x+1, f);;
val f : int -> intseq = <fun>
# f 0;;
- : intseq = Cons (1, <fun>)
# f 100;;
- : intseq = Cons (101, <fun>)
```
* 如果返回值的 x 是下一个元素

   通过给x作为参数，我们得到元素的顺序
   
```ocaml
# let Cons(x, f) = f 0;;
val x : int = 1
val f : int -> intseq = <fun>
# let Cons(x, f) = f x;;
val x : int = 2
val f : int -> intseq = <fun>
# let Cons(x, f) = f x;;
val x : int = 3
val f : int -> intseq = <fun>
```
* 获取第 N 个元素函数

```ocaml
# let rec nthseq n (Cons(x, f)) =
    if n = 1 then x
    else nthseq (n-1) (f x);;
val nthseq : int -> intseq -> int = <fun>
# nthseq 10 (f 0);;
- : int = 10
```
