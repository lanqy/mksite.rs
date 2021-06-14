---
title: OCaml简介第3部分
description: OCaml简介第3部分
created: 2018/07/14
author: lanqy
---

## OCaml简介第3部分

译自：https://qiita.com/zenwerk/items/bfc1978718b5da3f463b

### 异常处理

它在您除以零或指定一个不存在的文件时发生。

```ocaml
# 1/0;;
Exception: Division_by_zero.
# open_in "";;
Exception: Sys_error ": No such file or directory".
```
#### 抛出异常（ raise 表达式 ）

##### raise 异常
##### raise（ 异常参数 ）

如果异常需要参数，则（）是必需的。

```ocaml
# raise Not_found;;
Exception: Not_found.
# raise (Sys_error ": No such file or directory");;
Exception: Sys_error ": No such file or directory".
# raise (Sys_error ": 我会抛出异常！");;
Exception: Sys_error ": ?\136\145?\154?\138\155?\135??\130常?\129".
```

```ocaml
# let rec fact n =
    if n < 0 then raise (Invalid_argument ": negative argument")
    else if n = 0 then 1 else n * fact (n-1);;
val fact : int -> int = <fun>
# fact 5;;
- : int = 120
# fact (-1);;
Exception: Invalid_argument ": negative argument".
```

#### 异常处理( try with )

##### try 表达式 with 异常1 -> 表达式1 | ...

```ocaml
# try raise Not_found with
  | Not_found -> "not found !"
  | _ -> "unknown !";;
- : string = "not found !"

(* 前面定义的 fact 函数例子 *)
# try fact (-1) with
  | Invalid_argument _ -> 0
  | _ -> 9999;;
- : int = 0
```
#### 异常定义

异常类型的构造函数中称为异常构造函数。
该变种是 exn 类型

```ocaml
(* 确认异常的变体类型 *)
# Not_found;;
- : exn = Not_found
# raise;;
- : exn -> 'a = <fun>
```

异常定义是为exn类型添加一个新的构造函数。

exn类型是特殊的，你可以稍后添加一个构造函数。

##### exception 异常名

```ocaml
(* 异常定义 *)
# exception Hoge;;
exception Hoge
# exception Fuga of string;;
exception Fuga of string
# raise Hoge;;
Exception: Hoge.
# raise (Fuga "fuga!");;
Exception: Fuga "fuga!".
```

#### 关于exn类型

既然exn也是一个变体类型，它也可以作为一个参数传递。

```ocaml
# exception Hoge;;
exception Hoge

(* exn 类型列表 *)
# let exnlist = [Not_found; Hoge; (Invalid_argument "fuga")];;
val exnlist : exn list = [Not_found; Hoge; Invalid_argument "fuga"]

(* 接收exn类型的函数 *)
# let f = function
  | Hoge -> "hoge!"
  | x -> raise x;;
val f : exn -> string = <fun>
# f Hoge;;
- : string = "hoge!"
# f Not_found;;
Exception: Not_found.
```
### unit 类型

输出字符串的程序。

```ocaml
# print_string "hoge\n";;
hoge
- : unit = ()
```
返回类型是 unit 类型

unit 类型的值只是一个名为（）的常量，称为 unit 值。

#### unit 类型的用法

* （）上没有可以执行的操作
* 用作返回值本身没有意义的函数的返回值
* 在定义不需要有意义的参数的函数时用作参数

```ocaml
# let const () = 777;;
val const : unit -> int = <fun>
# const ();;
- : int = 777
```

##### 用作判断操作是否成功的返回值

```ocaml
（*
   () 将匹配模式，如果操作成功，将返回单位类型。
   也就是说，如果匹配成功，则表示操作成功。
*）
# let () = Bytes.set "Test" 1 'C';;
```

### 可变的数据结构

#### 修改字符串

##### "string".[index] <- 'char'

```ocaml
# let s = "life";;
val s : string = "life"
# s.[2] <- 'v';;
- : unit = ()
# s;;
- : string = "live"

(* String.set 弃用 *)
# let f2 = "hoge";;
val f2 : string = "hoge"
# Bytes.set f2 0 'H';;
- : unit = ()
# f2;;
- : string = "Hoge"
```

##### 该操作操作参考目的地 

```ocaml
# let s = "hoge";;
val s : string = "hoge"
# let a = (s, s);;
val a : string * string = ("hoge", "hoge")
# Bytes.set s 3 'E';;
- : unit = ()

(* 两者都被改变，因为参考目标是相同的 *)
# a;;
- : string * string = ("hogE", "hogE")
```

#### 物理相等

##### 物理相等 => 比较数据地址时的平等性

* 使用 ==, !=

##### 结构相等 => 平等作为价值进行比较

* 使用=，<>

```ocaml
# let s1 = "hoge" and s2 = "hoge";;
val s1 : string = "hoge"
val s2 : string = "hoge"
(* 结构相等 *)
# s1 = s2;;
- : bool = true
(* 物理相等 *)
# s1 == s2;;
- : bool = false
# s1 != s2;;
- : bool = true
```
#### 可修改的记录
* 修改记录 => 使用 mutable 关键字
* 记录修改 => record.field <- 值

```ocaml
# type account = {name:string;mutable amount:int};;
type account = { name : string; mutable amount : int; }
# let ac = {name = "bob"; amount = 1000};;
val ac : account = {name = "bob"; amount = 1000}
# ac.amount <- 999;;
- : unit = ()
# ac;;
- : account = {name = "bob"; amount = 999}
(* 不可改变 *)
# let () = ac.name <- "Hoge";;
Error: The record field name is not mutable
(* 这样是可以的 *)
# ac.name.[0] <- 'B';;
- : unit = ()
# ac;;
- : account = {name = "Bob"; amount = 999}
```

#### 创建引用（ ref ）
* 创建引用 => ref函数
* 引用获取 => !引用
* 引用目标重写 => 引用 := 值

```ocaml
(* 创建引用 *)
# let h = ref "Hoge" and f = ref "Fuga";;
val h : string ref = {contents = "Hoge"}
val f : string ref = {contents = "Fuga"}

(* 引用获取 *)
# let () = print_string (!h ^ !f ^ "\n");;
HogeFuga

(* 引用重写 *)
# h := !f;;
- : unit = ()
# let () = print_string (!h ^ !f ^ "\n");;
FugaFuga
```

#### 引用的引用 ( ref ref )

它也可以像C语言中的双指针一样
```ocaml
(* 创建引用 *)
# let r1 = ref 5 and r2 = ref 2;;
val r1 : int ref = {contents = 5}
val r2 : int ref = {contents = 2}
(* 引用的引用创建 *)
# let rr1 = ref r1 and rr2 = ref r2;;
val rr1 : int ref ref = {contents = {contents = 5}}
val rr2 : int ref ref = {contents = {contents = 2}}
(* 引用操作 *)
# let () = !rr1 := !(!rr2);;
# (!r1, !r2);;
- : int * int = (2, 2)
```

#### 数组
* [|值1;值2; ... |]
* 引用元素 => [| ... |].( 下标 )
* 改变数组元素 => [| ... |].( 下标 ) <- value

数组的长度是固定的。

您可以通过下标直接获取任何元素。

### 程序控制结构

作为OCaml的规范，参数的评估顺序是未定义的。

因此，准备了用于像过程性语言那样控制执行顺序的语法。

#### 顺序执行

##### 表达式1; 表达式2; ... 表达式n

与如下如下代码一样

```ocaml
let _ = 表达式1 in
let _ = 表达式2 in
.
.
.
表达式n
```
最后的表达式n是整个返回值。

#### begin 与 end

你可以写 begin 表达式1; ...表达式n end ，而不是（表达式; ...表达式n）

这似乎是首选。

#### ignore

如果相当于一个句子的表达式返回非 unit 类型，则会出现警告

忽略警告的函数 

```ocaml
(* 将返回值设置为0（int） *)
# let print_hello () = print_string "Hello, "; 0;;
val print_hello : unit -> int = <fun>

(* 警告出现在 print_hello *)
# print_hello (); print_string "World\n";;
Warning 10: this expression should have type unit.
Hello, World
- : unit = ()

(* 忽略不发出警告 *)
# ignore (print_hello ()); print_string "World\n";;
Hello, World
- : unit = ()
```
;是一个分隔符

;不是终止符，它被用作分隔符。

因此，要小心如果在表达式n的末尾添加一个分号作为表达式n，经常会出现额外的错误。

#### 条件语句

if then 的 then 子句的返回值是（），则else可以省略。

if 表达式1 then 表达式2 => if 表达式1 then 表达式2 else（）

这意味着如果表达式1不成立，将会做任何事情。

#### 循环语句

for 和 while 语句

##### for 

for => for variable = start_value to end_value do 表达式 done

或

for variable = start_value downto end_value do 表达式 done

```ocaml
(* for 循环 *)
# for i = 1 to 10 do begin print_int i; () end done;;
12345678910- : unit = ()
# for i = 10 downto 1 do begin print_int i; () end done;;
10987654321- : unit = ()
```

##### while

while 条件 do 表达式 done

while据说它用得不多。

```ocaml
let quit_loop = ref false in
  while not !quit_loop do
  print_string "Have you had enough yet? (y/n) ";
  let str = read_line () in
  if str.[0] = 'y' then
      quit_loop := true
  done;;
Have you had enough yet? (y/n) n
Have you had enough yet? (y/n) y
- : unit = ()
```
