<script lang="ts">
    import { DrawMode, JsComplex } from "../pkg";
    import Plotter from "./Plotter.svelte";
    import Toggle from "./Toggle.svelte";

    const ZOOM_FACTOR: number = 1.3;

    let mainPlotter: Plotter;
    let fractalSwitchEnabled = false;
    let drawMode = DrawMode.ParameterStability;
    let maxIter = 40;
    // crazy one: z*z*z-1*i-0.21
    // good one: (z*z+1)/(z*z-1)+z
    let functionStr = "z*z+c";

    $: drawMode = fractalSwitchEnabled
        ? DrawMode.ParameterStability
        : DrawMode.DomainColouring;

    let juliaPlotter: Plotter;
    let juliaPlotterWidth: number;
    let juliaPlotterHeight: number;

    $: {
        const r = 240 / 300;
        const w = Math.min(300, windowWidth / 3);
        const h = Math.min(240, windowHeight / 3);
        if (w * r < h) {
            juliaPlotterWidth = w;
            juliaPlotterHeight = w * r;
        } else {
            juliaPlotterWidth = h / r;
            juliaPlotterHeight = h;
        }
    }
    function onPick(e: { detail: { complex: JsComplex } }) {
        if (juliaPlotter) juliaPlotter.setParameterC(e.detail.complex);
    }
    let windowWidth: number;
    let windowHeight: number;
</script>

<svelte:window bind:innerWidth={windowWidth} bind:innerHeight={windowHeight} />

<main>
    <Plotter
        bind:this={mainPlotter}
        --left="0"
        --top="0"
        bind:width={windowWidth}
        bind:height={windowHeight}
        {maxIter}
        {drawMode}
        {functionStr}
        on:pick={onPick}
    />

    <div class="fractal-container input">
        <div class="fractal-inner-toggle">
            <Toggle bind:checked={fractalSwitchEnabled} /><span>fractal</span>
        </div>
        {#if fractalSwitchEnabled}
            <input
                bind:value={maxIter}
                type="range"
                min="0"
                max="300"
                step="10"
            />
        {/if}
    </div>

    {#if fractalSwitchEnabled}
        <Plotter
            bind:this={juliaPlotter}
            --right="20px"
            --top="140px"
            width={juliaPlotterWidth}
            height={juliaPlotterHeight}
            drawMode={DrawMode.Julia}
            {maxIter}
            {functionStr}
        />
    {/if}

    <div class="zoom-container input">
        <button on:click={() => mainPlotter.zoom(ZOOM_FACTOR)}>+</button>
        <button on:click={() => mainPlotter.zoom(1 / ZOOM_FACTOR)}>−</button>
    </div>

    <div class="function-container input">
        f(z)=<span
            bind:textContent={functionStr}
            class="function-inner"
            contenteditable
        />
    </div>
</main>

<style>
    .input {
        color: white;
        font-size: xx-large;
        letter-spacing: 10px;
        font-family: monospace;
        background-color: rgba(162, 162, 162, 0.6);
        padding: 6px;
        border-radius: 4px;
    }

    .fractal-container {
        position: absolute;
        top: 30px;
        right: 20px;
        z-index: 1;
        display: flex;
        flex-direction: column;
    }

    .fractal-container input {
        width: 100%;
    }

    .fractal-inner-toggle {
        align-self: flex-end;
        margin: 6px;
    }

    .fractal-inner-toggle span {
        align-self: right;
        font-size: x-large;
        letter-spacing: normal;
        margin-left: 14px;
    }

    .zoom-container {
        position: absolute;
        bottom: 30px;
        right: 20px;
        z-index: 1;
        display: flex;
        flex-direction: column;
    }

    .zoom-container button {
        border-radius: 4px;
        color: white;
        background: none;
        font-size: inherit;
        padding: 0px 10px;
        border: none;
    }

    .zoom-container button:hover {
        background: rgb(153, 153, 153);
    }

    .zoom-container button:active {
        background: rgb(190, 190, 190);
    }

    .function-container {
        position: absolute;
        bottom: 0;
        bottom: 30px;
        left: 20px;
        z-index: 1;
    }

    .function-inner {
        padding: 2px;
        border: none;
    }

    .function-inner {
        outline: none;
    }
</style>
