use crate::world::system::{System, SysEnvComponentMut, SystemRuntimeError, SysEnvComponent};
use crate::world::entity::component::{Component, ComponentManager, ComponentWriteAccess};
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::{Entity, EntityContainer};
use std::time::Duration;

pub struct TranslateSystem {
    root: Option<Entity>,
}

impl<'a> System<'a> for TranslateSystem {
    type Environment = ();

    fn new() -> Self{
        Self { root: None }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        self.root = Some(source.clone().into());
        Result::Ok(())
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok(())
    }

    fn on_run(&self, _: Self::Environment, _: Duration) {
        if let Some(ref parent) = self.root {
            let mut acc_offsets = Vec::new();
            TranslateSystem::update_gpu_buffers(parent, &mut acc_offsets);
        }
    }
}

impl TranslateSystem {
    fn update_gpu_buffers(parent: &Entity, acc_offsets: &mut Vec<Transform>) {
        if let Some(mgr) = parent.component::<Transform>() {
            acc_offsets.push((*mgr.lock_component_for_write()).clone());
        }

        if let Some(mgr) = parent.component::<GraphicsModel>() {
            let absolute_transform = acc_offsets.iter().fold(Transform::new(), |acc, x| acc.with_offset(x));
            (*mgr.lock_component_for_write()).view.translate(absolute_transform);
        }

        let mut iter = parent.query_direct_children();
        while let Some(child) = iter.next() {
            TranslateSystem::update_gpu_buffers(&child, acc_offsets);
        }

        acc_offsets.pop();
    }
}