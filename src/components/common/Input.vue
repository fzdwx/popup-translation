<script lang="ts" setup>
  const props = defineProps(["platform", "modelValue"]);
defineEmits(['update:modelValue'])
</script>

<template>
  <div class="ipt">
    <label for="ipt">
      {{props.platform}}:
    </label>
    <input type="text" name="ipt" id="ipt" :value="modelValue"
    @input="$emit('update:modelValue',$event.target.value)"/>
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