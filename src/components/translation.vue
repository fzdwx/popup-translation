<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core"
import Bing from "./bing.vue";

interface TranslationInfo {
  text: string
}

const translationInfo: TranslationInfo = reactive({
  text: ""
})

/**
 * 刷新翻译
 */
const unListenRefreshTranslation = listen('refresh-translation', async (event: Event<string>) => {
  greet()
})

async function greet() {
  await getSelectionText()
    .then(val => {
      translationInfo.text = val.toString()
    })
    .catch(err => {
      console.log(err);
    })
}

</script>

<template>
  <div class="card">
    <button type="button" @click="greet()">测试获取光标选择文本功能</button>
    <Bing :text="translationInfo.text" />
  </div>
</template>