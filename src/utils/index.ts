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

export { r, rs }
