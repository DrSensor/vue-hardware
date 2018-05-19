<template>
  <svg :height="height" viewBox="85 -12 108 232">
    <path id="bulb" fill="red" :fill-opacity="brightness" stroke="red" stroke-width="5" d="M 94.8 89 L 94.8 27.9 C 94.8 5.9 114.4 -12 138.5 -12 C 162.6 -12 182.3 5.9 182.3 27.9 L 182.3 89 L 94.8 89 Z " />
    <rect id="cathode" width="14" height="82" x="162" y="93" fill="orange" />
    <rect id="anode" width="14" height="126" x="103" y="93" fill="orange" />
    <rect id="flat-surface" width="108" height="13" x="85" y="82" fill="crimson" />
  </svg>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator'
import { unit, createUnit, compile } from 'mathjs'

createUnit('px', { aliases: ['pixel', 'pixels', 'dot'] })
const baseUnit = (unitName: string) => (value: string) => unit(value).equalBase(unit(unitName))
const calculate = (expression: string) => compile(expression).eval()

@Component
export default class Led extends Vue {
  @Prop({ validator: baseUnit('pixel/meter') }) scale!: string
  @Prop({ validator: baseUnit('meter') }) size!: string

  @Prop({ validator: baseUnit('watt') }) maxPower!: string
  @Prop({ validator: baseUnit('volt') }) inputVoltage!: string
  @Prop({ validator: baseUnit('ampere') }) inputCurrent!: string

  get height(): number { return calculate(`1.3 * ${this.size}*${this.scale}`).toNumber('pixel') }
  get brightness(): number { return calculate(`${this.inputVoltage}*${this.inputCurrent} / ${this.maxPower}`) }
}
</script>
