import { SetupContext } from 'vue'
import Agg from './Agg'
import { Mode } from '@/types'
import Split from './Split'

export default defineComponent((props: {}, { slots }: SetupContext) => {
  const { mode } = useConfigState()

  return (
    <div>
      {r(mode.value == Mode.Aggregate, () => (
        <Agg />
      ))}

      {r(mode.value == Mode.Split, () => (
        <Split />
      ))}
    </div>
  )
})
