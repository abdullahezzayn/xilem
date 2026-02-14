// Copyright 2026 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use masonry::peniko::Color;
use masonry::widgets;

use crate::core::{Arg, MessageCtx, MessageResult, Mut, View, ViewArgument, ViewMarker};
use crate::{Pod, ViewCtx, WidgetView};

/// A frosted-glass style container that applies a backdrop blur treatment behind `child`.
///
/// This view wraps Masonry's [`BackdropBlur`](widgets::BackdropBlur) widget.
pub fn backdrop_blur<Child, State, Action>(child: Child) -> BackdropBlur<Child, State, Action>
where
    State: ViewArgument,
    Child: WidgetView<State, Action>,
{
    BackdropBlur {
        child,
        blur_radius: 18.0,
        tint: Color::from_rgba8(0xff, 0xff, 0xff, 0x24),
        clip_content: true,
        phantom: PhantomData,
    }
}

/// The [`View`] created by [`backdrop_blur`].
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct BackdropBlur<V, State, Action> {
    child: V,
    blur_radius: f64,
    tint: Color,
    clip_content: bool,
    phantom: PhantomData<fn(State) -> Action>,
}

impl<V, State, Action> BackdropBlur<V, State, Action> {
    /// Sets the blur radius, in logical pixels.
    pub fn blur_radius(mut self, blur_radius: f64) -> Self {
        self.blur_radius = blur_radius;
        self
    }

    /// Sets the tint color applied to the blur treatment.
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

impl<V, State, Action> ViewMarker for BackdropBlur<V, State, Action> {}
impl<Child, State, Action> View<State, Action, ViewCtx> for BackdropBlur<Child, State, Action>
where
    Child: WidgetView<State, Action>,
    State: ViewArgument,
    Action: 'static,
{
    type Element = Pod<widgets::BackdropBlur>;
    type ViewState = Child::ViewState;

    fn build(
        &self,
        ctx: &mut ViewCtx,
        app_state: Arg<'_, State>,
    ) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = self.child.build(ctx, app_state);
        let widget = widgets::BackdropBlur::new(child.new_widget)
            .blur_radius(self.blur_radius)
            .tint(self.tint)
            .clip_content(self.clip_content);
        (ctx.create_pod(widget), child_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
        app_state: Arg<'_, State>,
    ) {
        if self.blur_radius != prev.blur_radius {
            widgets::BackdropBlur::set_blur_radius(&mut element, self.blur_radius);
        }
        if self.tint != prev.tint {
            widgets::BackdropBlur::set_tint(&mut element, self.tint);
        }
        if self.clip_content != prev.clip_content {
            widgets::BackdropBlur::set_clip_content(&mut element, self.clip_content);
        }

        let mut child_element = widgets::BackdropBlur::child_mut(&mut element);
        let child_element = child_element.downcast();
        self.child
            .rebuild(&prev.child, view_state, ctx, child_element, app_state);
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
    ) {
        let mut child_element = widgets::BackdropBlur::child_mut(&mut element);
        let child_element = child_element.downcast();
        self.child.teardown(view_state, ctx, child_element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        message: &mut MessageCtx,
        mut element: Mut<'_, Self::Element>,
        app_state: Arg<'_, State>,
    ) -> MessageResult<Action> {
        let mut child_element = widgets::BackdropBlur::child_mut(&mut element);
        let child_element = child_element.downcast();
        self.child
            .message(view_state, message, child_element, app_state)
    }
}
