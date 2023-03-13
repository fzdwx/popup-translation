<script setup lang="ts">
import { inject, reactive } from "vue";
import { google } from "../platform/google";

import { IconTransformFilled, IconCalendar, IconCopy } from "@tabler/icons-vue";
import Textbox from "./common/Textbox.vue";
import Button from "./common/Button.vue";
import AggTranslation from "./aggregateMode/Translation.vue";

import { Platform, TranslationInfo } from "../types/type";

// const takes = inject<{isTakes: boolean}>("isTakes");
const platform = inject<{ current: Platform }>("plat");
const model = inject<{ currentModel: number }>("model");

const state: TranslationInfo = reactive({
  source: {
    text: "",
    loading: false,
  },

  google: {
    text: "",
    loading: false,
  },
});

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
  <AggTranslation :show="model?.currentModel === 1" />
  <div class="content" v-if="model?.currentModel !== 1">
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
