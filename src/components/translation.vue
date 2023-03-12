<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { getSelectionText } from "../command/core";
import { deepl } from "../platform/deepl";
import { freegpt } from "../platform/chatgpt";
import { google } from "../platform/google";
import Card from "./card.vue";

import deeplImage from "../assets/deepl.png";
import chatgptImage from "../assets/chatgpt.png";
import googleImage from "../assets/google.ico";

interface TranslationItem {
  text: string;
  loading: boolean;
}

const resetItem = () => {
  return {
    text: "",
    loading: true,
  } as TranslationItem;
};

interface TranslationInfo {
  source: TranslationItem;
  deepl: TranslationItem;
  chatgpt: TranslationItem;
  google: TranslationItem;
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

  google: {
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

async function refresh(source: TranslationItem) {
  deepl(source.text, "auto", "chinese")
    .then((text) => {
      state.deepl.text = text;
      state.deepl.loading = false;
    })
    .catch((err) => {
      console.log("deepl error", err);
    });

  freegpt(source.text, "chinese").then((text) => {
    state.chatgpt.text = text;
    state.chatgpt.loading = false;
  });

  google(source.text, "auto", "chinese").then((text) => {
    state.google.text = text;
    state.google.loading = false;
  });
}

async function greet() {
  state.source = resetItem();
  state.chatgpt = resetItem();
  state.deepl = resetItem();
  state.google = resetItem();

  await getSelectionText()
    .then((val) => {
      state.source.text = val.toString();
      state.source.loading = false;
      refresh(state.source);
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
      :img-src="googleImage"
      title="Google"
      :text="state.google.text"
      :load="state.google.loading"
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
