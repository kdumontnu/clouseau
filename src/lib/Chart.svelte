<script lang="ts">
  const chart_length = 1000;
  let chart_index = 0;
  let chart_points: { x: number; y: number }[] = new Array(chart_length).fill({ x: window.performance.now(), y: 0 });

  export function add_point(point: { x: number; y: number }) {
    chart_points[chart_index] = point;
    chart_index += 1;
    if (chart_index >= chart_points.length) {
      chart_index = 0;
    }
  }

  import { LayerCake, Svg } from 'layercake';
  import Line from './chart/Line.svelte';
  import AxisY from './chart/AxisY.svelte';
</script>

<LayerCake x="x" y="y" data={chart_points.slice(chart_index).concat(chart_points.slice(0,chart_index))} padding={{ top: 20, left: 20 }} yDomain={[0, 100]}>
  <Svg>
    <AxisY />
    <Line stroke={'blue'} />
  </Svg>
</LayerCake>
