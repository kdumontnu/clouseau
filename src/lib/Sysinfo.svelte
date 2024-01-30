<script lang="ts">
  import { onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
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

  (function update() {
    frame = requestAnimationFrame(update);

    const time = window.performance.now();
    frequency = 1000.0 / (time - last_time);

    last_time = time;

    getCpuData();

    cpu_usage.forEach((cpu, i) => {
      chart[i].add_point({ x: time, y: cpu });
    });
  })();

  onDestroy(() => {
    cancelAnimationFrame(frame);
  });
</script>

<div>Frame: {frame} ({frequency.toFixed(1)}Hz)</div>
<div>CPUs:</div>
<ul>
  {#each cpus as cpu, i}
    <li>
      <div>{cpu}: {cpu_usage[i]}</div>
      <div class="chart-container"><Chart bind:this={chart[i]} /></div>
    </li>
  {/each}
</ul>

<style>
  .chart-container {
    width: 100%;
    height: 100px;
  }
</style>
