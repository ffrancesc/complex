<script lang="ts">
    import { ComplexPlane, DrawMode } from "../pkg";
    import { createEventDispatcher, onMount } from "svelte";

    export let functionStr: string;
    export let drawMode: DrawMode;
    export let maxIter: number = 0;

    export let width: number;
    export let height: number;

    export function zoom(fact: number) {
        plane.zoom(fact);
    }

    const dispatch = createEventDispatcher();

    let canvas: HTMLCanvasElement;
    let plane: ComplexPlane;

    $: if (plane && canvas) plane.set_function(functionStr);
    $: if (plane && canvas) {
        plane.set_draw_mode(drawMode);
        plane.set_max_iter(maxIter);

        canvas.width = width;
        canvas.height = height;
        plane.set_resolution(width, height);
    }

    onMount(() => {
        // Init plane
        plane = new ComplexPlane(
            canvas.getContext("webgl2") as WebGL2RenderingContext,
            functionStr,
            drawMode,
            maxIter
        );

        // Init draw loop
        let frame: number;
        (function draw_loop() {
            frame = requestAnimationFrame(draw_loop);
            plane.draw();
        })();

        return () => {
            cancelAnimationFrame(frame);
        };
    });
</script>

<canvas
    bind:this={canvas}
    on:mousedown={(e) => plane.on_pointer_down(e.clientX, e.clientY)}
    on:mousemove={(e) => {
        plane.on_pointer_move(e.clientX, e.clientY);
        dispatch("hover", {
            value: plane.display_value_at(e.clientX, e.clientY),
        });
    }}
    on:mouseup={() => plane.on_pointer_up()}
    on:touchstart|preventDefault={(e) =>
        plane.on_pointer_down(e.touches[0].clientX, e.touches[0].clientY)}
    on:touchmove|preventDefault={(e) =>
        plane.on_pointer_down(e.touches[0].clientX, e.touches[0].clientY)}
    on:touchend|preventDefault={() => plane.on_pointer_up()}
/>

<style>
    canvas {
        display: block;
        position: fixed;
        left: var(--left);
        top: var(--top);
    }
</style>
