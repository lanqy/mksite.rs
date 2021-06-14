---
title: HTML5 Canvas的命中区域检测以及如何侦听Canvas形状上的Click事件
description: HTML5 Canvas的命中区域检测以及如何侦听Canvas形状上的Click事件
created: 2020/12/09
author: lanqy
---

# HTML5 Canvas的命中区域检测以及如何侦听Canvas形状上的Click事件

您需要一个简单的 onClick 画布形状吗？ 但是 canvas 没有此类监听器的 API。 您只能在整个画布上监听事件，而不能在部分画布上监听事件。 我将描述两种主要方法来解决此问题。

让我们从简单的 html5 canvas 图形开始。假设我们要在页面上绘制几个圆圈。

```js
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

// create circles to draw
const circles = [
  {
    x: 40,
    y: 40,
    radius: 10,
    color: 'rgb(255,0,0)'
  },
  {
    x: 70,
    y: 70,
    radius: 10,
    color: 'rgb(0,255,0)'
  }
];

// draw circles
circles.forEach(circle => {
  ctx.beginPath();
  ctx.arc(circle.x, circle.y, circle.radius, 0, 2 * Math.PI, false);
  ctx.fillStyle = circle.color;
  ctx.fill();
});
```
查看 demo http://codepen.io/lavrton/pen/QdePBY

现在，我们可以简单地聆听整个画布上的点击：

```js
canvas.addEventListener('click', () => {
   console.log('canvas click');
});
```

但是如果我们想监听一个圆圈上的点击。这该怎么做？如何检测到我们点击了一个圆圈？

## 方法＃1 - 利用数学的力量

有了有关圆的坐标和大小的信息后，我们可以简单地使用数学方法通过简单的计算来检测圆的点击。我们需要做的就是从click事件中获取鼠标位置，并检查所有圆的交点：

```js
function isIntersect(point, circle) {
  return Math.sqrt((point.x-circle.x) ** 2 + (point.y - circle.y) ** 2) < circle.radius;
}

canvas.addEventListener('click', (e) => {
  const pos = {
    x: e.clientX,
    y: e.clientY
  };
  circles.forEach(circle => {
    if (isIntersect(mousePoint, circle)) {
      alert('click on circle: ' + circle.id);
    }
  });
});
```

这种方法非常普遍，并在许多项目中广泛使用。您可以轻松找到更复杂形状的数学函数，例如矩形，椭圆形，多边形等。

这种方法非常好，如果您没有大量的形状，则可以超快。

但是很难将这种方法用于非常复杂的形状。例如，您正在使用具有二次曲线的线。

## 方法＃2 - 模拟点击区域

命中区域的想法很简单-我们只需要将像素放在点击区域下方，然后找到具有相同颜色的形状即可：

```js
function hasSameColor(color, circle) {
  return circle.color === color;
}

canvas.addEventListener('click', (e) => {
  const mousePos = {
    x: e.clientX - canvas.offsetLeft,
    y: e.clientY - canvas.offsetTop
  };
  // get pixel under cursor
  const pixel = ctx.getImageData(mousePos.x, mousePos.y, 1, 1).data;

  // create rgb color for that pixel
  const color = `rgb(${pixel[0]},${pixel[1]},${pixel[2]})`;

  // find a circle with the same colour
  circles.forEach(circle => {
    if (hasSameColor(color, circle)) {
      alert('click on circle: ' + circle.id);
    }
  });
 });
```

但是正是这种方法行不通，因为它可能具有相同颜色的形状，对吗？为了避免这种冲突，我们应该创建一个特殊的“命中图”画布。它将具有几乎相同的形状，但是每个形状将具有独特的颜色。因此，我们需要为每个圆生成随机颜色：

```js
// colorsHash for saving references of all created circles
const colorsHash = {};

function getRandomColor() {
const r = Math.round(Math.random() * 255);
const g = Math.round(Math.random() * 255);
const b = Math.round(Math.random() * 255);
return `rgb(${r},${g},${b})`;
}



const circles = [{
 id: '1', x: 40, y: 40, radius: 10, color: 'rgb(255,0,0)'
}, {
 id: '2', x: 100, y: 70, radius: 10, color: 'rgb(0,255,0)'
}];

// generate unique colors
circles.forEach(circle => {
 // repeat until we find trully unique colour
 while(true) {
    const colorKey = getRandomColor();
    // if colours is unique
    if (!colorsHash[colorKey]) {
       // set color for hit canvas
       circle.colorKey = colorKey;
       // save reference 
       colorsHash[colorKey] = circle;
       return;
    }
 }
});
```

之后，我们需要绘制每个形状两次。首先在可见的画布上，然后在“命中”画布上。

```js
circles.forEach(circle => {
  // draw on "scene" canvas first
  ctx.beginPath();
  ctx.arc(circle.x, circle.y, circle.radius, 0, 2 * Math.PI, false);
  ctx.fillStyle = circle.color;
  ctx.fill();
  
  // then draw on offscren "hit" canvas
  hitCtx.beginPath();
  hitCtx.arc(circle.x, circle.y, circle.radius, 0, 2 * Math.PI, false);
  hitCtx.fillStyle = circle.colorKey;
  hitCtx.fill();
});
```

现在，当您在画布上单击时，您需要的是在命中的画布上获取一个像素并找到具有相同颜色的形状。而且此操作非常快速，您无需遍历所有形状。另外，形状的复杂程度也无关紧要。绘制任何您想要的颜色，并对每个形状使用不同的颜色。

查看完整 demo ：http://codepen.io/lavrton/pen/OWKYMr

## 哪种方法更好？

这取决于。第二种“命中”方法的主要瓶颈是必须绘制两次形状。因此性能可能会下降两次！但是在热门画布上绘画可能会更简单。您可以在那里跳过阴影和笔触，可以简化某些形状，例如，仅用矩形替换文本。但是在完成绘制之后，这种方法可以超快。因为获取像素并访问存储的形状的散列是非常快的操作。

## 它们可以一起使用吗？

当然。一些画布库使用这种混合方法。

它以这种方式工作：

对于每种形状，您必须计算简化的边界矩形（x，y，宽度，高度）。然后，您可以使用第一种“数学”方法来过滤与鼠标位置和边界矩形相交的形状。之后，您可以使用第二种方法绘制命中并测试相交，以获得更准确的结果。

## 为什么不只在这种情况下使用SVG？

因为有时候画布可以更高性能，并且更适合您的高级任务。同样，这取决于任务。所以canvas vs SVG不在本文的讨论范围之内。如果您想使用画布并进行命中检测，则必须使用某些工具，对吗？

## 那其他事件呢？像 mousemove，mouseenter 等？

您只需要在描述的方法中添加一些额外的代码即可。一旦您可以100％检测到鼠标下方的形状，就可以模拟所有其他事件。

## 有什么好的即用型解决方案吗？

当然。只需尝试使用 Google “ html5 canvas框架” 即可。但是我个人的建议是 http://konvajs.github.io/。对了，我是该库的维护者。Konva 仅使用第二种方法，它支持我们通常对DOM元素具有的所有鼠标和触摸事件（甚至更多，例如拖放）

翻译自： https://lavrton.com/hit-region-detection-for-html5-canvas-and-how-to-listen-to-click-events-on-canvas-shapes-815034d7e9f8/