<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core"
import { deepl } from "../platform/deepl"
import loding from "./loding.vue"

interface TranslationInfo {
  source: string
  sourceLoading: boolean

  target: string
  targetLoading: boolean
}

const translationInfo: TranslationInfo = reactive({
  source: "",
  sourceLoading: false,

  target: "",
  targetLoading: false,
})

/**
 * 刷新翻译
 */
const unListenRefreshTranslation = listen('refresh-translation', async (event: Event<string>) => {
  greet()
})

async function greet() {
  translationInfo.source = ""
  translationInfo.sourceLoading = true

  translationInfo.target = ""
  translationInfo.targetLoading = true

  await getSelectionText()
    .then(val => {
      translationInfo.source = val.toString()
      translationInfo.sourceLoading = false
      deepl(translationInfo.source, "auto", "zh").then(text => {
        translationInfo.target = text
        translationInfo.targetLoading = false
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
      <loding :load="translationInfo.sourceLoading">
        <div>
          {{ translationInfo.source }}
        </div>
      </loding>
    </div>
    <div>
      <loding :load="translationInfo.targetLoading">
        <div>
          {{ translationInfo.target }}
        </div>
      </loding>
    </div>
  </div>
</template>