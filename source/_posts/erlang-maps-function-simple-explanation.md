---
title: Erlang maps 函数 “简单” 的解释
description: Erlang maps 函数 “简单” 的解释
created: 2018/08/01
author: lanqy
---
# Erlang maps 函数 “简单” 的解释

译自：https://qiita.com/kayama0fa/items/b0e60644ed40b318a513

## Erlang maps 函数 “简单” 的解释

描述 maps 函数，一行的简要说明和示例代码。 有关更多信息，请参阅官方参考。

原始转发（官方参考）http://erlang.org/doc/man/maps.html

描述，以便您可以立即找到该功能，并能够大致掌握它的移动方式。 评述的顺序是根据基于主观性的重要性来安排的。

本页的TODO：

- 由于它几乎完全写入，检查是否有错误。
- 如果我能负担得起的话，我会做得更好一点。但是如果你写了别的东西，你想增加另一个库。
- 示例代码已改进

## 创建一个空 maps - maps:new/0

```erlang
M0 = maps:new(),
```

## 向 maps 添加值 - maps:put/3

```erlang
M1 = maps:put(a, 1, M0),
M2 = maps:put(b, 2, M1),
M3 = maps:put(c, 3, M2),
M4 = maps:put(d, "abc",M3),
M5 = maps:put(e, <<"アイウエオ"/utf8>>, M4), 
```

## 从 maps 中取值 - maps:get/2, maps:get/3

```erlang
1 = maps:get(a, M5),
not_found = maps:get(g, M5, not_found),
```

## 从 maps 中删除值 - maps:remove/2

```erlang
M4 = maps:remove(e, M5),
M5 = maps:remove(d, M4),
```

## 从 maps 中查找和检索值 - maps:find/2

```erlang
{ok, _} = maps:find(e, M5),
error = maps:find(g, M5),
```

## 检查 maps 是否具有指定的 key - maps:is_key/2

```erlang
true = maps:is_key(a, M5),
false = maps:is_key(x, M5),
```

## 获取在 maps 中注册的数据数量 - maps:size/1

```erlang
5 = maps:size(M5),
1 = maps:size(M1),
0 = maps:size(M0),
```

## 获取在 maps 中注册的 keys 列表 - maps:keys/1

```erlang
[a, b, c] = lists:sort(maps:keys(M3)),
```

## 获取在 maps 中注册的值列表 - maps:valus/1

```erlang
[1,2,3] = lists:sort(maps:values(M3)),
```

## maps 一个列表的 keys 和 值的元组 - maps:to_list/1

```erlang
[{a, 1},{b, 2}] = maps:to_list(M2),
```

## 从 keys 和值元组列表中获取 maps - maps:from_list/1

```erlang
M2 = maps:from_list([{a, 1}, {b, 2}]),
```

## 将任意筛选应用于 maps 以创建新 maps - maps:filter/2

```erlang
FilterM = maps:from_list([{a, 1},{b, 2}]),
FilterM = maps:filter(fun(_K, V) -> (V rem 3) =:= 1 end, M3),
```

## 在 maps 上执行卷积操作 - maps:fold/3

```erlang
6 = maps:fold(fun(_, V, Acc) when is_integer(V) -> V + Acc; (_, _, Acc) -> Acc end, 0, M5),
```

## 返回 maps 的每个值的任意处理结果 - maps:map/2

```erlang
NewMap = maps:from_list([{a, 10}, {b, 20}, {c, 30}]),
NewMap = maps:map(fun(_K, V) -> V * 10 end, M3),
```

## 合并两个 maps - maps:merge/2

％如果存在相同的键，则优先考虑第二个 maps 的值

```erlang
MapA = maps:from_list([{a, 10}, {b, 20}]),
MapB = maps:from_list([{a, 1}, {c, 30}]),
MapAB = maps:from_list([{a, 1}, {b, 20}, {c, 30}]),
MapAB = maps:merge(MapA, MapB),
```

## 更新 maps 的任何键的值 - maps:update/3

```erlang
UpdateMap1 = maps:from_list([{a, 1}, {b, 2}]),
UpdateMap2 = maps:from_list([{a, 10}, {b, 2}]),
UpdateMap2 = maps:update(a, 10, UpdateMap1),
```

## 用函数更新 maps 任意键的值 - maps:update_with/3

```erlang
UpdateMap2 = maps:update_with(a, fun(V) -> V * 10 end, UpdateMap1),
```

## 来自 update_with/3，一个可以在没有 key 时指定默认值的函数 - maps:update_with/4

```erlang
UpdateMap3 = maps:from_list([{a, 1}, {b, 2}, {c, 3}]),
UpdateMap3 = maps:update_with(c, fun(V) -> V * 10 end, 3, UpdateMap1),
```

## 仅使用指定的键从 maps 中重新创建 maps - maps:with/2

```erlang
WithKeys = [a, c],
WithMap1 = maps:from_list([{a, 1},{b, 2}, {c, 3}]),
WithMap2 = maps:from_list([{a, 1}, {c, 3}]),
WithMap2 = maps:with(WithKeys, WithMap1),
```

## 使用非指定的键重建 maps，使用 with/2 相反的键 - maps:without/2

```erlang
WithMap3 = maps:from_list([{b, 2}]),
WithMap3 = maps:without(WithKeys, WithMap1),
```

## 从已删除该键的 maps 和 maps 中返回任意键的值 - take/2

```erlang
{2, M1} = maps:take(b, M2)
```
