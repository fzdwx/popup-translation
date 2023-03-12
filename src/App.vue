<script setup lang="ts">
import { reactive, provide,  watchEffect } from 'vue'
import Translation from "./components/translation.vue";
import Nav from "./components/Nav.vue";
import { Platform } from "./types/type";
const plat = reactive({
  current: Platform.YouDao,
});

const takes = reactive({
  isTakes: false,
});
// watchEffect(()=>{
//   console.log(takes.isTakes);
// });
provide("plat", plat);
</script>

<template>
  <div class="header">
    <Nav :plat = plat :takes= takes />
  </div>
  <div class="container">
    <Translation />
  </div>
</template>

<style scoped></style>
