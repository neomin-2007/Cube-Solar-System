use std::f32::consts::PI;
use macroquad::prelude::*;

pub struct Orb {
    pub vertex: Vec<[f32; 3]>,
    pub edges: Vec<[usize; 2]>,

    pub size: f32,
    pub translation_dist: f32,

    pub rot_x: f32,
    pub rot_y: f32,
}

impl Orb {

    pub fn new_cube(size: f32) -> Orb {

        let mut vertex = vec![
            [-1.0, -1.0, -1.0],
            [ 1.0, -1.0, -1.0],
            [ 1.0,  1.0, -1.0],
            [-1.0,  1.0, -1.0],
            [-1.0, -1.0,  1.0],
            [ 1.0, -1.0,  1.0],
            [ 1.0,  1.0,  1.0],
            [-1.0,  1.0,  1.0],
        ];
    
        for v in &mut vertex {
            v[0] *= size;
            v[1] *= size;
            v[2] *= size;
        }

        let edges = vec![
            [0, 1], [1, 2], [2, 3], [3, 0],
            [4, 5], [5, 6], [6, 7], [7, 4],
            [0, 4], [1, 5], [2, 6], [3, 7],
        ];
    
        Orb {
            vertex,
            edges,
            size,
            translation_dist: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
        }
    }

    pub fn render(&self, x: f32, y: f32) {
        for edge in &self.edges {

            let vertex1 = rotate(
                self.vertex[edge[0]][0] + self.translation_dist,
                self.vertex[edge[0]][1] + self.translation_dist,
                self.vertex[edge[0]][2] + self.translation_dist,
                self.rot_x,
                self.rot_y,
            );

            let vertex2 = rotate(
                self.vertex[edge[1]][0] + self.translation_dist,
                self.vertex[edge[1]][1] + self.translation_dist,
                self.vertex[edge[1]][2] + self.translation_dist,
                self.rot_x,
                self.rot_y,
            );

            let p1 = project(
                vertex1[0],
                vertex1[1],
                vertex1[2],
                500.0 + self.size,
                100.0,
            );

            let p2 = project(
                vertex2[0],
                vertex2[1],
                vertex2[2],
                500.0 + self.size,
                100.0,
            );


            draw_line(p1[0] + x, p1[1] + y, p2[0] + x, p2[1] + y, 2.0, WHITE);
        }
    }
}

pub fn transform(x: f32, y: f32, z: f32) -> [f32;3] {
    return [x, y, z];
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

pub fn rotate(x: f32, y: f32, z: f32, angleX: f32, angleY: f32) -> [f32;3] {

    let radX = degrees_to_radians(angleX);
    let radY = degrees_to_radians(angleY);

    let cosX = radX.cos();
    let sinX = radX.sin();

    let cosY = radY.cos();
    let sinY = radY.sin();

    let xRotY = x * cosY - z * sinY;
    let zRotY = x * sinY + z * cosY;

    let yRotX = y * cosX - zRotY * sinX;
    let zRotX = y * sinX + zRotY * cosX;

    return [xRotY, yRotX, zRotX];
}

pub fn project(x: f32, y: f32, z: f32, scale: f32, distance: f32) -> [f32;2] {

    let x2d = (x * scale)/(z + distance) + (screen_width() / 2 as f32);
    let y2d = (y * scale)/(z + distance) + (screen_height() / 2 as f32);

    return [x2d, y2d];
}