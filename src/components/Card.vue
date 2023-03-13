<script setup lang="ts">
import Loding from "./Loading.vue";

const props = defineProps<{
  imgSrc: string;
  title: string;
  text: string;
  load: boolean;
}>();
</script>

<template>
  <div>
    <!-- title -->
    <div class="title-container">
      <img class="ico" :src="props.imgSrc" />
      <h6>{{ props.title }}</h6>
    </div>
    <!-- content  -->
    <div class="text-container target-height">
      <Loding :load="props.load">
        <!-- 
          if load then show loading
          else show text
         -->
        <div class="text">
          {{ props.text }}
        </div>
      </Loding>
    </div>
  </div>
</template>

<style scoped>
.title-container {
  display: flex;
  align-items: center;
  height: 25px;
  margin-bottom: 5px;
  margin-left: 10px;
  margin-right: 10px;
}

.ico {
  display: inline-block;
  width: 25px;
  height: 25px;
  margin-right: 5px;
}

.text-container {
  margin-left: 10px;
  margin-right: 10px;

  box-shadow: 0 0 0 1px rgba(36, 104, 193, 0.8);
  border-radius: 8px;
  border: 1px solid transparent;
  overflow: auto;
}

.text {
  padding: 10px;
}

.source-height {
  max-height: 160px;
  min-height: 100px;
}

.target-height {
  max-height: 60%;
  min-height: 100px;
}
</style>
