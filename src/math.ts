// tslint:disable:no-var-requires
import core from 'mathjs/core'

const math = core.create()

math.import(require('mathjs/lib/type/unit'))
math.import(require('mathjs/lib/expression/function')) // compile, eval
math.import(require('mathjs/lib/function/arithmetic')) // basic arithmetic like divide, exponent, etc

// export which functions to use
export const unit = math.unit
export const createUnit = math.createUnit
export const compile = math.compile
