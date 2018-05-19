<template>
  <div id="app">
    <Led size="3mm" scale="10px/mm"
        input-voltage="3.3V" input-current="13mA"
        max-power="43mW" />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator'
import Led from '@/components/Led.vue'

@Component({ components: { Led } })
export default class App extends Vue {}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
