---
title: 使用 Gruntjs 压缩合并前端静态资源（图片、JavaScript 和 CSS ）
description: 使用 Gruntjs 压缩合并前端静态资源（图片、JavaScript 和 CSS ）
created: 2018/05/21
author: lanqy
---

# 使用 Gruntjs 压缩合并前端静态资源（图片、JavaScript 和 CSS ）
### 安装环境：
#### Nodejs：https://nodejs.org/en/ 安装最新版本
#### 进入目录（cd projectName）
#### 新建package.json 内容如下：
```js
{
  "name": "my-project-name",
  "version": "0.1.0",
  "devDependencies": {
    "grunt": "^0.4.5",
    "grunt-contrib-concat": "^1.0.0",
    "grunt-contrib-cssmin": "^0.12.3",
    "grunt-contrib-imagemin": "^1.0.0",
    "grunt-contrib-jshint": "^0.12.0",
    "grunt-contrib-nodeunit": "~0.4.1",
    "grunt-contrib-uglify": "^0.5.1"
  }
}
```
#### 命令行执行 ``` npm install  ```
#### 接着安装grunt命令行工具 ``` npm install -g grunt-cli```
#### 接着配置Gruntfile.js文件
```js
module.exports = function(grunt) {

    grunt.initConfig({

        //our JSHint options
        jshint: {
            all: ['main.js'] //files to lint
        },
        //our concat options
        concat: {
            options: {
                separator: ';' //separates scripts
            },
            dist: {
                src: ['html5/javascripts/jquery.min.js', 'html5/javascripts/dialog.js', 'html5/javascripts/utils.js','html5/javascripts/limit.js','html5/javascripts/pageScrollAjax.js','html5/javascripts/app.js','html5/javascripts/tabs.js','html5/javascripts/swiper.min.js'], //需要合并的文件，注意顺序
                dest: 'html5/javascripts/build.js' //合并后生成的文件
            }
        },

        //压缩js
        uglify: {
            js: {
                files: {
                    'html5/javascripts/build.min.js': ['html5/javascripts/build.js'] //合并后压缩
                }
            }
        },
        cssmin: {
         options: {
             keepSpecialComments: 0
         },
         compress: {
             files: { // css压缩app.min.css为压缩后生成的文件,后面的数组为需要合并压缩的文件（同样注意顺序）
                 'html5/stylesheets/app.min.css': [
                     "html5/stylesheets/common.css",
                     "html5/stylesheets/swiper.min.css",
                     "html5/stylesheets/dialog.css",
                     "html5/stylesheets/app.css",
                     "html5/stylesheets/new.css",
                     "html5/stylesheets/app-type.css"
                 ]
             }
         }
     },

     imagemin: {
            /* 压缩图片大小 */
            dist: {
                options: {
                    optimizationLevel: 3 //定义 PNG 图片优化水平
                },
                files: [{
                    expand: true,
                    cwd: 'images/html5',
                    src: ['**/*.{png,jpg,jpeg}'], // 优化 img 目录下所有 png/jpg/jpeg 图片
                    dest: 'images/html5' // 优化后的图片保存位置，覆盖旧图片，并且不作提示（建议新建一个目录）
                }]
            }
        }

    });
    //加载任务
    grunt.loadNpmTasks('grunt-contrib-jshint');
    grunt.loadNpmTasks('grunt-contrib-concat');
    grunt.loadNpmTasks('grunt-contrib-uglify');
    grunt.loadNpmTasks('grunt-contrib-cssmin');
    grunt.loadNpmTasks('grunt-contrib-imagemin');

    // default tasks to run
    // grunt.registerTask('default', ['jshint', 'concat', 'uglify']);
    grunt.registerTask('development', ['jshint']);
    grunt.registerTask('production', ['jshint', 'concat', 'uglify','imagemin','cssmin']);
  }


```

1.检查js语法```grunt jshint```

2.合并js ```grunt concat```

3.压缩js```grunt uglify```

4.压缩css```grunt imagemin```

5.压缩图片```grunt cssmin```

（注意js压缩之前要先合并，也就是第一步和第二部有先后顺序）

6.批量处理 ```grunt production```


### 相关链接：
#### Gruntjs 批量无损压缩图片大小：
#### https://www.zfanw.com/blog/gruntjs-optimize-image-size-loseless.html 
#### 前端js和css的压缩合并之grunt：
#### http://www.haorooms.com/post/qd_grunt_cssjs 
#### grunt官网：
#### http://gruntjs.com/getting-started 
