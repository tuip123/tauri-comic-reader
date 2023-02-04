# tauri-comic-reader
基于 tauri+vite+vue3 开发的本地漫画阅读器
****
## 简介
学习开发tauri的练手项目，基于tauri的本地漫画库管理与漫画阅读器，可使用第三方图片查看器启动。

第三方启动器建议使用HoneyView。
****
## 更新日志
v 0.0.1

实现基础的管理与第三方启动器启动
****
## TODO
v 0.1.0
* **页面开发**
  1. 设置页面
  2. 漫画书架
  3. 漫画阅读
* **系统适配**
  1. macOS
  2. Linux（ArchLinux 与 Ubuntu）
* **发布release**

v 0.2.0
* **功能开发**
1. 重构部分rust方法
2. 增加版本更新时，（如果需要）更新数据库的功能

v 0.3.0
~~还早呢~~
* **~~功能开发~~**
1. ~~检查更新~~
2. ~~自动更新~~

****
## 编译与使用
***请注意，项目目前并不稳定，可能无法达到预期效果***

项目目前尚未完善，并未发布release

想要使用该阅读器你需要
1. 根据tauri的<a href="https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites" target="_blank">开始指南</a>完成环境配置
2. ```git clone```
3. ```npm i```
4. 使用```npm run tauri dev```启动项目或```npm run tauri build```构建项目