import { Config } from '@/types'
import { useToggle } from '@vueuse/core'
import { LogicalSize, appWindow } from '@tauri-apps/api/window'

export const useConfigState = createGlobalState(() => {
  const config = ref<Config>()

  onMounted(async () => {
    config.value = await readConfig()
  })

  const toogleSettingVal = ref<Boolean>(false)
  const showSetting = () => {
    return toogleSettingVal.value
  }
  const toogleSetting = () => {
    const toogle = useToggle(toogleSettingVal)
    return {
      toogleSettingVal,
      toogle,
      showSetting
    }
  }

  const resetWindow = () => {
    appWindow.setSize(new LogicalSize(800, 600))
  }

  return {
    config,
    toogleSetting,
    resetWindow
  }
})
