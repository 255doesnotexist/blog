---
title: 第二章：批处理系统 · rCore 2024S 随记 
date: 2024-04-29
description: OSComp 2024 随记 · 二
---

# 前言

实际实践参考的文档：[rCore Tutorial Guide 2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/)。

拥有更为详细细节介绍的 rCore OS 官方文档：[rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/0intro.html)。

# 第二章：应用程序与基本执行环境

## 引言

> 批处理系统 (Batch System) 出现于计算资源匮乏的年代，其核心思想是： 将多个程序打包到一起输入计算机；当一个程序运行结束后，计算机会 自动 执行下一个程序。

## 实践体验

> 本章我们引入了用户程序。为了将内核与应用解耦，我们将二者分成了两个仓库，分别是存放内核程序的 rCore-Tutorial-Code-20xxx （下称代码仓库，最后几位 x 表示学期）与存放用户程序的 rCore-Tutorial-Test-20xxx （下称测例仓库）。 你首先需要进入代码仓库文件夹并 clone 用户程序仓库（如果已经执行过该步骤则不需要再重复执行）：

```bassh
$ git clone https://github.com/LearningOS/rCore-Tutorial-Code-2024S.git
$ cd rCore-Tutorial-Code-2024S
$ git checkout ch2
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test-2024S.git user
```

> 上面的指令会将测例仓库克隆到代码仓库下并命名为 user ，注意 /user 在代码仓库的 .gitignore 文件中，因此不会出现 .git 文件夹嵌套的问题，并且你在代码仓库进行 checkout 操作时也不会影响测例仓库的内容。

> 在 qemu 模拟器上运行本章代码：

```bash
$ cd os
$ make run LOG=INFO
```