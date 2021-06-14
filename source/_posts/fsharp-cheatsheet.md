---
title: F# 备忘单
description: F# 备忘单
created: 2018/07/19
author: lanqy
---

# F# 备忘单

译自：https://dungpa.github.io/fsharp-cheatsheet/

这张备忘单浏览了一些 F# 3.0 的常用语法。如果您有任何意见，更正或建议添加内容，请打开问题或发送拉取请求到https://github.com/dungpa/fsharp-cheatsheet。

## 注释

块注释放在 (* 和 *) 之间。行注释从 // 开始并一直持续到行结束。

```text
(* 这是块注释 *)

// 这是行注释
```

在 /// 允许我们使用 XML 标记生成文档后，XML 文档注释就会出现。

```fsharp
/// `let`关键字定义了一个（不可变的）值
let result = 1 + 1 = 2
```

## 字符串

F# 字符串类型是 System.String 类型的别名。

```fsharp
/// 使用字符串连接创建字符串
let hello = "Hello" + "World"
```

使用以 @ 符号开头的逐字字符串以避免转义控制字符（除了通过 "" 转义 "）。

```fsharp
let verbatimXml = @"<book title="" Paradise Lost "">"
```
我们甚至不必转义 " 使用三引号字符串。

```fsharp
let tripleXml = """<book title="Paradise Lost">"""
```

反斜杠字符串通过去除前导空格来缩进字符串内容。

```fsharp
let poem = 
    "The lesser world was daubed\n\
     By a colorist of modest skill\n\
     A master limned you in the finest inks\n\
     And with a fresh-cut quill."
```

## 基本类型和文字

大多数数字类型都有相关的后缀，例如，uy 表示无符号 8 位整数，L 表示有符号 64 位整数。

```fsharp
let b, i, l = 86uy, 86, 86L

val b : byte = 86uy
val i : int = 86
val l : int64 = 86L
```

其他常见的例子是 32 位浮点数的 F 或 f ，小数的 M 或 m ，大整数的 I。

```fsharp
let s, f, d, bi = 4.14F, 4.14, 0.7833M, 9999I

val s : float32 = 4.14f
val f : float = 4.14
val d : decimal = 0.7833M
val bi : System.Numerics.BigInteger = 9999
```

有关完整参考，请[参阅文字（MSDN）](http://msdn.microsoft.com/en-us/library/dd233193.aspx)。

## 函数

let 关键字还定义了命名函数。

```fsharp
let negate x = x * -1
let square x = x * x
let print x = printfn "The number is: %d" x

let squareNegateThenPrint x = 
    print (negate (square x))
```

### 管道和组合操作符

管道运算符 |> 用于将函数和参数链接在一起。双重反引号标识符便于提高可读性，尤其是在单元测试中：

```fsharp
let ``square, negate, then print`` x = 
    x |> square |> negate |> print
```

此运算符在使用前通过提供类型信息来协助 F# 类型检查器是必不可少的：

```fsharp
let sumOfLengths (xs : string []) =
    xs
    |> Array.map (fun s -> s.Length)
    |> Array.sum
```

组合运算符 >> 用于组合函数：

```fsharp
let squareNegateThenPrint' = 
    square >> negate >> print
```

### 递归函数

rec 关键字与 let 关键字一起用于定义递归函数：

```fsharp
let rec fact x = 
    if x < 1 then 1
    else x * fact (x - 1)
```

相互递归函数（那些相互调用的函数）由 and 关键字表示：

```fsharp
let rec even x = 
    if x = 0 then true
    else odd (x - 1)
end odd x = 
    if x = 1 then true
    else even (x - 1)
```

## 模式匹配

通常通过 match 关键字来促进模式匹配。

```fsharp
let rec fib n = 
    match n with
    | 0 -> 0
    | 1 -> 1
    | _ -> fib (n - 1) + fib (n - 2)
```

为了匹配复杂的输入，可以使用 when 在模式上创建过滤器或防护：

```fsharp
let sign x = 
    match x with
    | 0 -> 0
    | x when x < 0 -> -1
    | x -> 1
```

模式匹配可以直接在参数上完成：

```fsharp
let fst' (x, _) = x
```

或通过 function 关键字隐式：

```fsharp
/// 类似于 `fib`;使用 `function` 进行模式匹配
let rec fib' = function
    | 0 -> 0
    | 1 -> 1
    | n -> fib' (n - 1) + fib' (n - 2)
```

有关更完整的参考，请访问[模式匹配（MSDN）](http://msdn.microsoft.com/en-us/library/dd547125.aspx)。

## 集合

### 列表

列表是相同类型的元素的不可变集合。

```fsharp
// 列表使用方括号和`;`分隔符
let list1 = ["a";"b"]
// :: 用于将元素加在列表开头
let list2 = "c" :: list1
// @ 用于连接列表
let list3 = list1 @ list2

// 使用( :: )运算符在列表上递归
let rec sum list = 
    match list with
    | [] -> 0
    | x :: xs -> x + sum xs
```

### 数组

数组是连续数据元素的固定大小，从零开始，可变的集合。

```fsharp
// 数组使用方括号和条形
let array1 = [|"a";"c"|]
// 使用点进行索引访问
let first = array1.[0]
```

### 序列

序列是相同类型的逻辑系列元素。仅根据需要计算各个序列元素，因此在不使用所有元素的情况下，序列可以提供比列表更好的性能。

```fsharp
// 序列可以使用 yield 并包含子序列
let seq1 = 
    seq {
        // “yield”增加一个元素
        yield 1
        yield 2

        // "yield!" 添加一个完整的子序列
        yield! [5..10]
    }

```

### 集合上的高阶函数

同样的清单 [1; 3; 5; 7; 9] 或数组 [| 1; 3; 5; 7; 9 |] 可以以各种方式生成。

- 使用范围运算符..
```fsharp
let xs = [1..2..9]
```
- 使用列表或数组理解
```fsharp
let yx = [| for i in 0..4 -> 2 * i + 1 |]
```
- 使用 init 函数
```fsharp
let zs = List.init 5 (fun i -> 2 * i + 1)
```

列表和数组具有用于操作的全面的高阶函数集。

- fold 从列表（或数组）的左侧开始，foldBack 的方向相反
```fsharp
let xs' = Array.fold (fun str n -> sprintf "%s,%i" str n) "" [|0..9|]
```
- reduce 不需要初始累加器
```fsharp
let last xs = List.reduce (fun acc x -> x) xs
```
- map 转换列表（或数组）的每个元素
```fsharp
let ys' = Array.map (fun x -> x * x) [|0..9|]
```
- iter 列表并产生副作用
```fsharp
let _ = List.iter (printfn "%i") [0..9]
```

所有这些操作也可用于序列。序列的附加好处是对实现 IEnumerable<'T> 的所有集合的懒惰和统一处理。

```fsharp
let zs' = 
    seq {
        for i in 0..9 do
            printfn "Adding %d" i
            yield i
    }
```

## 元组和记录

元组是一组未命名但有序的值，可能是不同类型的：

```fsharp
// 元组结构
let x = (1, "Hello")

// 三重
let y = ("one", "two", "three")

// 元组解构/模式
let (a', b') = x
```

可以使用 fst，snd 或模式匹配获得元组的第一个和第二个元素：

```fsharp
let c' = fst (1, 2)
let d' = snd (1, 2)

let print' tuple = 
    match tuple with
    | (a, b) -> printfn "Pair %A %A" a b
```

记录表示命名值的简单聚合，可选择包含成员：

```fsharp
// 声明记录类型
type Person = { Name: string; Age: int }

// 通过记录表达式创建值
let paul = { Name = "Paul"; Age = 28}

// '复制并更新'记录表达式
let paulsTwin = {paul with Name = "Jim"}

```

记录可以使用属性和方法进行扩充：

```fsharp
type Person with
    member x.Info = (x.Name, x.Age)
```

记录本质上是带有额外顶部的密封类：默认不变性，结构相等和模式匹配支持。

```fsharp
let isPaul person = 
    match person with
    | { Name = "Paul" } -> true
    | _ -> false
```

## 识别联合

识别联合（DU）为可以是多个命名案例之一的值提供支持，每个案例可能具有不同的值和类型。

```fsharp
type Tree<'T> =
    | Node of Tree<'T> * 'T * Tree<'T>
    | Leaf

let rec depth = function 
    | Node(l, _, r) -> 1 + max (depth l) (depth r)
    | Leaf -> 0
```

F# Core 有一些用于错误处理的内置区分联合，例如 [Option](http://msdn.microsoft.com/en-us/library/dd233245.aspx) 和 [Choice](http://msdn.microsoft.com/en-us/library/ee353439.aspx) 。

```fsharp
let optionPatternMatch input = 
    match input with
    | Some i -> printfn "input is an int=%d" i
    | None -> printfn "input is missing"
```

单例区分联合通常用于创建具有模式匹配支持的类型安全抽象：

```fsharp
type OrderId = Order of string

// 创建DU值
let orderId = Order "12"

// 使用模式匹配来解构单个案例 DU
let (Order id) = orderId
```

## 异常

failwith 函数抛出异常类型 Exception 。

```fsharp
let divideFailwith x y = 
    if y = 0 then
        failwith "Divisor cannot be zero"
    else x / y
```

异常处理通过 try / with 表达式完成。

```fsharp
let divide x y =
    try
        Some (x / y)
    with :? System.DivideByZeroException ->
        printfn "Division by zero!"
        None
```

try / finally 表达式使您可以执行清理代码，即使代码块引发异常也是如此。这是一个定义自定义异常的示例。

```fsharp
exception InnerError of string
exception OuterError of string

let handleErrors x y =
    try
        try
            if x = y then raise (InnerError("inner"))
            else raise (OuterError("outer"))
        with
            printfn "Error1 %s" str
    finally
        printfn "Always print this."
```

## 类和继承

此示例是一个基本类，包含（1）本地 let 绑定，（2）属性，（3）方法和（4）静态成员。

```fsharp
type Vector(x: float, y: float) =
    let mag = sqrt(x * x + y * y) // (1)
    member this.X = x // (2)
    member this.Y = y
    member this.Mag = mag
    member this.Scale(s) = // (3)
        Vector(x * s, y * s)
    static member (+) (a: Vector, b: Vector) = // (4)
        Vector(x.X + b.X, a.Y + b.Y)
```

从派生类调用基类。

```fsharp
type Animal() =
    member __.Reset() = ()

type Dog() = 
    inherit Animal()
    member __.Run() =
        base.Reset()
```

上传表示为 :> 运算符。

```fsharp
let dog = Dog()
let animal = dog :> Animal
```

如果转换在运行时未成功，则动态向下转换（ :?> ）可能会抛出 InvalidCastException。

```fsharp
let shouldBeDog = animal :?> Dog
```

## 接口和对象表达式

声明 IVector 接口并在 Vector' 中实现它。

```fsharp
type IVector
    abstract Scale : float -> IVector

type Vector'(x, y) = 
    interface IVector with
        member __.Scale(s) =
            Vector'(x * s, y * s) :> IVector
    member __.X = x
    member __.Y = y
```

实现接口的另一种方法是使用对象表达式。

```fsharp
type ICustomer =
    abstract Name : string
    abstract Age : int

let createCustomer name age = 
    { new ICustomer with
        member __.Name = name
        member __.Age = age }
```

## 活动模式

完整的活动模式：

```fsharp
let (|Even|Odd|) i = 
    if i % 2 = 0 then Even else Odd

let testNumber i = 
    match i with
    | Even -> printfn "%d is even" i
    | Odd -> printfn "%d is odd" i
```

参数化活动模式：

```fsharp
let (|DivisibleBy|_|) by n =
    if n % by = 0 then Some DivisibleBy else None

let fizzBuzz = function
    | DivisibleBy 3 & DivisibleBy 5 -> "FizzBuzz"
    | DivisibleBy 3 -> "Fizz"
    | DivisibleBy 5 -> "Buzz"
    | i -> string i
```

部分活动模式共享参数化模式的语法，但它们的主动识别器只接受一个参数。

## 编译器指令

将另一个 F# 源文件加载到 FSI 中。

```fsharp
#load "../lib/StringParsing.fs"
```

引用.NET程序集（建议使用/符号以实现Mono兼容性）。

```fsharp
#r "../lib/FSharp.Markdown.dll"
```

在程序集搜索路径中包含目录。

```fsharp
#I "../lib"
#r "FSharp.Markdown.dll"
```

其他重要指令是 FSI（INTERACTIVE）中的条件执行和查询当前目录（ __SOURCE_DIRECTORY__ ）。

```fsharp
#if INTERACTIVE
let path = __SOURCE_DIRECTORY__ + "../lib"
#else
let path = "../../../lib"
#endif
```
