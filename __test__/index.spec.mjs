import test from 'ava'

import { getSelectedText } from '../index.js'

test('get selected text', (t) => {
  t.is(getSelectedText(), '')
})
