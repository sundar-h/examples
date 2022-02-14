// 组合式API
<script setup>
import { ref, onMounted } from "vue";

// 响应式数据
const count = ref(0);

function increment() {
  count.value++;
}

// 生命周期钩子;
onMounted(() => {
  console.log("The initial count is ${ count.value }.");
});
</script>

<template>
  <button @click="increment">Count is: {{ count }}</button>
</template>
