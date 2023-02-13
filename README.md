# Popup translation

A translation tool is realized by providing the ability of webview through wry.

![img.png](.github/one.gif)

## 🚀 Getting Started

```shell
cargo install --git https://github.com/fzdwx/popup-translation

# once
fanyi hello
fanyi -p youdao 你好

# daemon mode, read from clipboard, `ctrl+shift+c` to translate
fanyi

fanyi --help

```

## 💫 Features

1. The pop-up window function realized by using the webview capability provided by wry
2. Read text from the clipboard and translate it
    - Translation of selected text is supported under **Linux(x11)**, i.e. does not need to be copied to the clipboard.
    - **Windows** and **MacOS** are not supported yet https://github.com/fzdwx/popup-translation/issues/3
3. Pass in text from the command line for translation
4. Support multiple translation platforms
    1. [Bing](https://www.bing.com/)
    2. [Youdao](https://www.youdao.com/)
    3. [YouGlish](https://youglish.com/)
    4. [Dict](https://dict.cn/)

## 🕹️ Integrated

### nvim

```lua
map("n", "<leader>fy", function()
    local word = vim.fn.expand("<cWORD>")
    local cmd = { "fanyi", word }
    vim.fn.jobstart(cmd, { detach = true })
end, { desc = "qwe" })
```

![gif](https://user-images.githubusercontent.com/65269574/218270052-0338693e-31fd-458b-ac03-f668b6ffd8d2.gif)

## 🦹 TODO

- [ ] Position Options( current version is fixed)
    - e.g: `top_left`, `top_center`, `top_right`, `center` ...
- [ ] HotKey
    - [x] open
        - [x] read from clipboard
        - [ ] read from selection
            - [x] Linux
            - [ ] Windows
            - [ ] MacOS
    - [ ] close
- [ ] Only one webview is used throughout

## 📖 Thanks

1. [manateelazycat - popweb](https://github.com/manateelazycat/popweb/blob/main/extension/dict/popweb-dict.el)
2. [wry](https://github.com/tauri-apps/wry)

## License

MIT