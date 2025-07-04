// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use masonry::widgets;

use crate::core::{Arg, MessageCtx, MessageResult, Mut, View, ViewArgument, ViewMarker};
use crate::{Pod, ViewCtx, WidgetView};

/// A view which puts `child` into a scrollable region.
///
/// This corresponds to the Masonry [`Portal`](masonry::widgets::Portal) widget.
pub fn portal<Child, State, Action>(child: Child) -> Portal<Child, State, Action>
where
    State: ViewArgument,
    Child: WidgetView<State, Action>,
{
    Portal {
        child,
        // --- MARK: Modified ---
        constrain_horizontal: false,
        constrain_vertical: false,
        must_fill: false,
        // --- MARK: Modified ---
        right_to_left: false,
        phantom: PhantomData,
    }
}

/// The [`View`] created by [`portal`].
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct Portal<V, State, Action> {
    child: V,
    // --- MARK: Modified ---
    constrain_horizontal: bool,
    constrain_vertical: bool,
    must_fill: bool,
    // --- MARK: Modified ---
    /// The direction of the app language. If it's right to left,
    /// the vertical scrollbar will be placed at the left side of the portal.
    right_to_left: bool,
    phantom: PhantomData<fn(State) -> Action>,
}

// --- MARK: Modified ---
impl<V, State, Action> Portal<V, State, Action> {
    /// Builder-style method to set whether the child must fill the view.
    ///
    /// If `false` (the default) there is no minimum constraint on the child's
    /// size. If `true`, the child is passed the same minimum constraints as
    /// the `Portal`.
    pub fn content_must_fill(mut self, must_fill: bool) -> Self {
        self.must_fill = must_fill;
        self
    }

    // --- MARK: Modified ---
    /// Builder-style method to set the right to left direction of the app.
    /// 
    /// This will influence whether the vertical scrollbar is placed to the right
    /// side or to the left side of the portal.
    pub fn with_rtl(mut self, right_to_left: bool) -> Self {
        self.right_to_left = right_to_left;
        self
    }
}

impl<V, State, Action> ViewMarker for Portal<V, State, Action> {}
impl<Child, State, Action> View<State, Action, ViewCtx> for Portal<Child, State, Action>
where
    Child: WidgetView<State, Action>,
    State: ViewArgument,
    Action: 'static,
{
    type Element = Pod<widgets::Portal<Child::Widget>>;
    type ViewState = Child::ViewState;

    fn build(
        &self,
        ctx: &mut ViewCtx,
        app_state: Arg<'_, State>,
    ) -> (Self::Element, Self::ViewState) {
        // The Portal `View` doesn't get any messages directly (yet - scroll events?), so doesn't need to
        // use ctx.with_id.
        let (child, child_state) = self.child.build(ctx, app_state);
        let widget_pod = ctx.create_pod(
            widgets::Portal::new(child.new_widget)
                .content_must_fill(self.must_fill)
                .with_rtl(self.right_to_left)
        );
        (widget_pod, child_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
        app_state: Arg<'_, State>,
    ) {
        if self.constrain_horizontal != prev.constrain_horizontal {
            widgets::Portal::set_constrain_horizontal(&mut element, self.constrain_horizontal);
        }
        if self.constrain_vertical != prev.constrain_vertical {
            widgets::Portal::set_constrain_vertical(&mut element, self.constrain_vertical);
        }
        if self.must_fill != prev.must_fill {
            widgets::Portal::set_content_must_fill(&mut element, self.must_fill);
        }
        
        let child_element = widgets::Portal::child_mut(&mut element);
        self.child
            .rebuild(&prev.child, view_state, ctx, child_element, app_state);
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
    ) {
        let child_element = widgets::Portal::child_mut(&mut element);
        self.child.teardown(view_state, ctx, child_element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        message: &mut MessageCtx,
        mut element: Mut<'_, Self::Element>,
        app_state: Arg<'_, State>,
    ) -> MessageResult<Action> {
        let child_element = widgets::Portal::child_mut(&mut element);
        self.child
            .message(view_state, message, child_element, app_state)
    }
}
