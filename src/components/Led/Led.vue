<template>
  <svg :height="height" viewBox="85 -12 108 232">
    <text v-if="broken" x="103" y="45">BROKEN</text>
    <path id="bulb" fill="red" :fill-opacity="brightness" stroke="red" stroke-width="5" d="M 94.8 89 L 94.8 27.9 C 94.8 5.9 114.4 -12 138.5 -12 C 162.6 -12 182.3 5.9 182.3 27.9 L 182.3 89 L 94.8 89 Z " />
    <rect id="cathode" width="14" height="82" x="162" y="93" fill="orange" />
    <rect id="anode" width="14" height="126" x="103" y="93" fill="orange" />
    <rect id="flat-surface" width="108" height="13" x="85" y="82" fill="crimson" />
  </svg>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator'
import { unit, createUnit } from '@/math'

createUnit('px', { aliases: ['pixel', 'pixels', 'dot'] })
const baseUnit = (unitName: string) => (value: string) => unit(value).equalBase(unit(unitName))

@Component
export default class Led extends Vue {
  @Prop({ validator: baseUnit('pixel/meter'), default: '10px/mm' }) scale!: string
  @Prop({ validator: baseUnit('meter'), default: '3mm' }) size!: string

  @Prop({ validator: baseUnit('watt'), default: '43mW' }) maxPower!: string
  @Prop({ validator: baseUnit('volt'), default: '0V' }) inputVoltage!: string
  @Prop({ validator: baseUnit('ampere'), default: '0mA' }) inputCurrent!: string

  broken: boolean = false

  private led: any = {
    brightness: (Vin: number, Iin: number, Pmax: number) => Number(),
    diameter: (size: number, scale: number) => Number(),
  }

  async beforeCreate() {
    const loadWasm = await import('./led.rs')
    const wasm = await loadWasm.default()
    this.led = wasm.instance.exports
  }

  get height(): number { return this.led.diameter(
    unit(this.size).toNumber('millimeter'),
    unit(this.scale).toNumber('pixel/millimeter') * 1.3
  ) }

  get brightness(): number {
    const result = this.led.brightness(
      unit(this.inputVoltage).toNumber('volt'),
      unit(this.inputCurrent).toNumber('ampere'),
      unit(this.maxPower).toNumber('watt')
    )
    this.broken = (result > 1.00)
    return this.broken ? 0 : result
  }
}
</script>
