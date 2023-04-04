import { Config } from '@/types'

export const useConfigState = createGlobalState(() => {
  const config = ref<Config>()

  onMounted(async () => {
    config.value = await readConfig()
  })

  return {
    config
  }
})
