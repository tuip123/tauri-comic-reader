
import { createRouter, createWebHistory } from "vue-router";
import home from "@/view/Home.vue";
import comicBookcase from "@/view/ComicBookcase.vue";
import comicReader from "@/view/ComicReader.vue";
import config from "@/view/Config.vue";
const routes:any = [
    {
        path: '/',
        component: home
    },
    {
        path: '/ComicBookcase',
        component: comicBookcase
    },
    {
        path: '/ComicReader',
        component: comicReader
    },
    {
        path: '/Config',
        component: config
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router
