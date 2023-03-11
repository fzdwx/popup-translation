<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { message } from '@tauri-apps/api/dialog';

const frameRef = ref(null)

const props = defineProps<{
  url: string // thirt part website url
  jscode: string // after doc load will run it
}>()



const refrsh = computed(() => {
  return () => {
    message(props.url)
  }
})

</script>

<template>
  <iframe :on-load="refrsh" width="100%" height="100%" ref="frameRef" :src="props.url" />
</template>