<script setup lang="ts">
import Setting from '@/components/Setting/index'
import Nav from '@/components/Nav/index'
import Translation from '@/components/Translation/index'

const emit = defineEmits(['refreshAgg'])

const sendRefreshAgg = () => {
  console.log('asda')
  emit('refreshAgg')
}

const qwe = () => {
  console.log('12312312313')
}

const { showSetting } = useToogleSetting()
</script>
<template>
  <div class="app">
    <Nav>
      <template #agg>
        <button @click="sendRefreshAgg">刷新读取粘贴板</button>
      </template>
    </Nav>
    <Translation @refreshAgg="qwe" v-if="!showSetting()" />
    <Setting v-if="showSetting()" />
  </div>
</template>
