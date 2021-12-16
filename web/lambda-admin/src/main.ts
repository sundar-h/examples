import { createApp } from "vue";
import App from './App.vue'
import router from './router';
// import { store, key } from './store';
import ElementPlus from 'element-plus'
// import VueGridLayout from 'vue-grid-layout';
const app = createApp(App);


app
.use(router)
// .use(store, key)
.use(ElementPlus)
// .use(VueGridLayout)
.use(ElementPlus)
