<script setup lang="ts">
import { inject, reactive } from 'vue';

import { IconTransformFilled, IconCopy, IconTexture } from '@tabler/icons-vue';
import Textbox from '../../common/Textbox.vue';
import Button from '../../common/Button.vue';
import LangSwitch from '../../common/LangSwitch.vue';

import { Platform, TranslationInfo } from '../../../../types/type';
import { google } from '../../../../platform/google';
import { freegpt } from '../../../../platform/chatgpt';
import { deepl } from '../../../../platform/deepl';
import { youdao } from '../../../../platform/youdao';

const platform = inject<{ current: Platform }>('plat');

const state: TranslationInfo = reactive({
  source: {
    text: '',
    loading: false,
    result: '',
  },
  targetLang: 'chinese',
});

const getTextInputVal = (text: string) => {
  state.source.text = text;
};
const translateStart = () => {
  if (state.source.text === '') {
    return;
  }
  state.source.loading = true;

  switch (platform?.current) {
    case Platform.Google:
      google(state.source.text, 'auto', state.targetLang).then((text) => {
        state.source.result = text;
        state.source.loading = false;
      });
      break;
    case Platform.ChatGTP:
      freegpt(state.source.text, state.targetLang).then((text) => {
        state.source.result = text;
        state.source.loading = false;
      });
      break;
    case Platform.Deepl:
      deepl(state.source.text, 'auto', state.targetLang)
        .then((text) => {
          state.source.result = text;
          state.source.loading = false;
        })
        .catch((err) => {
          console.log('deepl error', err);
        });
      break;
    case Platform.YouDao:
      youdao(state.source.text, 'auto')
        .then((text) => {
          state.source.result = text;
          state.source.loading = false;
        })
        .catch((err) => {
          console.log('youdao api error', err);
        });
      break;
    case Platform.Bing:
      break;
  }
};

const cleanClick = () => {
  state.source.text = '';
};

const onChangeLang = (lang: string) => {
  state.targetLang = lang;
};
</script>
<template>
  <div class="content">
    <Textbox :isTextarea="true" :text="state.source.text" :getTextInputVal="getTextInputVal" :load="state.source.loading"> </Textbox>
    <div class="btns">
      <LangSwitch :onChange="onChangeLang" :lang="state.targetLang" />
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
