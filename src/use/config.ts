import { Config, Mode } from '@/types'
import { LogicalSize, appWindow } from '@tauri-apps/api/window'

const defaultTargetLang = 'chinese'

const config = ref<Config>({})
onMounted(async () => {
  config.value = await readConfig()
})

export const useConfigState = createGlobalState(() => {
  const resetWindow = () => {
    appWindow.setSize(new LogicalSize(800, 600))
  }

  const getTargetLang = () => {
    return config.value.targetLang || defaultTargetLang
  }

  const setTargetLang = (lang: string) => {
    config.value.targetLang = lang
  }

  const mode = computed(() => {
    return config.value.mode || Mode.Aggregate
  })

  return {
    config,
    mode,
    resetWindow,
    getTargetLang,
    setTargetLang
  }
})
