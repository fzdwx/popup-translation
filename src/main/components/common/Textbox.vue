<script setup lang="ts">
import { ref, watch, watchEffect } from 'vue';
import Loading from "./Loading.vue"
const isFocusOrBlur = ref(false);
const props = defineProps<{
  isTextarea: boolean,
  text?: string,
  load?: boolean,
  getTextInputVal?: (text: string) => void,
}>();

const textAreaBg = (val: boolean) => {
  isFocusOrBlur.value = val;
};

const translate_content = ref("");
watchEffect(()=>{
  translate_content.value = props.text as string;
});

watch(translate_content, (val) => {
  props.getTextInputVal?.(val);
});

const copyText = () => {
  const userAgent = navigator.userAgent;
  if (userAgent.includes("Mac")) {
    const textArea = document.createElement("textarea");
    textArea.value = translate_content.value;
    document.body.appendChild(textArea);
    textArea.select();
    // 复制选中的文本内容
    document.execCommand("copy");
    document.body.removeChild(textArea);
  }else {
    const input = document.createElement("input");
    input.setAttribute("type", "text");
    input.value = translate_content.value;
    document.body.appendChild(input);
    input.select();
    document.execCommand("copy");
    document.body.removeChild(input);
  }
};


</script>

<template>
  <div class="box"> 
    <Loading :load="props.load" v-if="isTextarea">
        <!-- 
          if load then show loading
          else show text
        -->
    </Loading>
    <textarea 
    name="source_text" 
    id="source_text" 
    cols="30" 
    rows="10"
    :style="{backgroundColor: isFocusOrBlur ? '#fff': '#1d1d1d', color: isFocusOrBlur ? 'black':'#fff'}"
    v-if="props.isTextarea"
    @focus="textAreaBg(true)"
    @blur="textAreaBg(false)"
    v-model="translate_content"
    >
    </textarea>
    <div class="text" v-else>
      {{ text }}
    </div>
    <div class="tool" @click="copyText">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.box {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  width: 400px;
  height: 300px;
  margin: 10px;
  border: 1px solid #fff;
  border-radius: 5px;
}
.box > textarea {
  width: 100%;
  height: 100%;
  padding: 5px;
  background-color: #1d1d1d;
  color: #ffffff;
  font-size: 18px;
  resize: none;
  border-radius: 5px;
}
.box .text {
  padding: 5px;
}

.box .tool {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  margin-right: 30px;
  font-size: 14px;
  cursor: pointer;
}
</style>