import {createRouter, createWebHistory} from 'vue-router'
import HelloWorld from "../views/HomeView.vue";
import ListerView from "../views/ListerView.vue";

const router = createRouter({
    history: createWebHistory('/web/'),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HelloWorld,
        },
        {
            path: '/list',
            name: 'list',
            component: ListerView
        }
    ]
})

export default router