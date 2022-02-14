<template>
  <div id="todo-list-app">
    <ol>
      <todo-item
        v-for="item in groceryList"
        v-bind:todo="item"
        v-bind:key="item.id"
      ></todo-item>
    </ol>
  </div>
</template>

<script>
const ToDoItem = {
  props: ["todo"],
  template: `<li>{{ todo.text }}</li>`,
};
const ToDoList = {
  data() {
    return {
      groceryList: [
        { id: 0, text: "Vegatables" },
        { id: 1, text: "Cheese" },
        { id: 2, text: "Whatever else" },
      ],
    };
  },
  components: {
    ToDoItem,
  },
};
Vue.createApp(ToDoList).mount("#todo-list-app");
</script>

<style></style>
