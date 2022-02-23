import { createRouter, createWebHashHistory } from "vue-router";

import Home from "../pages/home.vue";
import About from "../pages/about.vue";
import Rate from "../pages/Rate.vue";
import TodolistVue from "../components/Todolist.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/about",
    name: "About",
    component: About,
  },
  {
    path: "/rate",
    name: "Rate",
    component: Rate,
  },
  {
    path: "/todo",
    name: "Todolist",
    component: TodolistVue,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
