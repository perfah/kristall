use crate::backend::graphics::WGPUState;
use crate::world::World;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use crate::world::entity::component::ComponentManager;
use crate::world::entity::component::transform::Transform;
use crate::backend::graphics::model::Model;
use winit::window::Window;
use crate::world::entity::EntityContainer;
use crate::world::entity::component::model::GraphicsModel;
use winit::event::DeviceEvent;
use crate::backend::graphics::camera::Camera;
use crate::world::entity::component::camera::Camera as CameraComponent;
use rand_core::SeedableRng;
use crate::world::entity::prefab::rand_tile::RandomTile;
use cgmath::Vector3;

pub struct State {
    graphics_backend: WGPUState,
    world: World,
    camera: Camera,
    instances: Vec<(ComponentManager<Transform>, &'static str)>,
    loaded_models: HashMap<&'static str, Model>,
    delta: Duration,
    prev_instant: Instant
}

impl State {
    pub async fn new(window: &Window, vsync: bool) -> Self {
        let world_generator = RandomTile::from_entropy();
        let world = World::new(world_generator);
        let (camera_component, target) = world.query_entities(true)
            .map(|entity| (entity.component::<CameraComponent>(), entity.component::<Transform>()))
            .filter(|(a,b)| a.is_some() && b.is_some())
            .map(|(a,b)| (a.unwrap(), b.unwrap()))
            .next()
            .unwrap();

        let camera = Camera::new(camera_component, target);

        Self {
            graphics_backend: WGPUState::new(window, vsync).await,
            world: world.clone(),
            camera,
            instances: Vec::new(),
            loaded_models: HashMap::new(),
            delta: Duration::new(0, 0),
            prev_instant: Instant::now()
        }
    }

    pub fn update_graphics_data(&mut self) {
        let mut drawables = self.world.query_entities(true)
            .map(|e| (e.component::<Transform>(), e.component::<GraphicsModel>()))
            .filter(|(a, b)| a.is_some() && b.is_some())
            .map(|(a, b)| (a.unwrap(), b.unwrap()));

        self.instances.clear();

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

            self.instances.push( ((*transform).clone(), obj_path));
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, window: &Window){
        self.graphics_backend.resize(new_size);
        self.camera.resize(new_size, window);
    }

    pub fn input(&mut self, event: &DeviceEvent, window: &Window) -> bool {
        self.camera.process_events(event, &window)
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.prev_instant;
        self.prev_instant = now;

        let build_proj_matrix = self.camera.view_proj_matrix();
        
        let mut play: ComponentManager<Transform> = self.world.query_entity_by_name("player", true).next().unwrap().query_components(true).next().unwrap();
        play.peek_mut(|transform| transform.acc = Vector3{
            x: 2.0 * (rand::random::<f32>() - 0.5),
            y: 0.0,
            z: 2.0 * (rand::random::<f32>() - 0.5)
        });


        self.update_graphics_data();

        self.graphics_backend.update(&self.instances, build_proj_matrix);
        self.camera.update(self.delta);
    }

    pub fn render(&mut self){
        let fps =  1000 / (self.delta.as_millis() + 1u128);

        self.graphics_backend.render(&self.instances, &self.loaded_models, fps)
    }

    pub fn set_escape_status(&mut self, window: &Window, escape_status: bool) {
        self.camera.set_escape_status(window, escape_status)
    }
}