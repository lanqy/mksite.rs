---
title: git 常用命令
description: git 常用命令
created: 2019/12/27
author: lanqy
---

#  git 常用命令

## 解决冲突：

```text
git stash
git pull origin develop
git stash pop
```
## 切换分支：

```text
// 切换到 develop 分支
git checkout -b develop 
```
##  删除分支 

```text
git branch -d feature_x
```
##  合并分支到你当前的分支

```text
git merge <branch>
```
##  查看两个分支的差异

```text
git diff <source_branch> <target_branch>
```
## 提交流程：

```text
git add *

git commit -m "提交备注"

git push origin develop
```
## Git回滚代码到某个commit

```text
$ git reset --hard HEAD^        // 回退到上个版本
$ git reset --hard HEAD~3       // 回退到前3次提交之前，以此类推，回退到n次提交之前
$ git reset --hard commit_id    // 退到/进到 指定commit的sha码
```
## 删除某个标签

```text
git push origin :refs/tags/v2.7.72 // 删除标签 v2.7.72 
```
