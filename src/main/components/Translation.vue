<script setup lang="ts">
import { inject } from 'vue';
import AggTranslation from './mode/aggregate/Translation.vue';
import SplitTranslation from './mode/split/Translation.vue';

import { Mode } from '../../types/type';

const mode = inject<{ currentMode: Mode }>('mode');
</script>

<template>
  <AggTranslation v-if="mode?.currentMode === Mode.Aggregate" />
  <SplitTranslation v-else />
</template>
