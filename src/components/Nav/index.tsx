export default () => {
  const { toogleSetting } = useConfigState()

  return (
    <div class="flex flex-row">
      <div class="basis-1/4"></div>
      <div class="basis-1/4"></div>
      <div class=" basis-1/2">
        <div class="flex justify-end">
          <button type="button">设置</button>
        </div>
      </div>
    </div>
  )
}
