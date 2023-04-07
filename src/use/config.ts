import { Config } from '@/types'
import { LogicalSize, appWindow } from '@tauri-apps/api/window'

const config = ref<Config>()
onMounted(async () => {
  config.value = await readConfig()
})

export const useConfigState = createGlobalState(() => {
  const resetWindow = () => {
    appWindow.setSize(new LogicalSize(800, 600))
  }

  return {
    config,
    resetWindow
  }
})
