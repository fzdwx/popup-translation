<script setup lang="ts">
import { listen, Event } from "@tauri-apps/api/event";
import { ref } from "vue";
import { getSelectionText } from "../command/core"

const greetMsg = ref("");

/**
 * 刷新翻译
 */
const unListenRefreshTranslation = listen('refresh-translation', async (event: Event<string>) => {
  greet()
})

async function greet() {
  await getSelectionText()
    .then(val => {
      greetMsg.value = val.toString()
    })
    .catch(err => {
      console.log(err);
    })
}

</script>

<template>
  <div class="card">
    <button type="button" @click="greet()">测试获取光标选择文本功能</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
监听