<script setup lang="ts">
import { inject, reactive } from "vue";
import { google } from "../platform/google";

import { IconTransformFilled, IconCopy ,IconTexture} from "@tabler/icons-vue";
import Textbox from "./common/Textbox.vue";
import Button from "./common/Button.vue";
import AggTranslation from "./mode/aggregate/Translation.vue";

import { Mode, Platform, TranslationInfo } from "../types/type";
import { deepl } from "../platform/deepl";
import { freegpt } from "../platform/chatgpt";

// const takes = inject<{isTakes: boolean}>("isTakes");
const platform = inject<{ current: Platform }>("plat");
const mode = inject<{ currentMode: Mode }>("mode");

const state: TranslationInfo = reactive({
  source: {
    text: "",
    loading: false,
    result: "",
  },
});

// watchEffect(() => {
// if (takes?.isTakes) {
// TODO
// 划屏取词
// }
// });
const getTextInputVal = (text: string) => {
    state.source.text = text;
};
const translateStart = () => {
  if (state.source.text === "") {
    return;
  }
  state.source.loading = true;

  switch (platform?.current) {
    case Platform.Google:
      google(state.source.text, "auto", "chinese").then((text) => {
        state.source.result = text;
        state.source.loading = false;
      });
      break;
    case Platform.ChatGTP:
      freegpt(state.source.text, "chinese").then((text) => {
        state.source.result = text;
        state.source.loading = false;
      });
      break;
    case Platform.Deepl:
      deepl(state.source.text, "auto", "chinese")
        .then((text) => {
          state.source.result = text;
          state.source.loading = false;
        })
        .catch((err) => {
          console.log("deepl error", err);
        });
      break;
    case Platform.Bing:
      break;
    case Platform.YouDao:
      break;
  }
};

const cleanClick = ()=> {
  state.source.text = "";
};
</script>

<template>
  <AggTranslation v-if="mode?.currentMode === Mode.Aggregate" />
  <div class="content" v-else>
    <Textbox
      :isTextarea="true"
      :text="state.source.text"
      :getTextInputVal="getTextInputVal"
      :load="state.source.loading"
    >
    </Textbox>
    <div class="btns">
      <Button class="tran_btn" @click="cleanClick">
        <IconTexture />
        清空
      </Button>
      <Button class="tran_btn" @click="translateStart">
        <IconTransformFilled />
        翻译
      </Button>
    </div>
    <Textbox :isTextarea="false" :text="state.source?.result">
      <IconCopy />
      复制文本
    </Textbox>
  </div>
</template>

<style scoped>
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
