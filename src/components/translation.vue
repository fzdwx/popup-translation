<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core";
import { deepl } from "../platform/deepl";
import { freegpt } from "../platform/chatgpt";
import Card from "./card.vue";

import deeplImage from "../assets/deepl.png";
import chatgptImage from "../assets/chatgpt.png";

interface TranslationItem {
  text: string;
  loading: boolean;
}

interface TranslationInfo {
  source: TranslationItem;
  deepl: TranslationItem;
  chatgpt: TranslationItem;
}

const state: TranslationInfo = reactive({
  source: {
    text: "",
    loading: false,
  },

  deepl: {
    text: "",
    loading: false,
  },

  chatgpt: {
    text: "",
    loading: false,
  },
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
  state.source.text = "";
  state.source.loading = true;

  state.deepl.text = "";
  state.deepl.loading = true;

  state.chatgpt.text = "";
  state.chatgpt.loading = true;

  await getSelectionText()
    .then((val) => {
      state.source.text = val.toString();
      state.source.loading = false;
      deepl(state.source.text, "auto", "chinese").then((text) => {
        state.deepl.text = text;
        state.deepl.loading = false;
      });

      freegpt(state.source.text, "chinese").then((text) => {
        state.chatgpt.text = text;
        state.chatgpt.loading = false;
      });
    })
    .catch((err) => {
      console.log(err);
    });
}
</script>

<template>
  <div>
    <button type="button" @click="greet()">读取选中文本/粘贴板</button>
    <Card
      class="mtop20"
      :img-src="chatgptImage"
      title="Chatgpt"
      :text="state.chatgpt.text"
      :load="state.chatgpt.loading"
    />
    <Card
      class="mtop20"
      :img-src="deeplImage"
      title="Deepl"
      :text="state.deepl.text"
      :load="state.deepl.loading"
    />
  </div>
</template>

<style scoped>
.mtop20 {
  margin: 0 auto;
  margin-top: 20px;
}
</style>
