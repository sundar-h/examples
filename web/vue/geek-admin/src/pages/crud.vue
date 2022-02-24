<template>
  <div>
    <input type="text" v-model="prefix" placeholder="Filter prefix" />
  </div>

  <select size="5" v-model="selected">
    <option v-for="name in filterNames">{{ name }}</option>
  </select>
  <label> Name: <input v-model="first" /> </label>
  <label> Surname: <input v-model="last" /> </label>

  <div class="buttons">
    <button @click="">Create</button>
    <button @click="">Update</button>
    <button @click="">Delete</button>
  </div>
</template>

<script setup>
import { computed, ref } from "vue";
let prefix = ref("");
let selected = ref("");
let first = ref("");
let last = ref("");

let names = ref([
  { first: "Hans", last: "Emil" },
  { first: "Max", last: "Mustermann" },
]);

const filterNames = computed((name) => names.value);
</script>

<style>
select {
  float: left;
}
.buttons {
  clear: both;
}
button + button {
}
</style>
