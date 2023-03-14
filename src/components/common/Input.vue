<script lang="ts" setup>
const props = defineProps(["platform", "modeValue"]);
const emit = defineEmits(["update:modeValue"]);
const inputValue = (e: any) => {
  emit("update:modeValue", e.target.value);
};
</script>

<template>
  <div class="ipt">
    <label for="ipt"> {{ props.platform }}: </label>
    <input
      type="text"
      name="ipt"
      id="ipt"
      :value="modeValue"
      @input="inputValue($event)"
    />
  </div>
</template>

<style scoped>
.ipt {
  margin: 8px;
}
.ipt label {
  margin-right: 5px;
}
</style>

<style scoped>
.ipt {
  margin: 8px;
}
.ipt label {
  margin-right: 5px;
}
</style>
