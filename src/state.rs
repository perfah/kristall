use crate::util::wgpu::WGPUState;
use crate::world::World;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use crate::world::entity::component::ComponentManager;
use crate::world::entity::component::transform::Transform;
use crate::util::wgpu::model::Model;
use winit::window::Window;
use crate::world::entity::EntityContainer;
use crate::world::entity::component::model::GraphicsModel;
use winit::event::WindowEvent;
use crate::world::entity::component::camera::Camera;
use rand_core::SeedableRng;
use crate::world::entity::prefab::rand_tile::RandomTile;
use cgmath::Vector3;

pub struct State {
    backend: WGPUState,
    world: World,
    camera: ComponentManager<Camera>,
    instances: Vec<(ComponentManager<Transform>, &'static str)>,
    loaded_models: HashMap<&'static str, Model>,
    delta: Duration,
    prev_instant: Instant

    // HashMap<&'static str, Vec<Transform>
}

impl State {
    pub async fn new(window: &Window, vsync: bool) -> Self {
        let world_generator = RandomTile::from_entropy();
        let mut world = World::new(world_generator);

        Self {
            backend: WGPUState::new(window, vsync).await,
            world: world.clone(),
            camera: world.query_components(true).next().unwrap(),
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
                        &self.backend.device,
                        &self.backend.queue,
                        &self.backend.texture_bind_group_layout,
                        obj_path
                    ).ok().unwrap()
                );
            }

            self.instances.push( ((*transform).clone(), obj_path));
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
        self.camera.peek_mut(|camera| camera.aspect = new_size.width as f32 / new_size.height as f32);
        self.backend.resize(new_size);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera.peek_mut(|camera| camera.process_events(event)).unwrap()
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.prev_instant;
        self.prev_instant = now;
        if self.delta.as_millis() > 0 {
            let fps =  1000 / self.delta.as_millis();
            println!("FPS = {}",fps);
        }


        
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

        if self.delta.as_millis() == 0 {
            self.update_graphics_data();
        }
        

        self.backend.update(&self.instances, build_proj_matrix)
    

    }

    pub fn render(&mut self){
        self.backend.render(&self.instances, &self.loaded_models)
    }

}