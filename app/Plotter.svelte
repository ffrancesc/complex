<script lang="ts">
    import { Plotter, DrawMode, JsComplex } from "../pkg";
    import { createEventDispatcher, onMount } from "svelte";

    export let functionStr: string;
    export let drawMode: DrawMode;
    export let maxIter: number = 0;

    export let width: number;
    export let height: number;

    export function zoom(zoomFactor: number) {
        plotter.zoom(zoomFactor);
    }
    export function setParameterC(c: JsComplex) {
        plotter.set_parameter_c(c);
    }

    const dispatch = createEventDispatcher();
    function dispatchPick(clientX: number, clientY: number) {
        dispatch("pick", {
            complex: plotter.get_complex_at(clientX, clientY),
        });
    }

    let canvas: HTMLCanvasElement;
    let plotter: Plotter;

    $: if (plotter) plotter.set_function(functionStr);
    $: if (plotter && canvas) {
        plotter.set_draw_mode(drawMode);
        plotter.set_max_iter(maxIter);

        canvas.width = width;
        canvas.height = height;
        plotter.set_resolution(width, height);
    }

    onMount(() => {
        // Init plane
        plotter = new Plotter(
            canvas.getContext("webgl2") as WebGL2RenderingContext,
            functionStr,
            drawMode,
            maxIter
        );

        // Init draw loop
        let frame: number;
        (function draw_loop() {
            frame = requestAnimationFrame(draw_loop);
            plotter.draw();
        })();

        return () => {
            cancelAnimationFrame(frame);
        };
    });
</script>

<canvas
    bind:this={canvas}
    on:mousedown={(e) => plotter.on_pointer_down(e.clientX, e.clientY)}
    on:mousemove={(e) => {
        plotter.on_pointer_move(e.clientX, e.clientY);
        dispatchPick(e.clientX, e.clientY);
    }}
    on:mouseup={() => plotter.on_pointer_up()}
    on:touchstart|preventDefault={(e) => {
        const [x, y] = [e.touches[0].clientX, e.touches[0].clientY];
        plotter.on_pointer_down(x, y);
        dispatchPick(x, y);
    }}
    on:touchmove|preventDefault={(e) =>
        plotter.on_pointer_move(e.touches[0].clientX, e.touches[0].clientY)}
    on:touchend|preventDefault={() => plotter.on_pointer_up()}
/>

<style>
    canvas {
        position: absolute;
        left: var(--left);
        right: var(--right);
        top: var(--top);
    }
</style>
