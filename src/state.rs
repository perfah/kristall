use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::Arc;

use crate::backend::graphics::WGPUState;
use crate::world::World;
use crate::world::entity::component::ComponentManager;
use crate::world::entity::component::transform::Transform;
use crate::backend::graphics::model::Model;
use winit::window::Window;
use crate::world::entity::EntityContainer;
use crate::world::entity::component::model::GraphicsModel;
use winit::event::WindowEvent;
use crate::world::entity::component::camera::Camera;
use rand_core::SeedableRng;
use crate::world::entity::prefab::rand_tile::RandomTile;
use cgmath::Vector3;
use crate::backend::BackendProxy;
use crate::backend::graphics::transform::TransformSink;

pub struct State {
    backend_proxy: BackendProxy,
    graphics_backend: WGPUState,
    graphics_cache: HashMap<&'static str, Vec<Arc<TransformSink>>>,
    world: World,
    camera: ComponentManager<Camera>,
    loaded_models: HashMap<&'static str, Model>,
    delta: Duration,
    prev_instant: Instant
}

impl State {
    pub async fn new(window: &Window, vsync: bool) -> Self {
        let graphics_backend = WGPUState::new(window, vsync).await;
        let backend_proxy = BackendProxy::new(graphics_backend.device.clone(), graphics_backend.queue.clone());
        
        let world_generator = RandomTile::from_entropy();
        let world = World::new(world_generator, &backend_proxy);

        let mut state = Self {
            backend_proxy,
            graphics_backend,
            graphics_cache: HashMap::new(),
            world: world.clone(),
            camera: world.query_components(true).next().unwrap(),
            loaded_models: HashMap::new(),
            delta: Duration::new(0, 0),
            prev_instant: Instant::now()
        };

        state.update_graphics_data();
        state
    }

    // TODO: Use HashMap<&str, Vec<TransformSink>>
    pub fn update_graphics_data(&mut self) {
        let mut drawables = self.world.query_entities(true)
            .map(|e| (e.component::<Transform>(), e.component::<GraphicsModel>()))
            .filter(|(a, b)| a.is_some() && b.is_some())
            .map(|(a, b)| (a.unwrap(), b.unwrap()));

        self.graphics_cache.clear();

        while let Some((ref transform, ref graphics_model)) = drawables.next() {
            let obj_path = graphics_model
                .peek(|graphics_model| graphics_model.path_to_obj)
                .expect("Graphics model: Couldn't retrieve model path!");

            if !self.loaded_models.contains_key(obj_path) {
                self.loaded_models.insert(
                    obj_path,
                    Model::load(
                        &self.graphics_backend.device,
                        &self.graphics_backend.queue,
                        &self.graphics_backend.texture_bind_group_layout,
                        obj_path
                    ).ok().unwrap()
                );
            }


            if !self.graphics_cache.contains_key(obj_path) {
                self.graphics_cache.insert(obj_path, Vec::new());
            }

            let mut current = self.graphics_cache.remove(obj_path).unwrap();
        
            current.push(transform.lock_component_for_read().sink.clone());

            self.graphics_cache.insert(obj_path, current);
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
        self.camera.peek_mut(|camera| camera.aspect = new_size.width as f32 / new_size.height as f32);
        self.graphics_backend.resize(new_size);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera.peek_mut(|camera| camera.process_events(event)).unwrap()
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.prev_instant;
        self.prev_instant = now;

        let build_proj_matrix = {
            let mut camera = self.camera.lock_component_for_write();
            camera.update();
            camera.build_view_projection_matrix()
        };

        // TODO
        
        /*
        let mut play: ComponentManager<Transform> = self.world.query_entity_by_name("player", true).next().unwrap().query_components(true).next().unwrap();
        play.peek_mut(|transform| transform.acc = Vector3{
            x: 1.0 * (rand::random::<f32>() - 0.2),
            y: 0.0,
            z: 1.0 * (rand::random::<f32>() - 0.2)
        });
*/

        //self.update_graphics_data();
        self.graphics_backend.update(build_proj_matrix)
    

    }

    pub fn render(&mut self){
        let fps =  1000 / self.delta.as_millis();

        self.graphics_backend.render(&self.graphics_cache, &self.loaded_models, fps)
    }

}