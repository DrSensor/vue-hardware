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

@Component
export default class Led extends Vue {
  @Prop() scale!: string
  @Prop() size!: string

  @Prop() inputVoltage!: string
  @Prop() inputCurrent!: string
  @Prop() maxPower!: string

  get height(): number { return 232 }
  get brightness(): number { return 50 }
}
</script>
