// 项目入口
import { createApp } from 'vue'
import App from './App.vue'

import router from './router/index';

// 注册路由
createApp(App)
    .use(router)
    .mount('#app')
