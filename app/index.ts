import { ComplexPlane } from '../pkg';

const zoom_in_button = document.getElementById('zoom-in-button');
const zoom_out_button = document.getElementById('zoom-out-button');

const input = document.getElementById('input') as HTMLInputElement;
const niter_input = document.getElementById('niter-input') as HTMLInputElement;
const canvas = document.getElementById('canvas') as HTMLCanvasElement;
const label1 = document.getElementById('label1');
const label2 = document.getElementById('label2');
const label3 = document.getElementById('label3');

const plane = new ComplexPlane(canvas.getContext('webgl2'));

let last = performance.now();

const resize = () => {
    const width = window.innerWidth;
    const height = window.innerHeight;

    canvas.width = width;
    canvas.height = height;
    plane.set_resolution(width, height);
}

input.oninput = () => plane.set_function(input.value);
niter_input.oninput = () => plane.set_niter(parseInt(niter_input.value));
zoom_in_button.onclick = () => plane.zoom(1.3);
zoom_out_button.onclick = () => plane.zoom(1/1.3);

window.onmousedown = (e: MouseEvent) => plane.on_pointer_down(e.clientX, e.clientY);
window.onmousemove = (e: MouseEvent) => {
    label3.textContent = plane.display_value_at(e.clientX, e.clientY);
    plane.on_pointer_move(e.clientX, e.clientY)
};
window.onmouseup = (e: MouseEvent) => plane.on_pointer_up(e.x, e.y);
window.onresize = resize;


resize();

const draw_loop = () => {

    plane.draw();
    const now = performance.now();
    label1.textContent = `FPS: ${1000 / (now - last)}`;
    last = now;
    window.requestAnimationFrame(draw_loop);
}
draw_loop();