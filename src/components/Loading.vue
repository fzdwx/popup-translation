<script setup lang="ts">

const props = defineProps<{
  load: boolean
}>()

</script>

<template>
  <div v-if="props.load">
    <span class="loader" />
  </div>
  <div v-else>
    <slot />
  </div>
</template>

<style>
.loader {
  width: 100%;
  height: 4.8px;
  display: inline-block;
  position: relative;
  overflow: hidden;
}

.loader::after {
  content: '';
  width: 96px;
  height: 4.8px;
  background: #FFF;
  position: absolute;
  top: 0;
  left: 0;
  box-sizing: border-box;
  animation: hitZak 0.6s ease-in-out infinite alternate;
}

@keyframes hitZak {
  0% {
    left: 0;
    transform: translateX(-1%);
  }

  100% {
    left: 100%;
    transform: translateX(-99%);
  }
}
</style>