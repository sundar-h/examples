<template>
    <input type="text" v-model="title" @keydown.enter="addTodo" />
    <button v-if="active<all" @click="clear">清理</button>
    <ul v-if="todos.length">
        <li v-for="todo in todos">
            <input type="checkbox" v-model="todo.done" />
            <span :class="{ done: todo.done }"> {{ todo.title }} </span>
        </li>
    </ul>
    <div v-else>暂无数据</div>
    <div>
        全选<input type="checkbox" v-model="allDone" />
        <span>{{active}} / {{all}}</span>
    </div>
</template>

<script setup>
    import { ref, computed, watchEffect } from 'vue';
    import useStorage from '../utils/useStorage'
    let title = ref("");
    let todos = useStorage('todos', []);

    // watchEffect 监听reactive的数据变化
    watchEffect(() => {
        localStorage.setItem('todos', JSON.stringify(todos.value))
    })

    function addTodo() {
        todos.value.push({
            title: title.value,
            done: false
        });
        title = "";
    }

    function clear() {
        todos.value = todos.value.filter(todo => !todo.done )
    }

    let active = computed(() => {
        return todos.value.filter(todo => !todo.done).length
    })

    let all = computed(() => {
        return todos.value.length
    })

    let allDone = computed({
        get: function() {
            return active.value === 0
        },
        set: function(value) {
            todos.value.forEach((todo) => {
                todo.done = value
            })
        }
    })
</script>

<style>
.done {
    color: gray;
    text-decoration: line-through;
}
</style>