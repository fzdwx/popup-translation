<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core";
import { deepl } from "../platform/deepl";
import Card from "./card.vue";

import deeplImage from "../assets/deepl.png";

interface TranslationInfo {
  source: string;
  sourceLoading: boolean;

  deeplTarget: string;
  deeplTargetLoading: boolean;
}

const translationInfo: TranslationInfo = reactive({
  source: "",
  sourceLoading: false,

  deeplTarget: "",
  deeplTargetLoading: false,
});

/**
 * 刷新翻译
 */
const unListenRefreshTranslation = listen(
  "refresh-translation",
  async (event: Event<string>) => {
    greet();
  }
);

async function greet() {
  translationInfo.source = "";
  translationInfo.sourceLoading = true;

  translationInfo.deeplTarget = "";
  translationInfo.deeplTargetLoading = true;

  await getSelectionText()
    .then((val) => {
      translationInfo.source = val.toString();
      translationInfo.sourceLoading = false;
      deepl(translationInfo.source, "auto", "zh").then((text) => {
        translationInfo.deeplTarget = text;
        translationInfo.deeplTargetLoading = false;
      });
    })
    .catch((err) => {
      console.log(err);
    });
}
</script>

<template>
  <div>
    <button type="button" @click="greet()">刷新</button>
    <Card
      :img-src="deeplImage"
      title="Deepl"
      :text="translationInfo.deeplTarget"
      :load="translationInfo.deeplTargetLoading"
    />
  </div>
</template>
