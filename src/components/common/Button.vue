<script setup lang="ts">
</script>

<template>
  <button>
    <slot></slot>
  </button>
</template>

<style scoped>
button {
  width: 100px;
  height: 40px;
  padding: 4px;
  margin: 4px;
  border-radius: 8px;
  border: 1px solid transparent;
  font-size: 1em;
  font-weight: 500;
  color: #ffffff;
  background-color: #1d1d1d;
}
</style>