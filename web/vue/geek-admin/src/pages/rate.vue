<template>
  <h1>您的评分是 {{ score }}</h1>
  <!-- 使用update-rate 接收子组件emits的数据 -->
  <Rate :value="score" @update-rate="update"></Rate>
</template>

<script setup>
import { ref } from "vue";
import Rate from "../components/Rate.vue";
let score = ref(3.5);

function update(num) {
  score.value = num;
}
</script>
