---
title: 一步一步教你创建 react redux 应用
created: 2018/05/21
description: 一步一步教你创建 react redux 应用
author: lanqy
---
# 一步一步教你创建 react redux 应用

Redux 已经成为构建React应用的标配，有大量的实例展示它是怎样做到的，但是react-Redux 应用有很多部分，例如：“Reducers”, “Actions”, “Action Creators”, “State”, “Middleware” 等等)，这些都具有颠覆性的。

当我开始学习它的时候，我找不到任何资料说明“构建react-Redux应用的时候，应该先构建哪个部分”或者一般怎样构建react－redux应用，因此我通过几个实例和文章来教你怎样一步步构建react－redux应用。
<br/>

注：我通过“模拟”以保持较高水平和不至于太杂乱。我将使用经典的<a href="https://github.com/reactjs/redux/tree/master/examples/todos">Todo list app</a>作为基础来创建任何react应用。如果你的应用有多个（screens）界面，只需要重复这个步骤到你到界面中即可。

### 为什么要用Redux?

React—一个JS库，帮助我们将我们的应用拆分为多个组件，但是没有具体说明如何保持数据（`state`）的轨迹以及如何正常处理我们的所有事件（`Actions`）。

Redux—一个很受欢迎的库，为react提供一个可以轻松地保持数据（`state`）和事件（`actions`）。

> 实际上，Redux允许我们以我们的方式创建React应用，然后委托所有的状态（state）和事件（actions）到Redux上。
> 顺便说一个.一个简单到Todo应用，有八个步骤。理论上，早期的框架构建Todo应用很简单，但是做真实的应用比较难（这里指的是维护）。而React Redux做Todo应用就显得比较繁琐，但是真实的应用就比较方便（这里指的是维护）

让我们开始吧！

### 第一步－写一个详细的模拟

这个模拟应该包含所有的数据和视觉效果（例如TodoItem的删除线，一个“All”的过滤器），

第一步：模拟每个界面，

怎样模拟：

1、越详细越好，

2、画出所有的数据和视觉效果（例如item删除线）。

如下图：

<img src="/images/1-RtA4NF2PI__vcarQgXEEBg.png" />

### 把应用划分成为组件

尝试将应用划分为基于每个组件的用途的组件块。

我们有三个组件“`AddTodo`”, “`TodoList`” and “`Filter`”组件。

第二步：划分应用成为组件：

怎样划分：基于它们的用途来划分。

我们的应用有三个主要的“目的”：

1、添加一个新的Todo 项目（AddTodo组件）

2、展示Todo列表（TodoList组件）

3、过滤展示Todos（Filter组件）

如下图：

<img src="/images/1-AfzFa8zO_dQOmuSHL7bEww.png" />

### Redux术语：“`Actions`”和“`States`”

##### 每个组件做这两件事情：

 1、根据一些数据渲染DOM，这些数据，我们称之为状态（`state`）。

 2、监听用户和事件并发送到一个JS方法，我们称之为操作（`actions`）。

### 第三步－列出每个组件的`States`和`Actions`

我们通过第二步来仔细看看每一个组件，并列出它们每个的`States`和`Actions`。

我们有三个组件，分别为“`AddTodo`”, “`TodoList`” and “`Filter`”组件，让我们列出它们每个的`States`和`Actions`。

* 3.1 AddTodo 组件的`State` 和 `Actions`

这个部分，我们没有任何状态（`state`），这个组件不依赖任何数据或状态来改变外观，但是它需要让其它组件知道，当用户创建一个新的Todo的时候。让我们叫这个action为`ADD_TODO`


AddTodo组件
 State：
 1.(一个简单的输入框和一个按钮，不依赖于任何数据来展示)
 Actions（events）：
 1.AddTodo组件允许我们创建一个新的Todo项目通过监听DOM事件和从输入框获取数据。这个事件映射到JSON对象，我们称之为Action。

 在这个案例中，我们可以通过创建一个JSON对象来描述我们的AddTodo action,JSON如下：

```javascript
 {
  type:'ADD_TODO'
  payload:{
    data:'Learn Redux',
    id:1,
    completed:false
  }
 }

```
如下图：

<img src="/images/1-OrfmEw_gPw5kQ3ZO5AjzEQ.png" />

* 3.2 TodoList 组件的`State` 和 `Actions`

TodoList组件需要一个数组来展示数据，因此它需要一个状态（`state`）,我们称之为Todos（数组）。它也需要知道哪一个过滤器被触发，以相应的显示或隐藏Todo 项目，因此还需要一个状态（`state`）,让我们称之为`VisibilityFilter`（布尔值）。

进一步，它允许我们切换Todo项目为已完成或未完成状态。我们也需要让其它组件知道切换这个状态。我们把这个action叫做“`TOGGLE_TODO`”

TodoList组件：

State：

1、Todos数组

2、Visibility filter（显示过滤器），这个告诉那种类型的Todo项目需要显示或者隐藏。注：这个来自于“Filter”组件。

Actions（events）：

TodoList组件只有一个action，它允许用户切换Todo项目的完成状态，当用户点击到每个Todo项目时，它需要告诉其它到组件和数据库等切换Todo的状态。

我们需要通过JSON对象来描述我们的ToggleTodo action,JSON如下：

`
{
 type:"TOGGLE_TODO"
 payload:{
  id:<todoItem's id >
 }
}
`

如下图：

<img src="/images/1-waJ9BucAT9qW_sHcqNOB_Q.png" />

* 3.3 Filter 组件的`State` 和 `Actions`

Filter组件可以是一个链接或者一个简单的文本来渲染，但它依赖于是否激活，我们把这个状态（`state`）称之为“`CurrentFilter`”。

Filter组件依然需要告诉其它组件用户什么时候点击它，我们把这个操作(`action`)称之为“`SET_VIBILITY_FILTER`”。


Filter组件：

State(data)：

1.CurrentFilter－一个字符串，拥有告诉哪个过滤器需要显示，哪个链接可点击，例如“Active”和“completed”

Actions（events）：

Filter组件也只有一个操作（action），它只是监听用户点击过滤器点链接并告诉其它组件哪个链接已经被点击。

我们通过一个JSON对象来描述`SET_VIBILITY_FILTER`操作（action），JSON如下：

{
 type:"SET_VIBILITY_FILTER"
 payload:{
 filter:"Completed"
 }
}

如下图：

<img src="/images/1-3G9ycPVPovZPu35rGW3FLw.png" />

### Redux术语“```Action Creators```”

`Action Creators`是一些简单的方法用于从DOM事件中接受数据，格式化为标准的JSON“`Action`”。对象并返回这个对象（又称`Action`），这个帮助我们正规化我们的数据的样子。

进一步，它允许未来任何组件发送到这些Actions到其它组件去（又成为`dispatch`［调度］）。

### 第四步，为每个`Action`创建`Action Creators`

我们统计一共有三个`Action`:`ADD_TODO, TOGGLE_TODO and SET_VISIBILITY_FILTER`。让我们为它们每个创建`Action`:

```javascript
//1. 从AddTodo文本域获取文本并返回正确的“Action” JSON对象发送到其它到组件中。（Takes the text from AddTodo field and returns proper “Action” JSON to send to other components.）
export const addTodo = (text) => {
 return {
 type: ‘ADD_TODO’,
 id: nextTodoId++,
 text,  //<--ES6. same as text:text, in ES5（这个是ES6的写法，相当于ES5的text:text）
 completed: false //<-- initially this is set to false (completed字段初始化为false)
 }
}

//2. 获取filter 字符串并返回正确的“Action” JSON对象发送到其它组件中。（Takes filter string and returns proper “Action” JSON object to send to other components.）
export const setVisibilityFilter = (filter) => {
 return {
 type: ‘SET_VISIBILITY_FILTER’,
 filter
 }
}

//3. 获取Todo 项目的id并返回正确的“Action” JSON对象发送到其它组件中。（Takes Todo item’s id and returns proper “Action” JSON object to send to other components.）
export const toggleTodo = (id) => {
 return {
 type: ‘TOGGLE_TODO’,
 id
 }
}
```

### Redux术语“`Reducers`”

`Reducers`是一些方法，用于从`redux`和“`action`” JSON对象中获取状态（`state`）并返回新的状态（`state`）存回Redux中。

1、当用户操作时，Reducers方法会被“`Container`”触发［这句好难翻啊，估计翻不对］（Reducer functions are called by the “Container” containers when there is a user action）。

2、如果`reducer```改变了状态（`state`），Redux传递新的状态（new state）到每个组件并且React重新渲染每个组件。

```javascript
//(For example the below function takes Redux’ state(an array of previous todos), and returns a **new** array of todos(new state) w/ the new Todo added if action’s type is “ADD_TODO”.)
// 例如下面这个方法获取Redux 状态（一个之前的todos数组），并返回一个新的todos数组（new state），当action's的类型为“ADD_TODO”时，一个新的Todo就会被添加
const todo = (state = [], action) => {
 switch (action.type) {
  case ‘ADD_TODO’:
     return
       […state,{id: action.id, text: action.text, completed:false}];
 }
```

### 第五步，为每个`Action`写`Reducers`

注：为简洁起见，一些代码已经被简化了,另外我简单当展示一下`SET_VISIBILITY_FILTER`、`ADD_TODO` 和 `TOGGLE_TODO` ，代码如下：

```javascript
const todo = (state, action) => {
  switch (action.type) {
     case ‘ADD_TODO’:
      return […state,{id: action.id, text: action.text,
              completed:false}]

     case ‘TOGGLE_TODO’:
        return state.map(todo =>
                if (todo.id !== action.id) {
                  return todo
                }
                 return Object.assign({},
                    todo, {completed: !todo.completed})
            )

      case ‘SET_VISIBILITY_FILTER’: {
       return action.filter
      }

     default:
      return state
    }
}
```
### Redux术语“`Presentational`”和“`Container`”组件

把React和Redux的逻辑放在每个组件里是很混乱的，因此Redux建议创建一个假的仅仅包含组件的展示型组件和一个父级包装组件就做容器型组件负责Redux的`Actions`调度等等。

然后父级容器将数据传递到展示型组件，事件处理替展示型组件处理React，如下图：

<img src="/images/1-inU9OmAFSDYKFm8pstsCDw.png" />

说明：黄色点线代表展示型组件，黑色点线表示容器型组件。

###第六步－实现所有的展示型组件

现在是时候实现我们所有的（3个）展示型组件了

6.1 实现AddTodoForm组件

```
AddTodoForm组件：
这个组件渲染一个输入框和一个按钮，它接收一个onSubmit回调方法（来自父级容器组件）当用户提交一个新的Todo时，它发送和传递todo文本到onSubmit方法中
```
```javascript
let AddTodoForm = ({onSubmit}) => {
 let input
 return(
  <div>
   <form onSubmit={e=>{onSubmit(input.value)}}>
    <input ref={node = >{input = node}} />
    <button type="submit">Add Todo</button>
   </form>
  </div>
 )
}
export default AddTodoForm
```
如下图：

<img src="/images/1-WlASUkXRWSGaFZdZJXXiRQ.png" />

6.2 实现TodoList组件

TodoList组件:

这个组件渲染一个Todo列表，它接受`Todos`数组和一个`onTodoClick`回调方法（来自父级容器组件）。当用户点击每个Todo项目时，它发送Todo项

目的id到`onTodoClick`方法中并切换这个被点击的项目的状态。

```javascript
const TodoList = ({todos,onTodoClick})=>{
if(todos.length === 0)
 return <div>Add Todos</div>
return <ul>
{todos.map(todo=>
 <Todo key={todo.id}{...todo} onClick={()=>onTodoClick(todo.id)} />
)}
</ul>
}

export default TodoList
```
如下图：

<img src="/images/1-u1CX5abgafgbt3x-IgJ36A.png" />

6.2 实现Link组件


Link组件：

这个组件渲染个别链接，分别有三个链接，它接收“active”布尔值并渲染一个文本或者链接，它接收children属性来显示链接的名称，它也接收一

个onClick回调方法，当链接被点击的时候会被调用。

```javascript
const Link = ({active,children,onClick}) = >{
   if(active){
     return <span>{children}</span>
   }
   return <a href="#" onClick={e =>{onClick()}}>{children}</a>
}
export default Link
```
如下图：
<img src="/images/1-5WQbEnAhRP6fmfiCjOeS4A.png" />

注意：在实际的代码中，Link展示组件被包含在`FilterLink`容器组件中，进而3个FilterLink组件显示在Footer展示组件中。

### 第七步－为部分或者全部展示组件创建容器组件

最后，为每个组件创建Redux链接

7.1创建容器组件（AddTodo组件）

<a href="https://github.com/rajaraodv/redux/blob/master/examples/todos/containers/AddTodo.js">最后实际的代码在这里</a>

<img src="/images/1-ElBMxdAUzuVJ343uAlCVmA.png" />

7.2创建容器组件（TodoList组件）

<a href="https://github.com/rajaraodv/redux/blob/master/examples/todos/containers/VisibleTodoList.js">最后实际的代码在这里</a>

<img src="/images/1-malT38rul36L0Ygbt1JjzA.png" />

7.3创建容器组件（Filter组件）

<a href="https://github.com/rajaraodv/redux/blob/master/examples/todos/containers/FilterLink.js">最后实际的代码在这里</a>

<img src="/images/1-1Kgo8pIxbLAkuBho6aQQeQ.png" />

注意：在实际的代码中，Link展示组件被包含在`FilterLink`容器组件中，进而3个FilterLink组件排列和显示在Footer展示组件中。

###第八步－把它们放在一起

```javascript
import React from ‘react’ // ← Main React library
import { render } from ‘react-dom’ // ← Main react library
import { Provider } from ‘react-redux’ //← Bridge React and Redux
import { createStore } from ‘redux’ // ← Main Redux library
import todoApp from ‘./reducers’ // ← List of Reducers we created
//Import all components we created earlier
import AddTodo from ‘../containers/AddTodo’
import VisibleTodoList from ‘../containers/VisibleTodoList’
import Footer from ‘./Footer’ // ← （这个是一个展示型组件，包含三个FilterLink容器组件）This is a presentational component that contains 3 FilterLink Container comp
//Create Redux Store by passing it the reducers we created earlier.(创建Redux store树，传入reducers)
let store = createStore(reducers)
render(
 <Provider store={store}> //← （react-redux的提供组件，用于将store注入到所有到子组件中）The Provider component from react-redux injects the store to all the child components
 <div>
 <AddTodo />
 <VisibleTodoList />
 <Footer />
 </div>
 </Provider>,
 document.getElementById(‘root’) //<-- Render to a div w/ id "root"
)
```
就是这样！

注：本文翻译自<a href="https://medium.com/@rajaraodv/step-by-step-guide-to-building-react-redux-apps-using-mocks-48ca0f47f9a#.kljg6fuei">Step by Step Guide To Building React Redux Apps</a>，有翻译不对的地方，欢迎指正，谢谢！
