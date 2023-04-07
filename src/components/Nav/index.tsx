export default () => {
  const { toogleSetting } = useConfigState()
  const { toogle } = toogleSetting()

  return (
    <div class="flex flex-row">
      <div class="basis-1/4"></div>
      <div class="basis-1/4"></div>
      <div class=" basis-1/2">
        <div class="flex justify-end">
          <button type="button" onClick={() => toogle()}>
            设置
          </button>
        </div>
      </div>
    </div>
  )
}
