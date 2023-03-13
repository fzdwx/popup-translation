<script lang="ts" setup>
import { KeyInfo } from "../types/type";
import Input from "./common/Input.vue";
const props = defineProps<{
  keyList: KeyInfo;
  model: {
    currentModel: number;
  };
}>();
</script>

<template>
  <div class="set">
    <div class="model">
      选择模式:
      <select name="" id="" v-model="props.model.currentModel">
        <option :value="1">模式一</option>
        <option :value="2">模式二</option>
      </select>
    </div>
    <Input
      v-for="item in props.keyList"
      :platform="item.platform"
      v-model="item.key"
    ></Input>
  </div>
</template>

<style scoped>
.set {
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: start;
  width: 400px;
  height: calc(100% - 50px);
  padding: 8px;
  background-color: #1d1d1d;
  box-shadow: 1px 4px 10px rgba(255, 255, 255, 0.6);
}
.model {
  margin: 8px;
}
.model > select {
  width: 100px;
  height: 30px;
  margin: 8px;
  font-size: 16px;
  background-color: #1d1d1d;
  color: #fff;
}
</style>
