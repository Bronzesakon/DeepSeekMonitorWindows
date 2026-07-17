import { createApp } from "vue";
import { createPinia } from "pinia";
import { createRouter, createWebHashHistory } from "vue-router";
import App from "./App.vue";
import "./style.css";

import DashboardView from "./views/DashboardView.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", redirect: "/dashboard" },
    { path: "/dashboard", component: DashboardView },
    { path: "/settings", component: () => import("./views/SettingsView.vue") },
    { path: "/widget", component: () => import("./views/WidgetView.vue") },
    { path: "/trend-detail", component: () => import("./views/TrendDetailView.vue") },
  ],
});

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.mount("#app");
