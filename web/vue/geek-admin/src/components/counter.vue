<template>
    <h1 @click="add">{{count}}</h1>
</template>

<script setup>
    import { ref } from 'vue';
    let count = ref(1)
    let color = ref('red')

    function add() {
        count.value++
        color.value = Math.random()>0.5 ? "blue" : "red"
    }
</script>

// scope 使样式仅应用在当前组件上，可以避免样式冲突
// 在style 里使用 JavaScript变量
<style scoped>
h1 {
    /* color: red; */
    color: v-bind(color);
}
</style>