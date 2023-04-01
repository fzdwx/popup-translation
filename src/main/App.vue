<script setup lang="ts">
import { reactive, provide, watchEffect, ref, onBeforeMount } from 'vue';

import Translation from './components/Translation.vue';
import Nav from './components/Nav.vue';
import Set from './components/Set.vue';
import SplitModeNav from './components/mode/split/Nav.vue';
import AggregateModeNav from './components/mode/aggregate/Nav.vue';
import Button from './components/common/Button.vue';
import LangSwitch from './components/common/LangSwitch.vue';

import { readConfig } from '../command/core';
import { Platform, KeyInfo, Mode, Shortcuts } from '../types/type';

const plat = reactive({
  current: Platform.Google,
});

const takes = reactive({
  isTakes: true,
});

const keyList = reactive<KeyInfo>({
  chatGpt: {
    platform: 'ChatGPT Key',
    key: '',
  },
  google: {
    platform: 'Google Key',
    key: '',
  },
  youdao: {
    platform: 'youdao Key',
    key: '',
  },
});
const mode = reactive({
  currentMode: Mode.Split,
});
const showSetPage = reactive({
  show: false,
});
const shortcuts = reactive<Shortcuts>({
  toggle: 'Alt+Shift+T',
});

const currentLang = reactive({
  lang: 'chinese',
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
    if (config.mode !== undefined) {
      mode.currentMode = config.mode;
    }
    if (config.shortcuts !== undefined) {
      shortcuts.toggle = config.shortcuts.toggle;
    }
  });
});

// for aggregate mode
// refresh selection text
let reloadSelectionText = ref(false);
const reload = () => {
  reloadSelectionText.value = true;
};

const switchLang = (lang: string) => {
  currentLang.lang = lang;
};

provide('plat', plat);
provide('mode', mode);
provide('reloadSelectionText', reloadSelectionText);
provide('showSetPage', showSetPage);
provide('currentLang', currentLang);
</script>

<template>
  <div class="header">
    <Nav :plat="plat" :takes="takes" :showSetPage="showSetPage">
      <template #platform_link>
        <AggregateModeNav :reload="reload" :mode="mode" v-if="mode.currentMode === Mode.Aggregate">
          <template #agg-btn>
            <LangSwitch :onChange="switchLang" />
          </template>
        </AggregateModeNav>
        <SplitModeNav v-else :plat="plat"></SplitModeNav>
      </template>
    </Nav>
  </div>
  <div class="container">
    <Translation />
  </div>
  <div class="setting" v-if="showSetPage.show">
    <Set :shortcuts="shortcuts" :keyList="keyList" :mode="mode" />
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
