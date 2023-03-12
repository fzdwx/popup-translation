<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core"
import { deepl } from "../platform/deepl"
import loding from "./loding.vue"

interface TranslationInfo {
  sorce: string
  target: string
  loaded: boolean
}

const translationInfo: TranslationInfo = reactive({
  sorce: "",
  target: "",
  loaded: false,
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
      translationInfo.sorce = val.toString()

      translationInfo.loaded = true
      deepl(translationInfo.sorce, "auto", "zh").then(text => {
        translationInfo.target = text
        translationInfo.loaded = false
      })

    })
    .catch(err => {
      console.log(err);
    })
}

</script>

<template>
  <div>
    <button type="button" @click="greet()">测试获取光标选择文本功能</button>
    <div>
      {{ translationInfo.sorce }}
    </div>
    <div>
      <loding :loaded="translationInfo.loaded">
        <div>
          {{ translationInfo.target }}
        </div>
      </loding>
    </div>
  </div>
</template>