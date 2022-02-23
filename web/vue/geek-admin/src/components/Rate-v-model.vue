<template>
  <div :style="fontstyle">
    <div class="rate" @mouseout="mouseOut">
      <!-- ☆ ★ -->
      <span @mouseover="mouseOver(num)" v-for="num in 5" :key="num">☆</span>
      <span class="hollow" :style="fontwidth">
        <span
          @click="onRate(num)"
          @mouseover="mouseOver(num)"
          v-for="num in 5"
          :key="num"
          >★</span
        >
      </span>
    </div>
    {{ rate }}
  </div>
</template>

<script setup>
import { defineProps, computed, ref } from "vue";

// defineProps 定义传输数据的格式
let props = defineProps({
  //   value: Number, // --> <!-- <Rate :value="score" @update-rate="update"></Rate> -->
  modelValue: Number, // -->  <Rate v-model="score" theme="green"></Rate>
  theme: { type: String, default: "orange" },
});

let width = ref(props.value);

// defineEmits 定义监听函数-> 向父组件发射数据
// let emits = defineEmits("update-rate");
let emits = defineEmits("update:modelValue");

// function onRate(num) {
//   emits("update-rate", num);
// }

function mouseOver(i) {
  width.value = i;
}

function mouseOut() {
  width.value = props.value;
}

const fontwidth = computed(() => `width:${width.value}em;`);

// let rate = computed(() =>
//   "☆☆☆☆☆★★★★★".slice(5 - props.value, 10 - props.value)
// );

const themObj = {
  black: "#000",
  white: "#fff",
  red: "#f522d",
  orange: "#fa541c",
  green: "#73d13d",
  blue: "#40a9ff",
};

const fontstyle = computed(() => {
  return `color:${themObj[props.theme]};`;
});
</script>

<style scoped>
.rate {
  position: relative;
  display: inline-block;
}

.rate > span.hollow {
  position: absolute;
  display: inline-block;
  top: 0;
  left: 0;
  width: 0;
  overflow: hidden;
}
</style>
