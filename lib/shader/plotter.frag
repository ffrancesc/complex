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
uniform int n_subsample;

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

// Color pixel st in (-1, 1)^2
vec3 color(vec2 st) {
    vec2 z = (st * scale - center) * vec2(1.0, -1.0);
    
    vec3 rgb = vec3(0.0);
    if (draw_mode == 1) {
        rgb = domain_color(z);
    } else if (draw_mode == 2) {
        rgb = iter_color(z);
    } else if (draw_mode == 3) {
        rgb = julia_color(z);
    } else {
        rgb = vec3(1.0, 0.0, 0.0); // unreachable
    }
    return rgb;
}


/** Return ith element after sapling n equidistant numbers between -1 and 1. 
 *  n = 2, i = 0, 1, 2     ->  -1, 0, 1                  
 *  n = 3  i = 0, 1, 2, 3  ->  -0.75, -0.25, 0.25, 0.75
 */
float subsample(int i, int n) {
    return  (2.0*float(i) - (float(n-1))) / float(n);
}

void main() {
    vec3 rgb = vec3(0.0); 
    vec2 twice_resolution = 2.0 * resolution;
    for (int i = 0; i < n_subsample; ++i) 
    for (int j = 0; j < n_subsample; ++j) {
        vec2 dst = vec2(
            subsample(i, n_subsample), 
            subsample(j, n_subsample)
        ) / twice_resolution;        
        rgb += color(st + dst);
    }
    rgb = rgb / float(n_subsample * n_subsample); // average

    fragColor = vec4(rgb, 1.0);
}