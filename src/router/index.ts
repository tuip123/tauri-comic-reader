import { createRouter, createWebHistory } from "vue-router";
const routes:any = [
    {
        path: '/',
        redirect:'/home'
    },
    {
        path: '/home',
        component: ()=>import('@/view/Home.vue')
    },
    {
        path: '/ComicBookcase',
        component: ()=>import('@/view/ComicBookcase.vue')
    },
    {
        path: '/ComicReader',
        component: ()=>import('@/view/ComicReader.vue')
    },
    {
        path: '/Config',
        component: ()=>import('@/view/Config.vue')
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router
