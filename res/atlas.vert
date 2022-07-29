#version 460 core

layout(location = 0) in vec2 vPosition;
layout(location = 1) in vec2 vTranslation;
layout(location = 2) in vec2 vScale;
layout(location = 3) in vec4 vColor;

out vec4 fColor;

layout(location = 0) uniform vec2 size;

vec2 map(const vec2 value, const vec2 fromPosition, const vec2 fromSize, const vec2 toPosition, const vec2 toSize) {
  return (value - fromPosition) / fromSize * toSize + toPosition;
}

void main() {
  fColor = vColor;
  gl_Position = vec4(map(vPosition * vScale + vTranslation, vec2(0), size, vec2(-1, 1), vec2(2, -2)), 0, 1);
}
