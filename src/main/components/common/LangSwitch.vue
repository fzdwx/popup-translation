<script setup lang="ts">
import { inject, reactive, ref } from 'vue';
import { Platform } from '../../../types/type';

const state = reactive<{
  lang: string;
}>({ lang: 'chinese' });

const props = defineProps<{
  onChange: (lang: string) => void;
}>();

const changeLang = () => {
  props.onChange(state.lang);
};
</script>
<template>
  <select v-model="state.lang" @change="changeLang">
    <option value="chinese">中文</option>
    <option value="english">English</option>
  </select>
</template>
