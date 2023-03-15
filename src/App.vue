<script setup lang="ts">
import { reactive, provide, watchEffect, ref, onBeforeMount } from "vue";

import Translation from "./components/Translation.vue";
import Nav from "./components/Nav.vue";
import Set from "./components/Set.vue";
import SplitModeNav from "./components/mode/split/Nav.vue";

import { KeyInfo, Mode, Platform } from "./types/type";
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
const mode = reactive({
  currentMode: Mode.Split,
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
provide("mode", mode);
provide("readText", readText);
provide("showSetPage", showSetPage);
</script>

<template>
  <div class="header">
    <Nav :plat="plat" :takes="takes" :showSetPage="showSetPage">
      <template #platform_link>
        <button
          type="button"
          v-if="mode.currentMode === Mode.Aggergate"
          @click="reload"
        >
          读取选中文本/粘贴板
        </button>
        <SplitModeNav v-else :plat="plat"></SplitModeNav>
      </template>
    </Nav>
  </div>
  <div class="container">
    <Translation />
  </div>
  <div class="setting" v-if="showSetPage.show">
    <Set :keyList="keyList" :mode="mode" />
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
