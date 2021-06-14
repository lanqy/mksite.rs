---
title: 通过代数数据类型和模式匹配进行语言比较：Koka，Rust，Haxe，Swift，Elm，PureScript，Haskell，OCaml，ReasonML，Kotlin，Scala，Dotty，Ruby，TypeScript
created: 2018/05/21
description: 通过代数数据类型和模式匹配进行语言比较：Koka，Rust，Haxe，Swift，Elm，PureScript，Haskell，OCaml，ReasonML，Kotlin，Scala，Dotty，Ruby，TypeScript
author: lanqy
---
# 通过代数数据类型和模式匹配进行语言比较：Koka，Rust，Haxe，Swift，Elm，PureScript，Haskell，OCaml，ReasonML，Kotlin，Scala，Dotty，Ruby，TypeScript

## 代数数据类型

### Koka

```haskell
type color {
  Red
  Green
  Blue
  Rgb( r : int, g : int, b: int )
}
```

或

```haskell
type color {
  Red; Green; Blue; Rgb( r : int, g : int, b: int )
}
```

### rust 

```rust
enum Color {
 Red,
 Green,
 Blue,
 Rgb { r: u8, g: u8, b: u8 }
}
```

### Haxe 

```haxe
enum Color {
  Red;
  Green;
  Blue;
  Rgb(r: Int, g: Int, b: Int);
}
```

### swift


```swift
enum Color {
  case Red, Green, Blue, Rgb(r: Int, g: Int, b: Int)
}
```

### Elm

```elm
type Color = Red | Green | Blue | Rgb { r: Int, g: Int, b: Int }
```

### PureScript

```haskell
data Color = Red | Blue | Green | Rgb{ r :: Int, g :: Int, b :: Int }
```

### Haskell

```haskell
data Color = Red | Green | Blue | Rgb {r :: Int, g :: Int, b :: Int}
```

### OCaml

```ocaml
type rgb = { r: int; g: int; b: int }
type color = Red | Green | Blue | Rgb of rgb
```

### ReasonML

```ocaml
type rgb = { r: int, g: int, b: int };
type color =
  | Red
  | Green
  | Blue
  | Rgb(rgb);
```

### Kotlin

```kotlin
sealed class Color {
    object Red: Color()
    object Green: Color()
    object Blue: Color()
    class Rgb(val r: Int,val  g: Int,val  b: Int): Color()
}
```

### Scala

```scala
sealed trait Color

final case object Red extends Color
final case object Green extends Color
final case object Blue extends Color
final case class Rgb(r: Int, g: Int, b: Int) extends Color
```

### Dotty

```scala
enum Color {
  case Red
  case Green
  case Blue
  case Rgb(r: Int, g: Int, b: Int)
}
```

### TypeScript

```typescript
type Color = Red | Green | Blue | Rgb;
const enum ColorKind {
  Red = "Red",
  Green = "Green",
  Blue = "Blue",
  Rgb = "Rgb"
}

interface Red {
    kind: ColorKind.Red;
}

interface Green {
    kind: ColorKind.Green;
}

interface Blue {
    kind: ColorKind.Blue;
}

interface Rgb {
    kind: ColorKind.Rgb;
    r: number;
    g: number;
    b: number;
}
```

### Ruby

```ruby
module Color
  Red = 1
  Green = 2
  Blue = 3
  Rgb = Struct.new(:r, :g, :b)
end
```

## 感想

### 符号

1. Koka, Rust, Haxe, Elm: 非常好
2. Dotty, Swift, PureScript, Haskell, OCaml: 比较好
3. Ruby: 一般
4. Kotlin: 很难写
5. Scala: 写起来很难

## 模式匹配

### Koka

```haskell

match(color) {
  Red   -> "#FF0000"
  Green -> "#00FF00"
  Blue  -> "#0000FF"
  Rgb(r,g,b) -> "#" + showHex(r,2) + showHex(g,2) + showHex(b,2)
}
```

### Rust

```rust
match color {
    Color::Red   => "#FF0000".to_string(),
    Color::Green => "#00FF00".to_string(),
    Color::Blue  => "#0000FF".to_string(),
    Color::Rgb{r, g, b} => format!("#{:02X}{:02X}{:02X}", r, g, b),
}
```

### Haxe

```haxe
switch( color ) {
  case Red:   "#FF0000";
  case Green: "#00FF00";
  case Blue:  "#0000FF";
  case Rgb(r, g, b): "#"+ StringTools.hex(r,2) + StringTools.hex(g,2) + StringTools.hex(b,2);
}
```

### Swift

```swift
switch color {
  case .Red:
    return "#FF0000"
  case .Green:
    return "#00FF00"
  case .Blue:
    return "#0000FF"
  case let .Rgb(r, g, b):
    return String(format:"#%02X%02X%02X", r, g, b)
}
```

### Elm

```elm
case color of
    Red   -> "#FF0000"
    Green -> "#00FF00"
    Blue  -> "#0000FF"
    Rgb {r, g, b} -> String.concat ["#", (toHex r), (toHex g), (toHex b)]
```

### PureScript

```haskell
case color of
  Red   -> "#FF0000"
  Green -> "#00FF00"
  Blue  -> "#0000FF"
  Rgb { r, g, b } -> "#" <> toHex r <>  toHex g <>  toHex b 
```

### Haskell

```haskell
case color of
    Red   -> "#FF0000"
    Green -> "#00FF00"
    Blue  -> "#0000FF"
    Rgb r g b -> printf "#%02X%02X%02X" r g b
```

### OCaml

```ocaml
match color with
      Red   -> "#FF0000"
    | Green -> "#00FF00"
    | Blue  -> "#0000FF"
    | Rgb {r; g; b} -> Printf.sprintf "#%02X%02X%02X" r g b;;
```

### ReasonML

```ocaml
switch (color) {
  | Red => "#FF000"
  | Green => "#00FF00"
  | Blue => "#0000FF"
  | Rgb{r, g, b} => Format.sprintf("#%02X%02X%02X", r, g, b)
  };
```

### Kotlin

```kotlin
when ( color ) {
  Color.Red   -> "#FF0000"
  Color.Green -> "#00FF00"
  Color.Blue  -> "#0000FF"
  is Color.Rgb -> "#%02X%02X%02X".format(color.r, color.g, color.b)
}
```

### Scala

```scala
color match {
  case Red   =>"#FF0000"
  case Green =>"#00FF00"
  case Blue  =>"#0000FF"
  case Rgb(r, g, b) => "#%02X%02X%02X".format(r, g, b)
}
```

### Dotty

```scala
color match {
  case Color.Red   => "#FF0000"
  case Color.Green => "#00FF00"
  case Color.Blue  => "#0000FF"
  case Color.Rgb(r, g, b) => "#%02X%02X%02X".format(r, g, b)
}
```

### TypeScript

```typescript
switch (color.kind) {
    case ColorKind.Red: return "#FF0000";
    case ColorKind.Green: return "#00FF00";
    case ColorKind.Blue: return "#0000FF";
    case ColorKind.Rgb: return "#" + [color.r, color.g, color.b].map((v) => v.toString(16)).join('').toUpperCase();
    default:
        const _exhaustiveCheck: never = color;
        return _exhaustiveCheck;
}
```

### Ruby

```ruby
case color
when Color::Red; "#FF000"
when Color::Green; "#00FF00"
when Color::Blue; "#0000FF"
when Color::Rgb; "#%02X%02X%02X" % [color.r, color.g, color.b]
end
```

## 感想

### 符号

1. Koka, Rust, Elm, PureScript, Haskell: 非常好
2. OCaml, Kotlin: 好
3. Haxe, Scala, Dotty, Ruby: 一般
4. TypeScript, Swift: 很难写

## 编译器的默认行为

1. Rust，Haxe，Swift，Kotlin，Elm，PureScript：如果未覆盖该模式，则会出现编译错误
2. Scala，Dotty，OCaml，ReasonML：即使不包含模式，编译也会在没有警告的情况下传递
3. Koka，Haskell：即使不包含模式，编译也会在没有警告的情况下通过
4. Ruby：外部

来自： https://qiita.com/xmeta/items/91dfb24fa87c3a9f5993
