import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/smart_address",
      name: "smart_address",
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import("../views/AddressView.vue"),
    },
    {
      path: "/solana",
      name: "solana",
      component: () => import("../views/SolanaView.vue"),
    },
  ],
  linkActiveClass: "bg-blue-500",
});

export default router;
