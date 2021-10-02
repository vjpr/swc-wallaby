import {transformSync} from '../index.js'

const res = transformSync("var a = b ? c() : d();")
console.log(res)
