import { defineStore } from "pinia";

export const useNavigationStore = defineStore("navigation", {
  state: () => ({
    activePage: "home",
    menuItems: [
      { title: "Home", route: "home" },
      { title: "About", route: "about" },
      // Add more menu items as needed
    ],
  }),
  actions: {
    setActivePage(page) {
      this.activePage = page;
    },
  },
});
