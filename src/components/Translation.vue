<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { inject, provide, reactive, ref, watchEffect } from "vue";
import { getSelectionText } from "../command/core";
import { deepl } from "../platform/deepl";
import { freegpt } from "../platform/chatgpt";
import { google } from "../platform/google";
import Card from "./common/Card.vue";

import deeplImage from "../assets/deepl.png";
import chatgptImage from "../assets/chatgpt.png";
import googleImage from "../assets/google.ico";

import { IconTransformFilled, IconCalendar, IconCopy } from "@tabler/icons-vue";
import Textbox from "./common/Textbox.vue";
import Button from "./common/Button.vue";

import { Platform } from "../types/type";

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

// const takes = inject<{isTakes: boolean}>("isTakes");
const platform = inject<{ current: Platform }>("plat");
const model = inject<{ currentModel: number }>("model");
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

// watchEffect(() => {
// if (takes?.isTakes) {
// TODO
// 划屏取词
// }
// });
const getTextInputVal = (text: string) => {
  if (text === "") {
    console.log(2);
    return;
  } else {
    state.source.text = text;
  }
};
const translateStart = () => {
  state.source.loading = true;

  switch (platform?.current) {
    case Platform.Bing:
      break;
    case Platform.Google:
      console.log(platform.current);
      google(state.source.text, "auto", "chinese").then((text) => {
        state.google.text = text;
        state.source.loading = false;
      });
      break;
    case Platform.YouDao:
      break;
  }
};
</script>

<template>
  <div v-if="model?.currentModel === 1">
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
  <div class="content" v-else>
    <Textbox
      :isTextarea="true"
      :text="state.source.text"
      :getTextInputVal="getTextInputVal"
      :load="state.source.loading"
    ></Textbox>
    <div class="btns">
      <Button class="tran_btn">
        <IconCalendar />
        清空
      </Button>
      <Button class="tran_btn" @click="translateStart">
        <IconTransformFilled />
        翻译
      </Button>
    </div>
    <Textbox :isTextarea="false">
      <IconCopy />
      复制文本
    </Textbox>
  </div>
</template>

<style scoped>
.mtop20 {
  margin: 0 auto;
  margin-top: 20px;
}
.content {
  display: flex;
  justify-content: center;
  align-items: center;
}
.btns {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.tran_btn:hover {
  border: 1px solid #fff;
}
</style>
