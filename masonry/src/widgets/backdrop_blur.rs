// Copyright 2026 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use accesskit::{Node, Role};
use tracing::{Span, trace_span};
use vello::Scene;

use crate::core::{
    AccessCtx, ChildrenIds, LayoutCtx, MeasureCtx, NewWidget, NoAction, PaintCtx, PrePaintProps,
    PropertiesRef, RegisterCtx, Widget, WidgetId, WidgetMut, WidgetPod, paint_background,
    paint_border, paint_box_shadow,
};
use crate::kurbo::{Affine, Axis, Point, Size};
use crate::layout::LenReq;
use crate::peniko::{Color, Fill};

/// A container that draws a frosted-glass style blur treatment behind its child.
///
/// This uses Vello's blurred rounded-rect primitive for a high-performance approximation.
/// The child is then laid out and painted on top.
pub struct BackdropBlur {
    child: WidgetPod<dyn Widget>,
    blur_radius: f64,
    tint: Color,
    clip_content: bool,
}

// --- MARK: BUILDERS
impl BackdropBlur {
    /// Creates a backdrop blur container with a single child.
    pub fn new(child: NewWidget<impl Widget + ?Sized>) -> Self {
        Self {
            child: child.erased().to_pod(),
            blur_radius: 18.0,
            tint: Color::from_rgba8(0xff, 0xff, 0xff, 0x24),
            clip_content: true,
        }
    }

    /// Sets the blur radius, in logical pixels.
    pub fn blur_radius(mut self, blur_radius: f64) -> Self {
        self.blur_radius = blur_radius;
        self
    }

    /// Sets the tint color for the blur treatment.
    pub fn tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }

    /// Sets whether child painting should be clipped to this widget's bounds.
    pub fn clip_content(mut self, clip_content: bool) -> Self {
        self.clip_content = clip_content;
        self
    }
}

// --- MARK: WIDGETMUT
impl BackdropBlur {
    /// Replaces the child widget.
    pub fn set_child(this: &mut WidgetMut<'_, Self>, child: NewWidget<impl Widget + ?Sized>) {
        this.ctx.remove_child(std::mem::replace(
            &mut this.widget.child,
            child.erased().to_pod(),
        ));
        this.ctx.children_changed();
    }

    /// Sets the blur radius, in logical pixels.
    pub fn set_blur_radius(this: &mut WidgetMut<'_, Self>, blur_radius: f64) {
        if this.widget.blur_radius != blur_radius {
            this.widget.blur_radius = blur_radius;
            this.ctx.request_render();
        }
    }

    /// Sets the blur tint color.
    pub fn set_tint(this: &mut WidgetMut<'_, Self>, tint: Color) {
        if this.widget.tint != tint {
            this.widget.tint = tint;
            this.ctx.request_render();
        }
    }

    /// Sets whether child painting should be clipped to this widget's bounds.
    pub fn set_clip_content(this: &mut WidgetMut<'_, Self>, clip_content: bool) {
        if this.widget.clip_content != clip_content {
            this.widget.clip_content = clip_content;
            this.ctx.request_layout();
        }
    }

    /// Returns a mutable reference to the child widget.
    pub fn child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, dyn Widget> {
        this.ctx.get_mut(&mut this.widget.child)
    }
}

// --- MARK: IMPL WIDGET
impl Widget for BackdropBlur {
    type Action = NoAction;

    fn register_children(&mut self, ctx: &mut RegisterCtx<'_>) {
        ctx.register_child(&mut self.child);
    }

    fn measure(
        &mut self,
        ctx: &mut MeasureCtx<'_>,
        _props: &PropertiesRef<'_>,
        axis: Axis,
        _len_req: LenReq,
        cross_length: Option<f64>,
    ) -> f64 {
        ctx.redirect_measurement(&mut self.child, axis, cross_length)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx<'_>, _props: &PropertiesRef<'_>, size: Size) {
        ctx.run_layout(&mut self.child, size);
        ctx.place_child(&mut self.child, Point::ORIGIN);
        ctx.set_baseline_offset(ctx.child_baseline_offset(&self.child));

        if self.clip_content {
            ctx.set_clip_path(size.to_rect());
        } else {
            ctx.clear_clip_path();
        }
    }

    fn pre_paint(&mut self, ctx: &mut PaintCtx<'_>, props: &PropertiesRef<'_>, scene: &mut Scene) {
        let border_box = ctx.border_box();
        let p = PrePaintProps::fetch(ctx, props);

        // Keep shadow below the blur treatment, then apply regular background/border on top.
        paint_box_shadow(scene, border_box, p.box_shadow, p.corner_radius);

        let blur_radius = self.blur_radius.max(0.0);
        let corner_radius = p.corner_radius.radius.max(0.0);
        let shape = border_box.to_rounded_rect(corner_radius);

        if blur_radius > 0.0 {
            scene.draw_blurred_rounded_rect_in(
                &shape,
                Affine::IDENTITY,
                border_box,
                self.tint,
                corner_radius,
                blur_radius,
            );
        } else if self.tint.components[3] > 0.0 {
            scene.fill(Fill::NonZero, Affine::IDENTITY, self.tint, None, &shape);
        }

        paint_background(
            scene,
            border_box,
            p.background,
            p.border_width,
            p.corner_radius,
        );
        paint_border(
            scene,
            border_box,
            p.border_color,
            p.border_width,
            p.corner_radius,
        );
    }

    fn paint(&mut self, _ctx: &mut PaintCtx<'_>, _props: &PropertiesRef<'_>, _scene: &mut Scene) {}

    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }

    fn accessibility(
        &mut self,
        _ctx: &mut AccessCtx<'_>,
        _props: &PropertiesRef<'_>,
        _node: &mut Node,
    ) {
    }

    fn children_ids(&self) -> ChildrenIds {
        ChildrenIds::from_slice(&[self.child.id()])
    }

    fn make_trace_span(&self, id: WidgetId) -> Span {
        trace_span!("BackdropBlur", id = id.trace())
    }
}
