<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core"
import { deepl } from "../platform/deepl"

interface TranslationInfo {
  sorce: string
  target: string
}

const translationInfo: TranslationInfo = reactive({
  sorce: "",
  target: ""
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

      deepl(translationInfo.sorce, "auto", "zh").then(text => {
        translationInfo.target = text
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
      {{ translationInfo.target }}
    </div>
  </div>
</template>