<script lang="ts" setup>
import { inject } from "vue";
import { KeyInfo, Model } from "../types/type";
import Input from "./common/Input.vue";
import Button from "./common/Button.vue";
import { saveKey2Json } from "../utils/file";
const props = defineProps<{
  keyList: KeyInfo;
  model: {
    currentModel: Model;
  };
}>();

const showSetPage = inject("showSetPage");
const showClick = () => {
  (showSetPage as { show: boolean }).show = false;
};

const save = async () => {
  await saveKey2Json(props.keyList);
};

</script>

<template>
  <div class="set">
    <div class="close-set">
      <span>翻译设置:</span>
      <span class="close-btn" @click="showClick">
        X
      </span>
    </div>
    <div class="model">
      选择模式:
      <select name="" id="" v-model="props.model.currentModel">
        <option :value="Model.ModelOne">模式一</option>
        <option :value="Model.ModelTwo">模式二</option>
      </select>
    </div>
    <Input v-for="item in props.keyList" :platform="item.platform" v-model="item.key"></Input>
    <div>
      <Button :style="{ border: '1px solid #fff' }" @click="save">保存</Button>
    </div>
  </div>
</template>

<style scoped>
.set {
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: start;
  width: 400px;
  height: 350px;
  padding: 8px;
  background-color: #1d1d1d;
  box-shadow: 1px 4px 10px rgba(255, 255, 255, 0.6);
}

.close-set {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  height: 45px;
  border-bottom: 1px solid #fff;
}

.close-btn {
  border: 1px solid #fff;
  width: 30px;
  padding: 5px;
  text-align: center;
  cursor: pointer;
}

.model {
  margin: 8px;
}

.model>select {
  width: 100px;
  height: 30px;
  margin: 8px;
  font-size: 16px;
  background-color: #1d1d1d;
  color: #fff;
}
</style>