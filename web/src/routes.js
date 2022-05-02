import Cookies from 'js-cookie'
import { createRouter, createWebHistory } from 'vue-router'
import Home from './views/Home.vue'
import About from './views/About.vue'
import Login from './views/Login.vue'
import NotFound from './views/NotFound.vue'

/** @type {import('vue-router').RouterOptions['routes']} */
const routes = [
  {
    path: '/', component: Home, meta: { title: 'Home' },
    children: [
      {
        path: '/about',
        meta: { title: 'About' },
        component: About,
      },
    ]
  },
  { path: '/login', component: Login },
  { path: '/:path(.*)', component: NotFound },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})
router.beforeEach(async (to, from) => {
  if (!Cookies.get('sid') && to.path !== '/login') {
    return '/login'
  }
})

export default router
