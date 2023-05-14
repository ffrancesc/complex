#version 300 es

precision mediump float;

#define E_SQRT 1.64872
#define TAU 6.28318530718

in vec2 st;
out vec4 fragColor;

uniform int draw_mode;
uniform int max_iter;
uniform vec2 resolution;
uniform vec2 scale;
uniform vec2 center;
uniform vec2 parameter_c;

vec3 complex2rgb(vec2 z) {
    float r = length(z);
    float h = atan(z.y, z.x) / TAU;
    float l = r > E_SQRT?
        1.0 - 0.5*pow( abs(2.0*(log(r) - floor(log(r)+0.5))), 0.5) :
        0.5 * r / E_SQRT;

    // hsl2rgb
    vec3 rgb = clamp( abs(mod(h*6.0+vec3(0.0,4.0,2.0),6.0)-3.0)-1.0, 0.0, 1.0 );
    return l + (rgb-0.5)*(1.0-abs(2.0*l-1.0));
}

vec2 add(vec2 z, vec2 w) {
    return z + w;
}

vec2 sub(vec2 z, vec2 w) {
    return z - w;
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

vec2 re(vec2 z) {
    return vec2(z.x, 0.0);
}

vec2 im(vec2 z) {
    return vec2(z.y, 0.0);
}

vec2 f(vec2 z, vec2 c) {
    return /*BEGIN REPLACE*/ z /*END REPLACE*/;
}

vec3 domain_color(vec2 z) {
    vec2 fz = f(z, vec2(0.0,0.0));
    return complex2rgb(fz);
}

vec3 iter_color(vec2 c) {
    vec2 z = vec2(0.0, 0.0);
    int i = 0;
    while(length(z) < 10.0 && ++i < max_iter) {
        z = f(z, c);
    }
    if (i == max_iter) {
        return vec3(0.0);
    } else {
        float smoothed = 0.0; // log(log(length(z))) / log(2.0);
        float stab = clamp((float(i) - smoothed) / float(max_iter), 0.0, 1.0);
        return vec3(stab, 0.0, stab);
    } 
}


vec3 julia_color(vec2 z) {
    int i = 0;
    while(length(z) < 10.0 && ++i < max_iter) {
        z = f(z, parameter_c);
    }
    if (i == max_iter) {
        return vec3(0.0);
    } else {
        float smoothed = 0.0; // log(log(length(z))) / log(2.0);
        float stab = clamp((float(i) - smoothed) / float(max_iter), 0.0, 1.0);
        return vec3(stab, 0.0, stab);
    } 
}


void main() {
    // vec3 avg = vec3(0.0);
    // for (int dx = -1; dx <= 1; ++dx)
    // for (int dy = -1; dy <= 1; ++dy) {
    //     vec2 dst = 0.0*vec2(dx,dy) / (2.0*resolution);
    //     vec2 z = ((st + dst) * scale - center) * vec2(1.0, -1.0);
    //     vec3 rgb = mode == 1? domain_color(z) : iter_color(z);
    //     avg += rgb;
    // }
    vec2 z = (st * scale - center) * vec2(1.0, -1.0);
    vec3 rgb;
    if (draw_mode == 1) {
        rgb = domain_color(z);
    } else if (draw_mode == 2) {
        rgb = iter_color(z);
    } else if (draw_mode == 3) {
        rgb = julia_color(z);
    } else {
        rgb = vec3(1.0, 0.0, 0.0);
    }
    fragColor = vec4(rgb, 1.0);
}