<script lang="ts" setup>
const props = defineProps(["platform", "modelValue"]);
const emit = defineEmits(["update:modelValue"]);
const inputValue = (e:any) => {
  emit("update:modelValue", e.target.value);
}
</script>

<template>
  <div class="ipt">
    <label for="ipt"> {{ props.platform }}: </label>
    <input
      type="text"
      name="ipt"
      id="ipt"
      :value="modelValue"
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