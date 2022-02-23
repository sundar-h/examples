// 手动实现迷你路由
import { inject } from 'vue';
import { createRouter } from 'vue-router';
import router from '../../../00-构建工具/router/index';
import { createWebHashHistory } from 'vue-router';

const ROUTER_KEY = "__router__";

function createRouter(options) {
    return New Router(options)
}

function useRouter() {
    return inject(ROUTER_KEY)
}

function createWebHashHistory(){
    function bindEvents(fn) {
        window.addEventListener('hashchange', fn)
    }

    return {
        bindEvents,
        url:window.location.hash.slice(1) || '/'
    }
}

class Router {
    constructor(options) {
        this.history = options.history
        this.routes = options.routes
        this.current = ref(this.history.url)

        this.history.bindEvents(()=>{
            this.current.value = window.location.hash.slice(1)
        })
    }

    install(app){
        app.provide(ROUTER_KEY, this)
    }
}

export {createRouter, createWebHashHistory, useRouter}
