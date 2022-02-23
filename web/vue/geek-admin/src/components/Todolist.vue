<template>
  <input type="text" v-model="title" @keydown.enter="addTodo" />
  <button v-if="active < all" @click="clear">清理</button>
  <ul v-if="todos.length">
    <li v-for="todo in todos">
      <input type="checkbox" v-model="todo.done" />
      <span :class="{ done: todo.done }"> {{ todo.title }} </span>
    </li>
  </ul>
  <div v-else>暂无数据</div>
  <div>
    全选<input type="checkbox" v-model="allDone" />
    <span>{{ active }} / {{ all }}</span>
  </div>
  <transition name="modal">
    <div class="info-wrapper" v-if="showModal">
      <div class="info">输入为空!!</div>
    </div>
  </transition>
</template>

<script setup>
import { ref, computed, watchEffect } from "vue";
import useStorage from "../utils/useStorage";
let showModal = ref(false);
let title = ref("");
let todos = useStorage("todos", []);

// watchEffect 监听reactive的数据变化
watchEffect(() => {
  localStorage.setItem("todos", JSON.stringify(todos.value));
});

function addTodo() {
  if (!title.value) {
    showModal.value = true;
    setTimeout(() => {
      showModal.value = false;
    }, 1500);
    return;
  }
  todos.value.push({
    title: title.value,
    done: false,
  });
  title.value = "";
}

function clear() {
  todos.value = todos.value.filter((todo) => !todo.done);
}

let active = computed(() => {
  return todos.value.filter((todo) => !todo.done).length;
});

let all = computed(() => {
  return todos.value.length;
});

let allDone = computed({
  get: function () {
    return active.value === 0;
  },
  set: function (value) {
    todos.value.forEach((todo) => {
      todo.done = value;
    });
  },
});
</script>

<style>
.done {
  color: gray;
  text-decoration: line-through;
}
.modal-enter-from {
  opacity: 0;
  transform: translateY(-60px);
}
.modal-enter-active {
  transform: all 0.3 ease;
}
.modal-leave-to {
  opacity: 0;
  transform: translateY(-60px);
}
.modal-leave-active {
  transform: all 0.3s ease;
}
.info-wrapper {
  position: fixed;
  top: 20px;
  width: 200px;
}
.info {
  padding: 20px;
  color: white;
  background: #d88986;
}
</style>
