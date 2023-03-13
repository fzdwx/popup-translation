<script setup lang="ts">
import {IconSettings, IconBrandGoogle, IconBrandBing, IconBrandNeteaseMusic} from "@tabler/icons-vue";
import Button from "./common/Button.vue";
import { Platform } from "../types/type";
import { ref } from "vue";
const props = defineProps<{
  plat: {
    current: Platform
  },
  takes: {
    isTakes: boolean,
  },
  showSetPage: {
    show: boolean,
  },
}>();

const click = (plat: Platform) => {
  props.plat.current = plat;
};

const isTakes = () => {
  props.takes.isTakes = !props.takes.isTakes;
};

const color = ref("white");
const show = () => {
  props.showSetPage.show = !props.showSetPage.show;
  if (props.showSetPage.show) {
    color.value = "skyblue";
  } else {
    color.value = "white";
  }
}
</script>

<template>
  <div class="nav">
    <div class="btns">
      <Button class="btn" :class="{ active: plat.current === Platform.YouDao }"  @click="click(Platform.YouDao)">
        <IconBrandNeteaseMusic :size="16" color="red"></IconBrandNeteaseMusic>
        有道词典
      </Button>
      <Button class="btn" :class="{ active: plat.current ===  Platform.Bing }" @click="click(Platform.Bing)">
        <IconBrandBing :size="16" color="skyblue"></IconBrandBing>
        bing
      </Button>
      <Button class="btn" :class="{ active: plat.current ===  Platform.Google }" @click="click(Platform.Google)">
        <IconBrandGoogle :size="16" color="skyblue"></IconBrandGoogle>
        google
      </Button>
    </div>
    <div class="setting">
      <div class="takes">
        <input type="checkbox" name="takes" id="takes" @change="isTakes()">
        <label for="takes">划屏取词</label>
      </div>
      <IconSettings :color="color" @click="show"></IconSettings>
    </div>
  </div>
</template>

<style scoped>
.nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  height: 50px;
  /* background-color: #fff; */
  border-bottom: 1px solid #ccc;
}
.nav .btns {
  display: flex;
}

.active {
  background-color: rgb(91, 77, 77);
}

.btns .btn {
  margin: 5px;
  flex: 1;
}

.nav .setting {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 25%;
  cursor: pointer;
}

.setting .takes {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 4px;
  margin-right: 8px;
  font-size: 14px;
}
.takes > input {
  width: 15px;
  height: 15px;
}
</style>
