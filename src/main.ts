import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from  "./router/index"
import VueLazyload from 'vue-lazyload'

createApp(App)
    .use(router)
    .use(VueLazyload,{})
    .mount("#app");
