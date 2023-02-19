# Changelog

## v0.3.0 2023/02/19

1. Support `openai` translation `fanyi -p=ai -k=xxx hello`, the key is required
2. Refactor `translation`, add `Api` and `Url` type
3. Fix position bug (`center` position is not correct)

## v0.2.4 2023/02/18

1. Add more Position options `PresetPosition`
    - --Position=top-left, top-center , top-right, bottom-left, bottom-center , bottom-right, center
    - --Position=tl, tc, tr, bl, bc, br, c
2. Pin `x11-clipboard` under linux

## v0.2.3 2023/02/15

1. Fix windows command line text parse error https://github.com/fzdwx/popup-translation/pull/8
2. Upgrade wry to `0.27.0`

## v0.2.2 2023/02/14

1. Support read from selection Windows(Simulate 'ctrl c' once before reading the contents of the
   clipboard) https://github.com/fzdwx/popup-translation/pull/4
2. Support for setting the window display position `fanyi --position=100,100`

## v0.2.1 2023/02/13

1. Support overwrite show shortcut
2. Support read from selection(linux x11)

## v0.2.0 2023/02/12

1. Support daemon mode
2. Use shortcuts to get pasteboard text and translate it

## v0.1.0 2023/02/12

MVP