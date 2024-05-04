node-get-selected-text
======================

[![CI](https://github.com/yetone/node-get-selected-text/actions/workflows/CI.yml/badge.svg)](https://github.com/yetone/node-get-selected-text/actions/workflows/CI.yml) [![NPM Version](https://img.shields.io/npm/v/node-get-selected-text)](https://www.npmjs.com/package/node-get-selected-text)

A tiny Node.js library that allows you to easily obtain selected text across all platforms (macOS, Windows, Linux).

Corresponding Rust package: [get-selected-text](https://github.com/yetone/get-selected-text)

## Usage

```typescript

import { getSelectedText } from 'node-get-selected-text'

const selectedText = getSelectedText()
console.log('selected text: ', selectedText)
```

## How does it work?

### macOS

Prioritize using the A11y API to obtain selected text. If the application does not comply with the A11y API, simulate pressing cmd+c to borrow from the clipboard to get the selected text.

To avoid annoying Alert sounds when simulating pressing cmd+c, it will automatically mute the Alert sound (Only the Alert sound is muted, it won't affect the volume of listening to music and watching videos). The volume of the Alert sound will be restored after releasing the key.

Therefore, on macOS, you need to grant accessbility permissions in advance. You can use this package: [node-mac-permissions](https://github.com/codebytere/node-mac-permissions#permissionsaskforaccessibilityaccess)

### Windows + Linux

Simulate pressing ctrl+c to use the clipboard to obtain the selected text.

