use sf2g::{
    graphics::{Color, FloatRect, IntRect, Vertex},
    system::Vector2,
};

/// Convenience methods for a `Vec` of vertices (vertex array).
pub trait VertexVecExt {
    /// Push a quad to a vector of quad primitives
    fn push_quad(&mut self, dst_rect: FloatRect, texture_rect: IntRect, color: Color);
}

impl VertexVecExt for Vec<Vertex> {
    fn push_quad(&mut self, dst_rect: FloatRect, texture_rect: IntRect, color: Color) {
        self.extend_from_slice(&[
            Vertex {
                position: Vector2::new(dst_rect.left, dst_rect.top),
                color,
                tex_coords: Vector2::new(texture_rect.left as f32, texture_rect.top as f32),
            },
            Vertex {
                position: Vector2::new(dst_rect.left + dst_rect.width, dst_rect.top),
                color,
                tex_coords: Vector2::new(
                    (texture_rect.left + texture_rect.width) as f32,
                    texture_rect.top as f32,
                ),
            },
            Vertex {
                position: Vector2::new(
                    dst_rect.left + dst_rect.width,
                    dst_rect.top + dst_rect.height,
                ),
                color,
                tex_coords: Vector2::new(
                    (texture_rect.left + texture_rect.width) as f32,
                    (texture_rect.top + texture_rect.height) as f32,
                ),
            },
            Vertex {
                position: Vector2::new(dst_rect.left, dst_rect.top + dst_rect.height),
                color,
                tex_coords: Vector2::new(
                    texture_rect.left as f32,
                    (texture_rect.top + texture_rect.height) as f32,
                ),
            },
        ]);
    }
}
