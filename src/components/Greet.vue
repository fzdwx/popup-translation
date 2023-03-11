<script setup lang="ts">
import { ref } from "vue";
import { getSelectionText } from "../command/core"

const greetMsg = ref("");
const name = ref("");

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
