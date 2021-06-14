---
title: 使用 Rebar3 构建您的第一个 Erlang 应用程序
created: 2018/08/10
description: 使用 Rebar3 构建您的第一个 Erlang 应用程序
author: lanqy
---
# 使用 Rebar3 构建您的第一个 Erlang 应用程序

Rebar3 是 Erlang 的构建工具和包管理工具。由于 Rebar3 带有 [Hex](https://hex.pm/) 插件，因此创建和发布 Erlang 软件包非常简单。让我们制作一个简单的 “hello world” 包，随意在家玩！

## 下载 Rebar3

在此下载最新版本：[http://www.rebar3.org/](http://www.rebar3.org/)。

```text
curl -O https://s3.amazonaws.com/rebar3/rebar3
```

使用 chmod 使其可执行，然后将其添加到环境变量 PATH。

```text
chmod +x rebar3
export PATH=$PATH:your-current-directory
```

## 你的第一个 Erlang 应用程序

从命令 rebar3 new 开始，从名为 app 的内置模板生成一个新项目。在这个例子中，我们正在创建一个名为 myapp 的项目。其他可用的模板有：release，lib，plugin，escript，cmake。

```text
$ rebar3 new app myapp
===> Writing myapp/src/myapp_app.erl
===> Writing myapp/src/myapp_sup.erl
===> Writing myapp/src/myapp.app.src
===> Writing myapp/rebar.config
===> Writing myapp/.gitignore
===> Writing myapp/LICENSE
===> Writing myapp/README.md
```

包的代码放在 src 目录中。

```text
$ cd myapp
$ tree
.
├── LICENSE
├── README.md
├── rebar.config
└── src
    ├── myapp.app.src
    ├── myapp_app.erl
    └── myapp_sup.erl
```

惯例是有一个 .app.src 文件将您的应用程序定义为 OTP 应用程序，因为 Rebar3 只处理 OTP [结构化项目](http://www.erlang.org/doc/design_principles/applications.html)。看起来很熟悉？该文件也是 Erlang 。查看完整[参考](http://www.erlang.org/doc/design_principles/applications.html#id73836)，看看它可以包含什么。

```erlang
$ cat src/myapp.app.src 
{application, 'myapp',
 [{description, "An OTP application"},
  {vsn, "0.1.0"},
  {registered, []},
  {mod, {'myapp_app', []}},
  {applications,
   [kernel,
    stdlib
   ]},
  {env,[]},
  {modules, []}
 ]}.
```

src / myapp_app.erl 中的代码非常简单。它只是确保您可以启动和停止您的 Erlang 应用程序：

```erlang
$ cat src/myapp_app.erl
-module('myapp_app').
-behaviour(application).
-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    'myapp_sup': start_link().

stop(_State) ->
    ok.
```

Rebar3 使用名为 rebar.config 的文件来指定附加元数据，例如[依赖项](https://github.com/rebar/rebar/wiki/Dependency-management)。rebar.config 可以包含很多字段。要查看它们，请查看[完整的样本](https://github.com/rebar/rebar/blob/master/rebar.config.sample)。

```text
$ cat rebar.config 
{erl_opts, [debug_info]}.
{deps, []}.
```

现在让我们使用 Rebar3 启动一个 Erlang shell ，其中包含您的应用程序和路径中的依赖项。运行应用程序：启动（myapp）。验证您的应用是否已正确加载。

```erlang
$ rebar3 shell
===> Verifying dependencies...
===> Compiling myapp
Erlang R16B03-1 (erts-5.10.4) [source] [64-bit] [smp:8:8] [async-threads:0] [hipe] [kernel-poll:false]
Eshell V5.10.4  (abort with ^G)
1> application:start(myapp).
ok
2> application:stop(myapp). 
ok
3> 
=INFO REPORT==== 29-Jun-2015::16:14:10 ===
    application: myapp
    exited: stopped
    type: temporary
```

要了解命令 rebar3 shell，Fred Hebert（[Learn You Some Erlang](http://learnyousomeerlang.com/) 的作者）在[这里](http://ferd.ca/rebar3-shell.html)写了一篇很好的帖子。

## Erlang 包

### 在你开始之前

我们需要安装一个名为 [rebar3_hex](https://github.com/hexpm/rebar3_hex) 的插件，以便使用来自 [Hex.pm](https://hex.pm/)（Erlang / Elixir 包管理器）的获取和安装 Erlang 包。只需将以下行添加到 rebar.config 文件中即可。 （你需要 Erlang 版 OTP 17.4 及以上版本）

```text
{plugins, [rebar3_hex]}.
```

然后通过：rebar3 update 命令运行，以启用插件。

```text
$ rebar3 update
===> Fetching jsx ({pkg,<<"jsx">>,<<"2.6.1">>})
===> Fetching ssl_verify_hostname ({pkg,<<"ssl_verify_hostname">>,
                                           <<"1.0.5">>})
===> Fetching rebar3_hex ({pkg,<<"rebar3_hex">>,<<"0.6.0">>})
===> Compiling ssl_verify_hostname
===> Compiling jsx
===> Compiling rebar3_hex
===> Updating package index...
```

如果您想在每次创建新的 Erlang 应用程序时避免此步骤，请将该条目添加到全局 rebar.config 并将其放在：

```text
~/.config/rebar3/rebar.config
```

## 寻找 Erlang 包

使用 search 命令可以找到在 [Hex.pm](https://hex.pm/) 上发布的远程 Erlang 包。您可以在查询中使用正则表达式字符：

```text
$ rebar3 hex search cowboy
cloudi_service_http_cowboy
cowboy
```

## 安装包

Rebar3 可以下载并安装 Erlang 包和任何必要的依赖项。将应用程序名称添加到 rebar.config 文件中的 deps 条目，然后运行命令：rebar3 compile。在这个例子中，我们尝试使用两个名为 cowboy 和 meck 的Erlang 包。

```text
{deps, [cowboy, meck]}.
$ rebar3 compile
===> Verifying dependencies...
===> Fetching ranch ({pkg,<<"ranch">>,<<"1.0.0">>})
===> Fetching meck ({pkg,<<"meck">>,<<"0.8.2">>})
===> Fetching cowlib ({pkg,<<"cowlib">>,<<"1.0.1">>})
===> Fetching cowboy ({pkg,<<"cowboy">>,<<"1.0.0">>})
===> Compiling cowlib
===> Compiling ranch
===> Compiling meck
===> Compiling cowboy
===> Compiling myapp
```

想要安装特定版本的 Erlang 包吗？将应用程序名称和版本写入元组中。您可以在 [Hex 主页](https://hex.pm/)上浏览包的可用版本。

```text
{deps, [{cowboy, “1.0.2”}, {meck, "0.8.3"}]}.
```

## 列出已安装的包

rebar3 deps 命令显示本地安装的软件包：

```text
$ rebar3 deps
cowboy (locked package 1.0.0)
meck (locked package 0.8.2)
```

## 卸载包

要卸载软件包，首先必须将其从 rebar.config 文件中删除，然后使用命令：rebar3 unlock。在这里，我们从列表中删除了包 meck。

```text
$ rebar3 unlock
$ rebar3 deps
cowboy (locked package 1.0.0)
```

## 进一步阅读

http://www.rebar3.org/
