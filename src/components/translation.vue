<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core"
import { deepl } from "../platform/deepl"
import loding from "./loding.vue"

interface TranslationInfo {
  source: string
  sourceLoading: boolean

  deeplTarget: string
  deeplTargetLoading: boolean
}

const translationInfo: TranslationInfo = reactive({
  source: "",
  sourceLoading: false,

  deeplTarget: "",
  deeplTargetLoading: false,
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

  translationInfo.deeplTarget = ""
  translationInfo.deeplTargetLoading = true

  await getSelectionText()
    .then(val => {
      translationInfo.source = val.toString()
      translationInfo.sourceLoading = false
      deepl(translationInfo.source, "auto", "zh").then(text => {
        translationInfo.deeplTarget = text
        translationInfo.deeplTargetLoading = false
      })

    })
    .catch(err => {
      console.log(err);
    })
}

</script>

<template>
  <div>
    <button type="button" @click="greet()">刷新</button>
    <div class="mtop20"><!-- deepl -->
      <div class="title-container">
        <img class="deepl-ico" src="../assets/deepl.png">
        <h6>Deepl</h6>
      </div>
      <div class="text-container target-height">
        <loding :load="translationInfo.deeplTargetLoading">
          <div class="text">
            {{ translationInfo.deeplTarget }}
          </div>
        </loding>
      </div>
    </div>
  </div>
</template>

<style scoped>
.title-container {
  display: flex;
  align-items: center;
  height: 25px;
  margin-bottom: 5px;
}

.deepl-ico {
  display: inline-block;
  width: 25px;
  height: 25px;
  margin-right: 5px;
}

.text-container {
  width: 95%;
  box-shadow: 0 0 0 1px rgba(36, 104, 193, 0.8);
  border-radius: 8px;
  border: 1px solid transparent;
  overflow: auto;
}

.mtop20 {
  margin: 0 auto;
  margin-top: 20px;
}

.text {
  padding: 10px;
}

.source-height {
  max-height: 160px;
  min-height: 100px;
}

.target-height {
  max-height: 60%;
  min-height: 100px;
}
</style>