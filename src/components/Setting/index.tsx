import { rs } from '@/utils'
import { SetupContext } from 'vue'
import { UseDark } from '@vueuse/components'

const dark = () => {
  return (
    <UseDark>
      {{
        default: ({ isDark, toggleDark }: any) => (
          <div>
            <button
              onClick={() => {
                toggleDark()
              }}
            >
              Is Dark: {rs(isDark, 'dark', 'light')}
            </button>
          </div>
        )
      }}
    </UseDark>
  )
}

const toogleSettingDiv = (toogleSettingVal: any, toogle: Function) => {
  return (
    <button
      onClick={() => {
        toogle()
      }}
    >
      {rs(toogleSettingVal, 'Close Setting', 'Open Setting')}
    </button>
  )
}

export default (props: {}, { slots }: SetupContext) => {
  const { config, toogleSetting } = useConfigState()

  const { toogleSettingVal, toogle } = toogleSetting()

  if (!toogleSettingVal.value) return toogleSettingDiv(toogleSettingVal, toogle)
  return (
    <div>
      <div>{toogleSettingDiv(toogleSettingVal, toogle)}</div>
      <div>{dark()}</div>
    </div>
  )
}
