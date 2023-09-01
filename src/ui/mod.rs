use bevy::{prelude::*, ui::widget::UiImageSize};


use self::{components::{UiTransformBinding, HasUi, EntityUiRoot, Root, HealthBar}, events::{DestroyUi, BindUi}};

const X_OFFSET: f32 = -15.0;
const Y_OFFSET: f32 = -50.0;

pub mod events;
pub mod components;
pub mod plugin;
pub mod systems;

