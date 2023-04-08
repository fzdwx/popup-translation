import { SetupContext } from 'vue'
function r(
  condition: boolean,
  ifTrue: () => JSX.Element,
  ifFalse?: () => JSX.Element
): JSX.Element {
  if (condition) {
    return ifTrue()
  }
  return ifFalse ? ifFalse() : (null as unknown as JSX.Element)
}

function rs(condition: boolean, ifTrue: string, ifFalse?: string): string {
  if (condition) {
    return ifTrue
  }
  return ifFalse ? ifFalse : ''
}

type Component<T> = (props: T, ctx: SetupContext) => JSX.Element

/**
 * Define a component
 * @param component {@link Component}
 * @returns {@link Component}
 * @example
 * export default defineComponent((props: {}, { slots }: SetupContext) => {
 *  return ( <div> {slots.default?.()} </div> )
 * })
 */
function defineComponent<T>(component: Component<T>): Component<T> {
  return component
}

export { r, rs, defineComponent }
