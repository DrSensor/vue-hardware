declare module 'mathjs/core' {
  import { MathJsStatic } from 'mathjs'
  namespace math {
    interface CoreOptions {
      epsilon: number
      matrix: string
      number: string
      precision: number
      predictable: boolean
      randomSeed: string
    }

    interface MathJsCore extends MathJsStatic {
      import(module: any): any
      config(options: CoreOptions): void
    }

    interface Core {
      create(options?: math.CoreOptions): MathJsCore
    }
  }

  const core: math.Core
  export default core
}
