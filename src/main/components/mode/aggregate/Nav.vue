<script setup lang="ts">
import { Mode } from '../../../../types/type';
import Button from '../../../components/common/Button.vue';

const props = defineProps<{
  reload: () => void;
}>();
</script>
<template>
  <Button style="width: 170px" @click="props.reload"> 读取选中文本/粘贴板 </Button>
  <slot name="agg-btn"> </slot>
</template>
