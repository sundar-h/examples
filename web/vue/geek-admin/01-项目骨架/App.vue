<template>
    <!-- router-link 负责跳转页面 -->
      <router-link to="/">首页</router-link> |
      <router-link to="/about">关于</router-link>

    <!-- router-view 负责渲染路由匹配的组件 -->
    <router-view></router-view>
</template>