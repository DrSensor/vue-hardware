import { Unit } from 'mathjs';

declare module 'mathjs' {
  interface MathJSON {
    mathjs?: string;
    value: number;
    unit: string;
    fixPrefix?: boolean;
  }

  interface Unit {
    clone(): Unit;
    equalBase(unit: Unit): boolean;
    equals(unit: Unit): boolean;
    format(options: FormatOptions): string;
    fromJSON(json: MathJSON): Unit;
    toJSON(): MathJSON;
    splitUnit(parts: ReadonlyArray<string>): Unit[];
    toNumeric(unit: string): number | Fraction | BigNumber;
    toSI(): Unit;
    toString(): string;
  }
}
