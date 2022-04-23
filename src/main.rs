extern crate oidn;

use std::sync::Arc;

use glam::DVec3;
use glam::UVec2;
use threadpool::ThreadPool;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::utility::*;
use crate::ray::*;

mod utility;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
//mod material;

const THREADS: usize = 8;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 3840;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
pub const SAMPLE_COUNT: i32 = 128;
const RAY_DEPTH: usize = 8;
const DENOISER: bool = true;

//static mut PIXELS: [(f64, f64, f64); IMAGE_WIDTH * IMAGE_HEIGHT] = [(0f64, 0f64, 0f64); IMAGE_WIDTH * IMAGE_HEIGHT];

struct ThreadJob {
    pixel: UVec2,
    world: Arc<HittableList<Sphere>>,
    camera: Arc<Camera>,
}

impl ThreadJob {
    pub fn run(&self) -> (Color, DVec3) {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        let mut pixel_normal = DVec3::new(0.0, 0.0, 0.0);

        for _ in 0..SAMPLE_COUNT {
            let u = ((self.pixel.y as f64) + rand::random::<f64>() ) / (IMAGE_WIDTH - 1) as f64;
            let v = ((self.pixel.x as f64) + rand::random::<f64>() ) / (IMAGE_HEIGHT - 1) as f64;

            let ray = self.camera.get_ray(u, v);
            let (clr, normal) = ray_color(&ray, &self.world, RAY_DEPTH);
            pixel_color += clr;
            pixel_normal += normal;
        }

        (vec3_normalize(pixel_color), vec3_normalize(pixel_normal))
    }
}

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(1.0, -0.15, -1.0), 0.3));
    world.add(Sphere::new(Point3::new(-1.0, -0.25, -1.0), 0.2));
    world.add(Sphere::new(Point3::new(0.0, -0.1, -1.0), 0.4));
    world.add(Sphere::new(Point3::new(20.5, 15.0, -25.0), 20.0));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let world_ptr = Arc::new(world);

    // camera
    let camera = Camera::new();
    let camera_ptr = Arc::new(camera);

    // render
    println!("Creating jobs!");

    let mut jobs: Vec<ThreadJob> = vec![];

    for j in 0..IMAGE_HEIGHT {
        for i in (0..IMAGE_WIDTH).rev() {
            jobs.push(ThreadJob { 
                pixel: UVec2::new(j as u32, i as u32),
                world: world_ptr.clone(), 
                camera: camera_ptr.clone(),
            });
        }
    }

    let pool = ThreadPool::new(THREADS);
    let (tx, rx) = std::sync::mpsc::channel();
    let jobs_len = jobs.len();

    let render_start = std::time::Instant::now();

    println!("Render queue created.");
    println!("Picture Size: {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("Samples: {}", SAMPLE_COUNT);
    println!("Light Bounces: {}", RAY_DEPTH);
    println!("Jobs: {}", jobs_len);
    println!("Threads: {}", THREADS);
    println!("Starting to render now!");

    let mut counter = 0;

    for job in jobs {
        let tx = tx.clone();
        pool.execute(move || {
            let res = job.run();
            tx.send((job.pixel, res, counter)).expect("channel will be there waiting for the pool");
        });
        counter += 1;
    }

    pool.join();
    let render_end = std::time::Instant::now();
    let render_time = render_end.duration_since(render_start);

    let rays_per_milli = (jobs_len * (SAMPLE_COUNT as usize) * RAY_DEPTH) / render_time.as_millis() as usize;
    println!("Rendering done! Took {}s", render_time.as_millis() as f64 / 1000.0);
    println!("Rays per millisecond: {}", rays_per_milli);

    let mut data: Vec<(UVec2, (DVec3, DVec3), i32)> = rx.iter().take(jobs_len).collect();
    data.sort_by(|a, b| b.2.cmp(&a.2));

    let mut image: Vec<f32> = vec![];

    for response in data {
        image.push(response.1.0.x as f32);
        image.push(response.1.0.y as f32);
        image.push(response.1.0.z as f32);
    }

    if DENOISER {
        println!("Starting denoiser pass");
        let mut filter_output = vec![0.0f32; image.len()];
        let device = oidn::Device::new();

        oidn::RayTracing::new(&device)
            .srgb(false)
            .image_dimensions(IMAGE_WIDTH, IMAGE_HEIGHT)
            .filter(&image, &mut filter_output)
            .expect("Filter config error!");

        if let Err(e) = device.get_error() {
            println!("Error denoising image: {}", e.1);
        }

        let mut image_data: Vec<u8> = vec![];

        for clr in filter_output {
            image_data.push(f32_to_u8(clr));
        }

        println!("Writing to file now!");
        write_to_image(image_data);
    } else {
        let mut image_data: Vec<u8> = vec![];

        for clr in image {
            image_data.push(f32_to_u8(clr));
        }

        println!("Writing to file now!");
        write_to_image(image_data);
    }
    println!("Done!");
}
