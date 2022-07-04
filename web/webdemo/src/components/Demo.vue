<template>
    <button @click="invoke">点击我</button>
</template>

<script setup lang="ts">
import { invoke, invoke2 } from '../api/invoke'

const invokeMe = async () => {
    // 处理网络错误
    const message = invoke2()
}
</script>
