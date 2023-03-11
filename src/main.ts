import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { listen } from "@tauri-apps/api/event";

createApp(App).mount("#app");
