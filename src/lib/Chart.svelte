<script lang="ts">
	let chart_points: {x: number, y: number}[] = [{x: 0, y: 0}];

  export function add_point(point: {x: number, y: number}) {
		chart_points = [...chart_points, point];
	}

  import { LayerCake, Svg } from 'layercake';
  import Line from './chart/Line.svelte';
  import AxisY from './chart/AxisY.svelte';

</script>

{Math.round(chart_points[chart_points.length - 1]?.y * 100)/100}%
<LayerCake
  x='x'
  y='y'
  data={ chart_points }
  padding={ { top: 20, left: 20 } }
  yDomain={ [0, 100] }
>
  <Svg>
    <AxisY/>
    <Line stroke={'blue'} />
  </Svg>
</LayerCake>