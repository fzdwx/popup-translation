<script setup lang="ts">
import { reactive, provide, watchEffect, ref, onBeforeMount } from "vue";

import Translation from "./components/Translation.vue";
import Nav from "./components/Nav.vue";
import Set from "./components/Set.vue";
import Model1Nav from "./components/nav/Model1Nav.vue";

import { KeyInfo, Model, Platform } from "./types/type";
import { readConfig } from "./command/core";

const plat = reactive({
  current: Platform.Google,
});

const takes = reactive({
  isTakes: true,
});

const keyList = reactive<KeyInfo>({
  chatGpt: {
    platform: "ChatGPT Key",
    key: "",
  },
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
  currentModel: Model.ModelTwo,
});
const showSetPage = reactive({
  show: false,
});

// watchEffect(()=>{
//   console.log(takes.isTakes);
// });

// read config
onBeforeMount(() => {
  readConfig().then((config) => {
    const keys = config.keys;
    keyList.chatGpt.key = keys.chatGpt;
    keyList.google.key = keys.google;
    keyList.youdao.key = keys.youdao;
  });
});

let readText = ref(false);
const reload = () => {
  readText.value = true;
};
provide("plat", plat);
provide("model", model);
provide("readText", readText);
provide("showSetPage", showSetPage);
</script>

<template>
  <div class="header">
    <Nav :plat="plat" :takes="takes" :showSetPage="showSetPage">
      <template #platform_link>
        <button
          type="button"
          v-if="model.currentModel === Model.ModelOne"
          @click="reload"
        >
          读取选中文本/粘贴板
        </button>
        <Model1Nav v-else :plat="plat"></Model1Nav>
      </template>
    </Nav>
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
<style scoped>
.setting {
  position: absolute;
  right: 10px;
  top: 50px;
}
</style>
