```
npm init @vitejs/app
npm install vue-router@4
npm install pinia
npm i -D axios # 网络请求库
npm i -D eslint/tslint # 代码风格约束规范 npm eslint --init


npm i -D sass # 项目中集成CSS预编译期(帮助我们更快更高效的编写CSS代码),我们选择SASS

## 集成框架
npm i element-plus --save

# 把开发中常见的属性 都封装为了响应式函数, 类似react的Hooks
npm i @vueuse/core

* 现代Web应用由三大组件构成: 组件、数据、路由

```

## Vue 的开发核心

Vite + Vue3 + Vue Router + Pinia + Element3 + Axios + Sass
eslint: 代码规范约束 (npm eslint --init, npm eslint src)
husky: git 钩子函数, 执行 git commit 命令的时候 (单元测试、eslint)

    * 以数据驱动, 以响应式的数据为驱动

    ## 项目组件
    * 导航
    * 侧边栏
    * 表格
    * 弹窗

## Chrome DevTools

- Element: HTML CSS
- Console: JavaScript
- Source: 源码
- Application: 本地存储和一些浏览器服务: Cookie, Localstorage, 通知等
- Network: 网络求情
- Performance: 调试网页性能
- Lighthouse: 网页性能报告
- Vue Devtools: Vue 调试插件

* more tools

- Animation: 调试动画
- Security: 安全性

## 权限系统

- 登录权限
- 页面权限
- 角色权限

后端 casbin

- 1 问. Cookie 和 Token
  Cookie 是由后端设置 SetCookie 设置 header, 浏览器会自动存储 cookie，然后当前域名在发送请求的时候，都会自动带上这个 cookie。

但是现在，前后端分离的场景下，通常后端项目会部署在不同的机器和服务器之上，Cookie 再跨域上有诸多限制，所以再这种场景下，我们更愿意手动去管理权限, 于是就诞生了基于 token 的权限解决方案,
把 token 理解为手动管理的 cookie.

- 后端实现 RBAC，用户登录以后，后端会返回页面的权限数据(动态权限数据)
- 这样页面分为两种，静态部分写在 src/router/index.js 中
- 动态部分通过 axios 获取，再通过 vue-router 的 addRoute 方法动态添加到路由配置中

mixin extends 尽量不要用
全局变量少用(app.config.globalProperties.x)， 吧所有的全局变量放到一个文件中管理

```
npm init vite@latest
npm i -D husky # git commit hook
npx husky install
npx husky add .husky/commit-msg "node scripts/verifyCommit.ts" # commit 的后置钩子 commit格式校验等
npx husky add .husky/commit-msg "node run lint" ## commit执行的前置钩子


# 测试相关
npm i -D jest@26 vue-jest@next @vue/test-util@next # 测试库
npm i -D babel-jest@26 @babel/core @babel/prset-any # babel 相关库
npm i -D ts-jest@26 @babel/preset-typesript @types/jest ## jest 适配typescript的库

# preset: 预设
```

## 开始

社区模板
npm install -g degit

npx degit nekobc1998923/vitecamp example-admin
