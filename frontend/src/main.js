import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import Index from "./components/Index.vue";
import { createRouter, createWebHashHistory } from "vue-router";

const pinia = createPinia();
const routes = [
  { path: "/", component: Index },
  {
    path: "/smart_address",
    component: Index,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  // 匹配当前路由时的样式
  linkActiveClass: "bg-blue-500",
});

const app = createApp(App);

app.use(pinia);
app.use(router);
app.mount("#app");
