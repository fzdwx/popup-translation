<script setup lang="ts">
import Setting from '@/components/Setting/index'
import Nav from '@/components/Nav/index'
import Translation from '@/components/Translation/index'

const { toogleSetting } = useConfigState()
const { toogle, showSetting } = toogleSetting()
</script>
<template>
  <div class="app">
    <Nav />
    <Translation v-if="!showSetting()" />
    <Setting v-if="showSetting()" />
  </div>
</template>
