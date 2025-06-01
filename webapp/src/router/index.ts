import {createRouter, createWebHistory} from 'vue-router'
import HomeView from "../views/HomeView.vue";
import ListerView from "../views/ListerView.vue";

const router = createRouter({
    history: createWebHistory('/web/'),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView,
        },
        {
            path: '/list',
            name: 'list',
            component: ListerView
        }
    ]
})

export default router