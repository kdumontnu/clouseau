<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Chart from './Chart.svelte';

  let frequency = 0.0;

  let last_time = window.performance.now();

  let frame: number;

  let cpus: string[] = [];
  let chart: Chart[] = [];

  async function getCpuInfo() {
    cpus = await invoke('cpu_info');
  }

  getCpuInfo();

  let cpu_usage: number[] = [];

  async function getCpuData() {
    cpu_usage = await invoke('cpu_usage');
  }

  // Get cpu usage every 100ms
  setTimeout(function update() {
    frame = requestAnimationFrame(update);

    const time = window.performance.now();
    frequency = 1000.0 / (time - last_time);

    last_time = time;

    getCpuData();

    cpu_usage.forEach((cpu, i) => {
      chart[i].add_point({ x: time, y: cpu });
    });
  }, 100);
</script>

<div>Frame: {frame} ({frequency.toFixed(1)}Hz)</div>
<div>CPUs:</div>
<ul class="resource-list">
  {#each cpus as cpu, i}
    <li>
      <div>{cpu}: {Math.round(cpu_usage[i] * 100) / 100}%</div>
      <div class="chart-container"><Chart bind:this={chart[i]} /></div>
    </li>
  {/each}
</ul>

<style>
  .chart-container {
    width: 100%;
    height: 100px;
  }
  .resource-list {
    list-style-type: none;
    padding-left: 40px;
    padding-right: 40px;
  }
</style>
