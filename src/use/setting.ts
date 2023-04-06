import { Config } from '@/types'
import { useToggle } from '@vueuse/core'

export const useConfigState = createGlobalState(() => {
  const config = ref<Config>()

  onMounted(async () => {
    config.value = await readConfig()
  })

  const toogleSettingVal = ref<Boolean>(false)
  const toogleSetting = () => {
    const toogle = useToggle(toogleSettingVal)

    return {
      toogleSettingVal,
      toogle
    }
  }

  return {
    config,
    toogleSetting
  }
})
