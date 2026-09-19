//! Type-directed egui widget registry and host registration API.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use super::*;

pub struct InspectWidgetContext<'a> {
    pub label: &'a str,
    pub unit: Option<InspectUnit>,
    pub hint: Option<&'a str>,
    /// Semantic role for context-sensitive presentation of otherwise identical
    /// Rust types, e.g. `Vec3` as position vs velocity vs scale.
    pub role: Option<&'a str>,
    pub number_input: Option<InspectNumberInput>,
}

impl<'a> InspectWidgetContext<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            unit: None,
            hint: None,
            role: None,
            number_input: None,
        }
    }

    pub fn from_metadata(metadata: &'a InspectFieldMetadata) -> Self {
        Self {
            label: metadata.label,
            unit: metadata.unit,
            hint: metadata.hint,
            role: metadata.role,
            number_input: metadata.number_input,
        }
    }
}

/// First-class advanced path for bespoke value presentation.
///
/// Implementations never receive a `read_only` flag. A read-only host calls
/// `show`; a host with actual authority may call `edit` and provide mutable data.
pub trait InspectorWidget<T: ?Sized> {
    fn show(&self, ui: &mut egui::Ui, value: &T, context: &InspectWidgetContext<'_>);
    fn edit(&self, ui: &mut egui::Ui, value: &mut T, context: &InspectWidgetContext<'_>) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct InspectorWidgetKey {
    type_id: TypeId,
    widget: Option<InspectWidgetId>,
}

trait ErasedInspectorWidget: Send + Sync {
    fn show(&self, ui: &mut egui::Ui, value: &dyn Any, context: &InspectWidgetContext<'_>) -> bool;
    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut dyn Any,
        context: &InspectWidgetContext<'_>,
    ) -> Option<bool>;
}

struct TypedInspectorWidget<T, W> {
    widget: W,
    marker: PhantomData<fn() -> T>,
}

impl<T, W> ErasedInspectorWidget for TypedInspectorWidget<T, W>
where
    T: 'static,
    W: InspectorWidget<T> + Send + Sync + 'static,
{
    fn show(&self, ui: &mut egui::Ui, value: &dyn Any, context: &InspectWidgetContext<'_>) -> bool {
        let Some(value) = value.downcast_ref::<T>() else {
            return false;
        };
        self.widget.show(ui, value, context);
        true
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut dyn Any,
        context: &InspectWidgetContext<'_>,
    ) -> Option<bool> {
        let value = value.downcast_mut::<T>()?;
        Some(self.widget.edit(ui, value, context))
    }
}

/// Reusable registry for value widgets supplied by the engine, a game, a mod, or
/// another package. Widget registration is separate from [`Inspect`] metadata:
/// types describe meaning; UI hosts decide how concrete Rust values are rendered.
#[derive(Resource, Default)]
pub struct InspectorWidgetRegistry {
    widgets: HashMap<InspectorWidgetKey, Box<dyn ErasedInspectorWidget>>,
}

impl InspectorWidgetRegistry {
    pub fn register<T, W>(&mut self, widget: W)
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.insert::<T, W>(None, widget);
    }

    pub fn register_named<T, W>(&mut self, id: InspectWidgetId, widget: W)
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.insert::<T, W>(Some(id), widget);
    }

    fn insert<T, W>(&mut self, id: Option<InspectWidgetId>, widget: W)
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        let key = InspectorWidgetKey {
            type_id: TypeId::of::<T>(),
            widget: id,
        };
        // Registration order is the explicit override policy: later registrations
        // replace the same `(Rust type, optional widget ID)` key. A named widget
        // remains independent from the default widget for that Rust type.
        self.widgets.insert(
            key,
            Box::new(TypedInspectorWidget::<T, W> {
                widget,
                marker: PhantomData,
            }),
        );
    }

    fn resolve(
        &self,
        type_id: TypeId,
        requested: Option<InspectWidgetId>,
    ) -> Option<&dyn ErasedInspectorWidget> {
        requested
            .and_then(|widget| {
                self.widgets
                    .get(&InspectorWidgetKey {
                        type_id,
                        widget: Some(widget),
                    })
                    .map(Box::as_ref)
            })
            .or_else(|| {
                self.widgets
                    .get(&InspectorWidgetKey {
                        type_id,
                        widget: None,
                    })
                    .map(Box::as_ref)
            })
    }

    pub fn show<T: 'static>(
        &self,
        ui: &mut egui::Ui,
        value: &T,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> bool {
        self.show_erased(ui, value, context, widget)
    }

    pub fn edit<T: 'static>(
        &self,
        ui: &mut egui::Ui,
        value: &mut T,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> Option<bool> {
        self.edit_erased(ui, value, context, widget)
    }

    pub fn show_erased(
        &self,
        ui: &mut egui::Ui,
        value: &dyn Any,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> bool {
        self.resolve(value.type_id(), widget)
            .is_some_and(|renderer| renderer.show(ui, value, context))
    }

    pub fn edit_erased(
        &self,
        ui: &mut egui::Ui,
        value: &mut dyn Any,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> Option<bool> {
        self.resolve((&*value).type_id(), widget)
            .and_then(|renderer| renderer.edit(ui, value, context))
    }
}

pub trait AppInspectorWidgetsExt {
    fn register_inspector_widget<T, W>(&mut self, widget: W) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static;

    fn register_named_inspector_widget<T, W>(
        &mut self,
        id: InspectWidgetId,
        widget: W,
    ) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static;
}

impl AppInspectorWidgetsExt for App {
    fn register_inspector_widget<T, W>(&mut self, widget: W) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.init_resource::<InspectorWidgetRegistry>();
        self.world_mut()
            .resource_mut::<InspectorWidgetRegistry>()
            .register::<T, W>(widget);
        self
    }

    fn register_named_inspector_widget<T, W>(&mut self, id: InspectWidgetId, widget: W) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.init_resource::<InspectorWidgetRegistry>();
        self.world_mut()
            .resource_mut::<InspectorWidgetRegistry>()
            .register_named::<T, W>(id, widget);
        self
    }
}
