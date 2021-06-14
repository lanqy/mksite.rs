---
title: 使用 Erlang / OTP 构建您的第一个多用户聊天室应用程序
created: 2018/07/18
description: 使用 Erlang / OTP 构建您的第一个多用户聊天室应用程序
author: lanqy
---
# 使用 Erlang / OTP 构建您的第一个多用户聊天室应用程序

译自：https://medium.com/@kansi/chatbus-build-your-first-multi-user-chat-room-app-with-erlang-otp-b55f72064901

> 在本教程中，我将使用 Erlang / OTP和 ErlBus 构建另一个消息传递系统（稍后我们将讨论它），并在此过程中探索 Erlang 的强大功能！

注意：以下假设假定您对 Erlang / OTP 和 Web 开发有基本的了解。

本文的目的是构建一个多用户聊天室应用程序，但这个想法听起来太无聊了！所以让我们添加一些创造力并称之为 ChatBus。现在您可能想知道这个名字是如何帮助的。等一下 ！让我解释 ChatBus 的内容。我们将聊天室的概念抽象为公共汽车，聊天室中的所有用户都被称为搭便车，因为他们不需要为乘坐这辆公共汽车支付任何费用！

在我们潜入并开始编码像疯狗之前！我们应该决定 ChatBus 要具备哪些功能。这通常称为软件开发中的需求规范。在实施之前或者很快项目将开始分崩离析时，始终要根据经验确切知道要实施的内容！

对于我们的多用户聊天系统，我们将具有以下功能：

- 允许用户创建新的总线（也称为聊天室）！
- 允许用户从一个总线切换到另一个总线！ （这里有一些不可能的任务）
- 动态更新乘坐所选巴士的可用巴士和搭便车的列表。

这完成了基本规范。现在我们继续决定实现这个项目所需的工具和库。

## 工具和库

- [Polymer](https://www.polymer-project.org/1.0/)：对于这个应用程序，我使用 [Polymer](https://www.polymer-project.org/1.0/) 来开发客户端，但是你可以选择在Bootstrap中实现它或者你喜欢什么，因为讨论客户端不在讨论的范围内。
- [ErlBus](https://github.com/cabol/erlbus)：在 Erlang 中传递消息非常简单，但是这个项目更进一步，使它更容易。它允许用户创建频道并为其订阅侦听器进程。这就是我们实现 ChatBus 所需要的一切！下面是一个快速示例，以显示它的力量：

```erlang

% 创建匿名函数以执行监听频道的操作
% 请注意，该函数有两个参数，第一个是self
% 第一个是自解释的，第二个是上下文
% (ctx) 在生成这个函数时可以设置。

F = fun({Channel, Msg}, Ctx) ->
        io:format("[Pid: ~p] [Channel: ~p] [Msg: ~p] [Ctx: ~p]~n", [self(), Channel, Msg, Ctx])
    end.
% 将此函数派生为进程。
MH2 = ebus_handler:new(F, {my_ctx, <<"MH2">>}).

% 将生成的进程订阅到通道（ch1）
% 注意：如果通道不存在，则创建该通道。
ebus:sub(ch1, [MH2]).

% 让我们发一条消息给 'ch1'
ebus:pub(ch1, "Hello!").
[Pid: <0.52.0>] [Channel: ch1] [Msg: "Hello!"] [Ctx: {my_ctx, <<MH2>>}]


```

我希望这足以说明如何使用 ChatBus，但如果不能，你可以在他们的 [GitHub 项目](https://github.com/cabol/erlbus) 页面看一些例子。
- [Cowboy](https://github.com/ninenines/cowboy)：我们将使用 Cowboy 来实现我们的网络服务器。
- [Rebar3](https://www.rebar3.org/docs/getting-started)：我们将使用rebar3作为此项目的构建和发布工具。


...

## 实现

ChatBus 的项目源码位于[此处](https://github.com/beamX/chatBus/tree/feat-chatbus-client)。

首先，我们将使用rebar3生成模板项目：

```text
$ rebar3 new release chatbus
```

这将生成一个模板项目。执行以确保项目正常工作：

```text
$ rebar3 compile
$ rebar3 release
$ ./_build/default/rel/chatbus/bin/chatbus console
```

最后一个命令应该打开 erlang shell。该项目应如下所示

```text
├── apps
│   └── chatbus
│      └── src
│          ├── chatbus_app.erl
│          ├── chatbus.app.src
│          └── chatbus_sup.erl
├── _build
│   └── default
│       ├── lib
│       └── rel
├── config
│   ├── sys.config
│   └── vm.args
├── LICENSE
├── Makefile
├── README.md
├── rebar.config
└── rebar.loc
```

首先，我们添加项目所需的依赖项，将 rebar.config 更改为像[这样](https://github.com/beamX/chatBus/blob/feat-chatbus-client/rebar.config)。我们在那里有以下依赖：

- cowboy：用于创建允许客户端连接的 Web 服务器。
- lager：记录错误和东西。
- mochiweb：解析 json。
- erlbus：用于创建频道和传递消息。
- sync：用于开发目的。您可以选择排除它，但您必须相应地更改 chatbus.app.src。

现在我们使用 cowboy 创建我们的Web服务器。在你的chatbus_app.erl文件中，启动函数如下所示：

```erlang
start(_StartType, _StartArgs) ->

    ok = application:ensure_started(ebus),
 
   Dispatch = cowboy_router:compile(
                 [{'_', [
                         {"/", cowboy_static, {priv_file, chatbus, "index.html"}},
                         {"/ws", ws_handler, []},
                         {"/[...]", cowboy_static, {priv_dir, chatbus, "./"}}]}]),
    {ok, _} = cowboy:start_http(http, 100, [{port, 9090}], [{env, [{dispatch, Dispatch}]}]),
    'chatbus_sup':start_link().
```

这里我们添加了一些路由。我们的应用程序将是一个单页面的 Web 应用程序，因此我们定义静态文件 “index.html” 的位置，任何点击 “/” 的人都将获得此 “index.html” 和其他静态文件。接下来我们指定一个路由 “/ws”，我们将用它来建立一个 websocket 连接，对该端点的任何请求都将由 ws_handler.erl 模块处理。加载静态文件后，将使用 javascript 调用此 websocket 端点。最后，我们使用 “[...]” 定义所有静态文件的位置。

现在我们将创建一个 bus_listener.erl 模块，它将订阅一个频道，收听和广播消息，

```erlang
-module(bus_listener).
-behaviour(ebus_handler).

%% API
-export([handle_msg/2]).

handle_msg({_Channel, {Sender, Type, Msg}}, User) ->
    if
        Sender =:= User -> ok;
        true -> User ! {Type, Msg}
    end
```

如果你通过[这里](https://github.com/cabol/erlbus#my_handlererl)的文档，上面的模块应该很容易理解，但不过我将解释这段代码，但稍后。

现在我们将创建一个 websocket 处理程序，它将在调用端点 “/ws” 时创建 websocket 连接。回想一下，我们在路由中将处理程序名称指定为 “ws_handler”，因此我们使用以下内容创建名为 “[ws_handler.erl](https://github.com/beamX/chatBus/blob/feat-chatbus-client/apps/chatbus/src/ws_handler.erl)” 的模块，

```erlang

-module(ws_handler).
-export([init/2]).
-export([websocket_handle/3]).
-export([websocket_info/3]).
-export([websocket_terminate/3]).
-export([send_active_channels/1]).
init(Req, _Opts) ->
    io:format("connected !~n"),
 
    %% subscribe to default bus
    BusFd = ebus_handler:new(bus_listener, self()),
    ok = ebus:sub(default, [BusFd]),
    %% send subscribes bus name
    auto_send(<<"bus_subscribed">>, default),
 
    {cowboy_websocket, Req, #{bus => default
                             ,bus_fd => BusFd
                             ,hitchhicker => false}}.
websocket_handle({text, Msg}, Req, #{bus := BusName
                                    ,bus_fd := BusFd
                                    ,hitchhicker := Hitchhicker} = 
                                     State) ->
    {ok, {Type, Msg1}} = parse_message(Msg),
    case Type of
        <<"chat">> ->
            ok = ebus:pub(BusName, {self(), Type, Msg1}),
            {ok, Req, State};
         <<"bus_list">> ->
            {ok, List}  = bus_manager:bus_list(),
            {ok, Reply} = encode_message(<<"bus_list">>, List),
            {reply, {text, Reply}, Req, State};
         <<"hitchhicker_list">> ->
            {ok, List}  = bus_manager:get_hitchhickers(BusName),
            {ok, Reply} = encode_message(<<"hitchhicker_list">>, List),
            {reply, {text, Reply}, Req, State};
         <<"bus_subscribed">> ->
            BusName2 = erlang:binary_to_atom(Msg1, utf8),
            ok = ebus:unsub(BusName, BusFd),
            ok = ebus:sub(BusName2, [BusFd]),
            {ok, Reply} = encode_message(<<"bus_subscribed">>, BusName2),
            {reply, {text, Reply}, Req, State#{bus => BusName2}};
         <<"add_bus">> ->
            BusNewName = erlang:binary_to_atom(Msg1, utf8),
            ok = ebus:unsub(BusName, BusFd),
            ok = ebus:sub(BusNewName, [BusFd]),
            %% signal bus_manager to send all client list of 
            %% active buses
            bus_manager:check_bus(BusName),
            %% send message to client updating his bus
            {ok, Reply} = encode_message(<<"bus_subscribed">>, 
                                         BusNewName),
            {reply, {text, Reply}, Req, State#{bus => BusNewName}};
         <<"username">> ->
            %% check if username is assignable
            case bus_manager:store_username(BusName, Msg1) of
                {ok, error} ->
                    {ok, Reply} = encode_message( <<"username_error">>, error),
                    {reply, {text, Reply}, Req, State};
                 _ ->
                    {ok, List} = bus_manager:get_hitchhickers(BusName),
                     ok = ebus:pub(BusName, {none,
                                  <<"hitchhicker_list">>, List}),
                     {ok, Reply} = encode_message(<<"username">>, 
                                                  Msg1),
                     {reply, {text, Reply}, Req, 
                      State#{hitchhicker => Msg1}}
            end;
         <<"terminate">> ->
            bus_manager:remove_hitchhicker(Hitchhicker),
            ebus:unsub(BusName, BusFd),
            ebus_handler:delete(BusFd),
            {ok, List} = bus_manager:get_hitchhickers(BusName),
            ok = ebus:pub(BusName, {none, <<"hitchhicker_list">>, 
                          List}),
            {shutdown, Req, State};
         _ ->
            io:format("unknown message type ~p~n", [Type]),
            {ok, Req, State}
    end;
websocket_handle(Data, Req, State) ->
   io:format("received ~p~n", [Data]),
   {ok, Req, State}.
%% handle erlang messages
websocket_info({Type, Msg}, Req, State) ->
   {ok, Reply} = encode_message(Type, Msg),
   {reply, {text, Reply}, Req, State};
websocket_info(Info, Req, State) ->
   io:format("[ws_info]: unknown message ~p~n", [Info]),
   {ok, Req, State}.
websocket_terminate(_Reason, _Req, _State) ->
   io:format("[ws_info]: terminating websocket ~n"),
   ok.
%% ===============================================================
%% other exports
%% ===============================================================
send_active_channels(Channels) ->
   lists:map(fun(Bus) ->
               ok = ebus:pub(Bus, {none, <<"bus_list">>, Channels})
             end, Channels).
%% ===============================================================
%% internal functions
%% ===============================================================
auto_send(Mtype, Msg) ->
   %% send subscribes bus name
   timer:send_after(10, self(), {Mtype, Msg}).
parse_message(Msg) ->
   {struct, Msg1} = mochijson2:decode(Msg),
   {<<"type">>, Type} = lists:keyfind(<<"type">>, 1, Msg1),
   {<<"msg">>, Content} = lists:keyfind(<<"msg">>, 1, Msg1),
   {ok, {Type, Content}}.
encode_message(Type, Msg) ->
   Reply = {[{type, Type}, {msg, Msg}]},
   {ok, iolist_to_binary(mochijson2:encode(Reply))}.

```

当客户端调用 “/ws” 端点时，将通过调用 init/2 函数创建 websocket 。在这个功能中发生了两件重要的事情

```erlang

BusFd = ebus_handler:new(bus_listener, self()),
ok = ebus:sub(default, [BusFd]),

```

在第一行中，我们使用 bus_listener 模块生成一个新的 ebus 处理程序进程。然后，此处理程序进程订阅名为 “default” 的通道。

```erlang
handle_msg({_Channel, {Sender, Type, Msg}}, User) ->
    if
        Sender =:= User -> ok;
        true -> User ! {Type, Msg}
    end
```

handle_msg/2 的第一个参数是包含通道名称和消息的元组，第二个参数是我们在创建此过程时传递给 ebus_handler:new/2 的参数，即 self()。接下来，websocket_handle/3 函数处理来自客户端的数据。人们可以很容易地注意到客户端发送包含消息类型和消息的 json 对象。根据消息类型，我们执行不同的操作，例如。消息类型 “chat” 用于在通道上发送消息，该消息使用 ebus:pub/2 完成，还有其他消息类型执行不同的功能，如更改用户名，添加新聊天室，发送连接用户列表等。

...

通过上面的讨论，我试图介绍处理消息传递部分的 ChatBus 的基本代码库。我鼓励读者探索代码库并尝试使用它。

总之，使用 Erlang 构建聊天系统非常容易 :)
