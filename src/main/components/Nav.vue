<script setup lang="ts">
import { IconSettings } from '@tabler/icons-vue';

import { Platform } from '../../types/type';
import { ref, watchEffect } from 'vue';
const props = defineProps<{
  plat: {
    current: Platform;
  };
  takes: {
    isTakes: boolean;
  };
  showSetPage: {
    show: boolean;
  };
}>();

const isTakes = () => {
  props.takes.isTakes = !props.takes.isTakes;
};

const color = ref('white');
const show = () => {
  props.showSetPage.show = !props.showSetPage.show;
  color.value = 'skyblue';
};

watchEffect(() => {
  if (!props.showSetPage.show) {
    color.value = 'white';
  }
});
</script>

<template>
  <div class="nav">
    <slot name="platform_link"></slot>
    <div class="setting">
      <div class="takes">
        <input type="checkbox" name="takes" id="takes" @change="isTakes()" checked />
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
  padding: 8px;
  /* background-color: #fff; */
  border-bottom: 1px solid #ccc;
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
