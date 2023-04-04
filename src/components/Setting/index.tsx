import { rs } from '@/utils'
import { SetupContext } from 'vue'
import { UseDark } from '@vueuse/components'

export default (props: {}, { slots }: SetupContext) => {
  const { config } = useConfigState()
  return (
    <div>
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
    </div>
  )
}
