# tauri-comic-reader

基于 tauri+vite+vue3 开发的本地漫画阅读器
****

## 简介

学习开发tauri的练手项目，基于tauri的本地漫画库管理与漫画阅读器，可使用第三方图片查看器启动。

第三方启动器建议使用HoneyView。

****

## 更新日志

v 0.0.1

实现基础的管理并借助第三方启动器启动

v 0.2.0

* **重构**
  reload_library方法重做，详见main.rs

* **功能开发**
   1.
   增加版本更新时，（如果需要）更新数据库的功能
   2.
   删除源文件改为移动到回收站
   3.
   漫画查看页面提供不同的阅读类型，左右翻页，右左翻页，单页翻页
   翻页类型需要实现鼠标滚轮翻页
   4.
   漫画阅读页面设置进行数据储存，包括左右侧边栏的开关，页面阅读类型设置，页面比例
   5.
   阅读时获取当前的index，为右侧预览提供高亮显示
   优化高亮显示样式
   6.
   获取滚动阅读的index

****

## 编译与使用

目前已经发布v0.2.0版本，可以前往
<a href="https://github.com/tuip123/tauri-comic-reader/releases" target="_blank">releases</a>
页面下载

想要开发该阅读器你需要

1. 根据tauri的<a href="https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites" target="_blank">开始指南</a>
   完成环境配置
2. ```git clone```
3. ```cd tauri-comic-reader```
4. ```npm install```
5. 使用```npm run tauri dev```启动项目或```npm run tauri build```构建项目
