// tslint:disable:no-var-requires
import core from 'mathjs/core'

const math = core.create()

math.import(require('mathjs/lib/type/unit'))

// export which functions to use
export const unit = math.unit
export const createUnit = math.createUnit
