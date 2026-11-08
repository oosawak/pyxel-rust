//! Node-local commands following Pyxel 3.0.0's MIT-licensed Python binding
//! (kitao/pyxel e0b528f, crates/pyxel-binding/src/cube/node.rs).
//! Contexts stay explicit and borrowed; no thread-local renderer is installed.
use super::with_node_draw_state;
use crate::{
    cube::{
        draw::{self, DrawState, Uvs},
        mesh::Material,
        scene::DrawContext,
        Mat4, Primitive, RcNode, Vec3,
    },
    Font, RcImage,
};

/// A node-bound drawing context. Coordinates are local to the node; parent
/// transforms, opacity and shading are resolved for every command.
/// Use `NodeScene::on_paint`, or construct one inside an existing `on_draw`.
pub struct NodePainter<'a> {
    ctx: &'a mut DrawContext,
    node: &'a RcNode,
}
impl<'a> NodePainter<'a> {
    pub fn new(ctx: &'a mut DrawContext, node: &'a RcNode) -> Self {
        Self { ctx, node }
    }
    pub fn node(&self) -> &RcNode {
        self.node
    }
    fn command<R>(
        &mut self,
        local: Option<&Mat4>,
        billboard: i32,
        f: impl FnOnce(&mut DrawContext, &Mat4, DrawState<'_>) -> R,
    ) -> R {
        with_node_draw_state(self.ctx, self.node, |ctx, world, mut state| {
            state.billboard = billboard;
            let world = local.map_or(*world, |local| world.mul_mat_value(local));
            f(ctx, &world, state)
        })
    }
    pub fn dither(&mut self, alpha: f32) -> &mut Self {
        self.ctx.dither_alpha = alpha.clamp(0., 1.);
        self
    }
    pub fn depth_test(&mut self, on: bool) -> &mut Self {
        self.ctx.depth_test = on;
        self
    }
    pub fn depth_write(&mut self, on: bool) -> &mut Self {
        self.ctx.depth_write = on;
        self
    }
    pub fn depth_offset(&mut self, offset: f32) -> &mut Self {
        self.ctx.depth_offset = offset;
        self
    }
    /// `None` disables decal projection, as in the official argumentless call.
    pub fn decal(&mut self, distance: Option<f32>) -> &mut Self {
        self.ctx.decal_distance = distance;
        self
    }
    pub fn shaded(&mut self, on: bool) -> &mut Self {
        self.ctx.shaded = on;
        self
    }
    pub fn pset(&mut self, pos: &Vec3, col: i32) {
        self.command(None, draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::pset(ctx, world, pos, col, state)
        });
    }
    pub fn line(&mut self, p1: &Vec3, p2: &Vec3, col: i32) {
        self.command(None, draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::line(ctx, world, p1, p2, col, state)
        });
    }
    pub fn tri(&mut self, p1: &Vec3, p2: &Vec3, p3: &Vec3, col: i32) {
        self.command(None, draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::tri(ctx, world, p1, p2, p3, col, state)
        });
    }
    pub fn trib(&mut self, p1: &Vec3, p2: &Vec3, p3: &Vec3, col: i32) {
        self.command(None, draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::trib(ctx, world, p1, p2, p3, col, state)
        });
    }
    pub fn circ(&mut self, pos: &Vec3, r: f32, col: i32) {
        self.command(None, draw::BILLBOARD_ON, |ctx, world, state| {
            draw::circ(ctx, world, pos, r, col, state)
        });
    }
    pub fn circb(&mut self, pos: &Vec3, r: f32, col: i32) {
        self.command(None, draw::BILLBOARD_ON, |ctx, world, state| {
            draw::circb(ctx, world, pos, r, col, state)
        });
    }
    pub fn sphereb(&mut self, pos: &Vec3, r: f32, col: i32) {
        self.command(None, draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::sphereb(ctx, world, pos, r, col, state)
        });
    }
    pub fn text(&mut self, pos: &Vec3, text: &str, col: i32, font: Option<&mut Font>) {
        self.command(None, draw::BILLBOARD_ON, |ctx, world, state| {
            draw::text(ctx, world, pos, text, col, font, state)
        });
    }
    pub fn sprite(
        &mut self,
        pos: &Vec3,
        img: &RcImage,
        uvs: Uvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
        rotate: f32,
    ) {
        self.command(None, draw::BILLBOARD_ON, |ctx, world, state| {
            draw::sprite(ctx, world, pos, img, uvs, w, h, colkey, rotate, state)
        });
    }
    pub fn rect(&mut self, mat: &Mat4, w: f32, h: f32, col: i32) {
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::rect(ctx, world, w, h, col, state)
        });
    }
    pub fn rectb(&mut self, mat: &Mat4, w: f32, h: f32, col: i32) {
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::rectb(ctx, world, w, h, col, state)
        });
    }
    pub fn elli(&mut self, mat: &Mat4, w: f32, h: f32, col: i32) {
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::elli(ctx, world, w, h, col, state)
        });
    }
    pub fn ellib(&mut self, mat: &Mat4, w: f32, h: f32, col: i32) {
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::ellib(ctx, world, w, h, col, state)
        });
    }
    pub fn boxb(&mut self, mat: &Mat4, size: &Vec3, col: i32) {
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::boxb(ctx, world, size, col, state)
        });
    }
    pub fn plane(
        &mut self,
        mat: &Mat4,
        img: &RcImage,
        uvs: Uvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
    ) {
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::plane(ctx, world, img, uvs, w, h, colkey, state)
        });
    }
    pub fn r#box(&mut self, mat: &Mat4, size: &Vec3, material: &Material) {
        let (color, image) = material.col_img.as_flat_and_image();
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::box_solid(
                ctx,
                world,
                size,
                color,
                image.as_ref(),
                material.colkey,
                state,
            )
        });
    }
    pub fn sphere(&mut self, pos: &Vec3, r: f32, material: &Material) {
        let (color, image) = material.col_img.as_flat_and_image();
        self.command(None, draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::sphere(
                ctx,
                world,
                pos,
                r,
                color,
                image.as_ref(),
                material.colkey,
                state,
            )
        });
    }
    /// Draw a Primitive with an additional local transform and flat/image material.
    /// Invalid primitive attributes return an error from the official rasterizer.
    pub fn prim(&mut self, mat: &Mat4, p: &Primitive, material: &Material) -> Result<(), String> {
        let (color, image) = material.col_img.as_flat_and_image();
        self.command(Some(mat), draw::BILLBOARD_OFF, |ctx, world, state| {
            draw::prim(
                ctx,
                world,
                p.mode,
                p.cull,
                &p.positions,
                (!p.indices.is_empty()).then_some(p.indices.as_slice()),
                (!p.normals.is_empty()).then_some(p.normals.as_slice()),
                (!p.uvs.is_empty()).then_some(p.uvs.as_slice()),
                color,
                image.as_ref(),
                material.colkey,
                state,
            )
            .map_err(str::to_owned)
        })
    }
}
