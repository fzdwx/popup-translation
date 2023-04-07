const toogleSettingVal = ref<Boolean>(false)

export const useToogleSetting = () => {
  const showSetting = () => {
    return toogleSettingVal.value
  }

  const toogle = useToggle(toogleSettingVal)

  return {
    toogleSettingVal,
    toogle,
    showSetting
  }
}
