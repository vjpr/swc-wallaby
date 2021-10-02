import {loadBinding} from '@node-rs/helper'
import path from 'path'
import {dirname} from 'path'
import {fileURLToPath} from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

/**
 * __dirname means load native addon from current dir
 * 'next-swc' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `next-swc.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `next-swc-[PLATFORM]`
 */
const bindings = loadBinding(
  path.join(__dirname, './native'),
  'swc-wallaby',
  '@live/swc-wallaby'
)

export async function transform(src, options) {
  const isModule = typeof src !== 'string'
  options = options || {}

  if (options?.jsc?.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax ?? 'ecmascript'
  }

  const {plugin, ...newOptions} = options

  if (plugin) {
    const m =
      typeof src === 'string'
        ? await this.parse(src, options?.jsc?.parser)
        : src
    return this.transform(plugin(m), newOptions)
  }

  return bindings.transform(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(newOptions)
  )
}

export function transformSync(src, options) {
  const isModule = typeof src !== 'string'
  options = options || {}

  if (options?.jsc?.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax ?? 'ecmascript'
  }

  const {plugin, ...newOptions} = options

  if (plugin) {
    const m =
      typeof src === 'string' ? this.parseSync(src, options?.jsc?.parser) : src
    return this.transformSync(plugin(m), newOptions)
  }

  return bindings.transformSync(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(newOptions)
  )
}

function toBuffer(t) {
  return Buffer.from(JSON.stringify(t))
}

export async function minify(src, opts) {
  return bindings.minify(toBuffer(src), toBuffer(opts ?? {}))
}

export function minifySync(src, opts) {
  return bindings.minifySync(toBuffer(src), toBuffer(opts ?? {}))
}

//module.exports.transform = transform
//module.exports.transformSync = transformSync
//module.exports.minify = minify
//module.exports.minifySync = minifySync
