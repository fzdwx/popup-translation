<script setup lang="ts">
import { reactive, provide, watchEffect } from "vue";

import Translation from "./components/Translation.vue";
import Nav from "./components/Nav.vue";
import Set from "./components/Set.vue";

import { KeyInfo, Platform } from "./types/type";
const plat = reactive({
  current: Platform.YouDao,
});

const takes = reactive({
  isTakes: false,
});

const keyList = reactive<KeyInfo>({
  google: {
    platform: "Google Key",
    key: "",
  },
  youdao: {
    platform: "youdao Key",
    key: "",
  },
});
const model = reactive({
  currentModel: 1,
});
const showSetPage = reactive({
  show: true,
});
// watchEffect(()=>{
//   console.log(takes.isTakes);
// });
provide("plat", plat);
provide("model", model);
</script>

<template>
  <div class="header">
    <Nav :plat="plat" :takes="takes" :showSetPage="showSetPage" />
  </div>
  <div class="container">
    <Translation />
  </div>
  <div class="setting" v-if="showSetPage.show">
    <Set :keyList="keyList" :model="model" />
  </div>
</template>

<style scoped>
.setting {
  position: absolute;
  right: 0;
  top: 50px;
}
</style>
