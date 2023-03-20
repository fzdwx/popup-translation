<!-- 

  聚合翻译模式

 -->
<script setup lang="ts">
import { Ref, inject, reactive, watchEffect } from "vue";
import { listen, Event } from "@tauri-apps/api/event";

import Card from "../../common/Card.vue";

import deeplImage from "../../../assets/deepl.png";
import chatgptImage from "../../../assets/chatgpt.png";
import googleImage from "../../../assets/google.ico";

import { getSelectionText } from "../../../command/core";
import { freegpt } from "../../../platform/chatgpt";
import { deepl } from "../../../platform/deepl";
import { google } from "../../../platform/google";
import { AggregateTranslationInfo, TranslationItem } from "../../../types/type";

const state: AggregateTranslationInfo = reactive({
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
    reload();
  }
);

async function reload() {
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

const resetItem = () => {
  return {
    text: "",
    loading: true,
  } as TranslationItem;
};

const reloadSelectionText = inject<Ref<boolean>>("reloadSelectionText");
watchEffect(() => {
  if (reloadSelectionText?.value === true) {
    reload();
    reloadSelectionText.value = false;
  }
});
</script>

<template>
  <div class="aggregate">
    <!-- <Card
      class="card"
      :img-src="chatgptImage"
      title="Chatgpt"
      :text="state.chatgpt.text"
      :load="state.chatgpt.loading"
    /> -->
    <Card
      class="card"
      :img-src="googleImage"
      title="Google"
      :text="state.google.text"
      :load="state.google.loading"
    />
    <Card
      class="card"
      :img-src="deeplImage"
      title="Deepl"
      :text="state.deepl.text"
      :load="state.deepl.loading"
    />
  </div>
</template>

<style scoped>
.card {
  margin: 0 auto;
  margin-top: 20px;
}

.aggregate {
  margin: 20px 0;
}
</style>
