#version 300 es

precision mediump float;

#define E_SQRT 1.64872
#define TAU 6.28318530718

in vec2 st;
out vec4 fragColor;

uniform vec2 center;
uniform float yscale;
uniform float xscale;
uniform int niter;


vec2 norm_sq(vec2 z);

vec3 complex2rgb(vec2 z) {
    float r = sqrt(norm_sq(z).x);
    float h = atan(z.y, z.x) / TAU;
    float l = r > E_SQRT?
        1.0 - 0.5*pow( abs(2.0*(log(r) - floor(log(r)+0.5))), 0.5) :
        0.5 * r / E_SQRT;

    // hsl2rgb
    vec3 rgb = clamp( abs(mod(h*6.0+vec3(0.0,4.0,2.0),6.0)-3.0)-1.0, 0.0, 1.0 );
    return l + (rgb-0.5)*(1.0-abs(2.0*l-1.0))*1.0; // s=1.0;
}

vec2 norm_sq(vec2 z) {
    float re = z.x;
    float im = z.y;
    return vec2(re*re + im*im, 0.0);
}

vec2 add(vec2 z, vec2 w) {
    return z+w;
}

vec2 sub(vec2 z, vec2 w) {
    return z-w;
}

vec2 mul(vec2 z, vec2 w) {
    return vec2(
        z.x * w.x - z.y * w.y,
        z.x * w.y + z.y * w.x
    );
}

vec2 div(vec2 z, vec2 w) {
    return vec2(
        z.x * w.x + z.y * w.y, 
        z.y * w.x - z.x * w.y
    ) / dot(w, w);
}

vec2 f(vec2 z) {
    return /*BEGIN REPLACE*/ z /*END REPLACE*/;
}

void main() {
    vec2 z = st * vec2(xscale, yscale) - center;

    for(int i = 0; i < niter; ++i) 
        z = f(z);

    fragColor = vec4(complex2rgb(z), 1.0);
}
