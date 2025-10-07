@group(0) @binding(0) var image: texture_2d<f32>;
@group(0) @binding(1) var linear: sampler;

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) texture_coordinates: vec2<f32>,
}

const position = array(vec2(-1., 1.), vec2(-1., -3.), vec2(3., 1.));
const texture_coordinates  = array(vec2(0., 0.), vec2(0., 2.), vec2(2., 0.));

@vertex fn vertex(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
	return VertexOutput(vec4(position[vertex_index], 0., 1.), vec2(texture_coordinates[vertex_index]));
}

@fragment fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
	return textureSample(image, linear, vec2(vertex.texture_coordinates.x, vertex.texture_coordinates.y));
}
