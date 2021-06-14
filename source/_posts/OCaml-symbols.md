---
title: OCaml 符号
created: 2018/10/23
description: OCaml 符号
author: lanqy
---
# OCaml 符号

## 1、 (* ... \*) ，(*\* ... \*\*) 

标识注释，(* ... \*)  一般用于代码注释，可嵌套。 (*\* ... \*\*)  一般用于文档注释，例如：
 
```ocaml
 (** 这里是注释. **)
val touch : (string * int) list -> string -> (string * int) list
```

## 2、 [ ... ] 

生成一个列表（类型）。 使用 ; 分隔符列表元素，例如：

```ocaml
# let words = ["foo"; "bar"; "baz"];;
val words : string list = ["foo"; "bar"; "baz"]
```

## 3、 ,

生成一个元组。但是，元组的类型是通过  *  号来分隔每个元素的，例如：

```ocaml
(* 类型定义 *)
type name = string * string

(* 元组生成 *)
let johndoe = ("John", "Doe")

(* 模式匹配 *)
match s with
| (first, last) -> Printf.printf "my name is %s %s" first last
```

## 4、 ;

这是一个句子休息。尽管 OCaml 的大部分语法都不需要这个分隔符，但它对短代码的函数并没有多大用处。正如上面已经提到的那样，这个符号也被用来分隔列表（下面）和记录元素

## 5、 &&

这是一个逻辑与（AND）运算符。我想你可以想象。还有，这是用于另一个目的的语法。

## 6、 ||

它是一个逻辑或（OR）运算符。or 也包含在保留字中，但现在已被弃用。

## 7、 :: 

合成列表的操作符， x :: xs  在  xs  列表的开头添加一个  x  元素，例如：

```ocaml
# let a = [2;3;4;5];;
val a : int list = [2; 3; 4; 5]
# let b = 1 :: a;;
val b : int list = [1; 2; 3; 4; 5]

# let a = 1 :: 2 :: 3 :: [];;
val a : int list = [1; 2; 3]
```


## 8、 '

单引号可以用于标识符。  x'  你也可以说变量名 f'  和函数名都可以。

但是， 你不能以 'a 的形式定义变量。这被称为类型变量，它用于类型符号。

## 9、 |

用于编写由变体类型和模式匹配分隔的多个模式。例如：

```ocaml
(* 变体类型的定义 *) 
type foobar = 
    Foo (* 可能是 Foo *) 
  | Bar 
  | Baz 
(* 模式匹配 *) 
match v with 
| Foo -> ... 
| Bar -> ... 
| Baz -> ...
```

## 10、 ->

它用于模式匹配守卫或类型的符号。例如： String.get  获取字符串中特定位置的字符的函数类型是  string -> int -> char 

```ocaml
(* 模式匹配 *) 
match v with 
| Foo -> ... 
| Bar -> ... 
| Baz -> ...
```

## 11、 ()

括号内没有任何内容。这是一个  unit ，它没有任何称为类型的东西......它在其他语言中是无效的。

() 如果你发现一个指定参数的函数，它是一个不需要任何参数的函数。函数不能在没有参数的情况下调用，因此可以通过指定来调用类型以()开头的 unit - >  函数。例如：

```ocaml
# Sys.getcwd () ;; (* 获取当前目录 *) 
-: string = "/Volumes/Users/szktty"
```

入口点是 let () = ... 也是这个符号，这是模式匹配，但不是一个参数。

## 12、 ^

连接字符串的运算符，这个操作符会生成一个新的字符串。例如：

```ocaml
# let name = "hello " ^ "lanqy";;
val name : string = "hello lanqy"
```

## 13、 +.  -.  *.  /. 

浮点数的加减乘除。例如：

```ocaml
(* 相加 *)
# 1.2 +. 1.3;;
- : float = 2.5
(* 相减 *)
# 1.3 -. 1.2;;
- : float = 0.10000000000000009
(* 相乘 *)
# 3.0 *. 2.0;;
- : float = 6.
(* 相除 *)
# 6.0 /. 2.0;;
- : float = 3.
```
## 14、  _

这是一个可以用于标识符的符号。在模式匹配中，它被称为通配符，但与其他变量相比，它实际上并没有特殊的功能。  _  即  v  它匹配任何值：

```ocaml
match exp with 
| 0 -> ... 
| v -> ... (* 匹配 0 以外的值 *)
```

但是，如果将此符号添加到变量名称（或函数名称）的开头，则该变量将绕过警告。通常情况下，如果您根本没有使用任何已定义的变量，编译器将被警告为未使用的变量，但该检查不会完成，不要写 let _ =  。

```ocaml
match exp with 
| 0 -> 0 
| v -> 1 (* w 被警告为未使用的变量，因为 v 未使用 *) 

match exp with 
| 0 -> 0 
| _ -> 1 (* 我不会收到警告 *)
```

另外，这不是一个单独的语言规范，也不是强制性的，但作为命名惯例_it有时附加到标识符的末尾（特别是记录字段名称），我们经常看到类似  end_ ,  to_  这样的命名方式，原因很简单，因为有同名的end, to 保留字，与其绞尽脑汁想一个命名，不如直接在单词后面加  _  ?

```ocaml
type location = {
  start : int;
  end_ : int;
}
```

## 15、  [| ... |] 

用于生成数组的语法（类型）， [...]  与这个区别是完美的，例如：

```ocaml
# [|1; 2; 3|];;
- : int array = [|1; 2; 3|]
```

## 16、  :=  <- 

:=  引用（指针）（赋值（当解引用不提供左值））,  <-  变量赋值或声明（赋值）, 例如：

```ocaml
# let y = ref None;;
val y : '_a option ref = {contents = None}
# y;;
- : '_a option ref = {contents = None}
# y := Some 3;;
- : unit = ()
# y;;
- : int option ref = {contents = Some 3}
# y := None;;
# y;;
- : int option ref = {contents = None}
let s = "hello world";;
let s' = s;;
s.[0] <- 'x';;
s';;
- : string = "xello world"
(* 数组 *)
let x = [| 2; 8; 3 |];;

(* 修改数组元素 *)
x.(1) <- 9;;             (* 设置索引为 1 的元素为 9 *)
x;;
- : int array = [|2; 9; 3|]
```

## 17、 = <> == !=

=  相等，  <>  不相等( 深层 )， ==  相等，  !=  不相等 ( 浅层次 )，OCaml中有两个相等运算符 = 和 == ，相应的不等式运算符 <> 和 !=。= 和 <> 检查结构相等，而 == 和 != 检查物理相等，例如：

```ocaml
# "a" == "a";;
- : bool = false
# "a" = "a";;
- : bool = true
# "a" != "a";;
- : bool = true
# "a" <> "a";;
- : bool = false
```

## 18、  ~ 

标签参数中使用的符号，例如：

```ocaml
# let f ~x ~y = x - y;;
val f : x:int -> y:int -> int = <fun>

# let x = 3 and y = 2 in f ~x ~y;;
- : int = 1

# let f ~x:x1 ~y:y1 = x1 - y1;;
val f : x:int -> y:int -> int = <fun>

# f ~x:3 ~y:2;;
- : int = 1
```

## 19、 ?

可选参数 = 用于标记参数的符号，可以省略。例如：

```ocaml
# let foo ?(z = 0) x y = x + y > z;;
val foo : ?z:int -> int -> int -> bool = <fun>
# foo 3 3 ~z: 2;;
- : bool = true
# foo 3 3 ~z: 10;;
- : bool = false
# foo 2 1;;
- : bool = true
```

(* 代码来自 https://stackoverflow.com/questions/23703470/ocaml-optional-argument *)

## 20、  ` 

表示多态变体的符号。还有其他多态变体符号  [>  和  [<  。

可以结合 http://blog.klipse.tech/reason/2018/03/12/blog-reason-types.html 这篇文章来理解  [>  和  [< 。

```ocaml
#  type card =  [  `Jorker  |  `Num  of  int  ];;  (* 多态变体类型 *) 
type card =  [  `Jorker  |  `Num  of  int  ] 

#  type in_data =  [  `Str  of  string  |  `Num  of  int  ] ;;  (* 多态变体类型 *) 
type in_data =  [  `Num  of  int  |  `Str  of  string  ] 

#  let get_number=  function  (* 接收多态变体参数 *) 
    `Num i -> i
   |  _  -> failwith " not a number " ;; 
val get_number: [>  ` Num  of 'a ]  -> ' a =  <fun > 

# get_number ( `Num  3 ) ;;  (* 应用多态变体类型 *) 
-: int  =  3
 ```
(* 代码来自 http://osiire.hatenablog.com/entries/2009/05/10 *)

## 21、  @@  和   |> 

OCaml 4.01 新添加的两个内置运算符： @@  和  |> ，它们非常简单，你可以在像这样的旧版本中自己定义它们：

```ocaml
let (@@) fn x = fn x
let (|>) x fn = fn x
(* 以下这两个是一样的 *)
print @@ "Hello";;
print "Hello";;
```

例如，这两行是等价的（我们加载一个文件，将其解析为XML，将生成的文档解析为选择文档，然后执行选择）

```ocaml
execute (parse_selections (parse_xml (load_file path)))

execute @@ parse_selections @@ parse_xml @@ load_file path
```

(* 代码来自 http://roscidus.com/blog/blog/2013/10/13/ocaml-tips/ *)

这样做的好处是，当你阅读一个  ( ，你必须沿着括号中的其余行扫描来找到匹配的。当你看到 @@ 时，你知道表达式的其余部分是前一个单一的参数功能。

管道运算符 |> 是相似的，但函数和参数是相反的。以下两行是相同的：

```ocaml
execute @@ parse_selections @@ parse_xml @@ load_file path

load_file path |> parse_xml |> parse_selections |> execute
```

(* 代码来自 http://roscidus.com/blog/blog/2013/10/13/ocaml-tips/ *)

## 22、  ;; 

顶层环境的结束符。例如：

```ocaml
let a = [2;3;4;5];;
val a : int list = [2; 3; 4; 5]
```

## 23、  @ 

列表的连接符。例如：


```ocaml
let a = [2;3] @ [4;5;6];;
val a : int list = [2; 3; 4; 5; 6]
```

参考资料：

- https://qiita.com/szktty/items/05cb2b754c88fbacc274
- http://roscidus.com/blog/blog/2013/10/13/ocaml-tips/
- https://stackoverflow.com/questions/23703470/ocaml-optional-argument
- http://rigaux.org/language-study/syntax-across-languages-per-language/OCaml.html
