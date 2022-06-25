use std::{f64::consts::PI, time::Duration};
use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Color};
use nalgebra::{Matrix3, Vector3};


const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[derive(Copy, Clone, Debug)]
struct Face {
    color: Color,
    pts: [usize;4]
}

fn main() {

    let ten_millis = Duration::from_millis(20);

    let mut window = Window::new("Raqote", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

    let a = 100.0;

    let mut dr:Vec<f64> = vec![2.0, 1.2, PI/4.0];
    let mut rg:Vector3<f64> = Vector3::new(500.0, 500.0, 0.0);

    let mut ca:f64 = dr[2].cos();
    let mut sa:f64 = dr[2].sin();
    let mut cb:f64 = dr[1].cos();
    let mut sb:f64 = dr[1].sin();
    let mut cg:f64 = dr[0].cos();
    let mut sg:f64 = dr[0].sin();


    let mut m:Matrix3<f64> = Matrix3::new(
        ca*cb, ca*sb*sg - sa*cg, ca*sb*cg + sa*sg, 
        sa*cb, sa*sb*sg + ca*cg, sa*sb*cg - ca*sg, 
        -sb, cb*sg, cb*cg);

    let mut p: Vector3<f64> = Vector3::new(1.0, 0.0, 0.0);

    let mut points = Vec::new();

    const BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

    for i in 0u8..8 {
        p[0] = (-1.0 + 2.0*((i & BIT_MASK[0])) as f64) * a;
        p[1] = (-1.0 + 2.0*((i & BIT_MASK[1])/2) as f64) * a;
        p[2] = (-1.0 + 2.0*((i & BIT_MASK[2])/4) as f64) * a;
        points.push(p);
    }

    let sfaceit = Face {
        color : Color::new(255, 255, 0, 0),
        pts: [0, 1, 3, 2]
    };

    let mut faces = [sfaceit;6];

    faces[1].color = Color::new(255, 0, 255, 0);
    faces[1].pts = [0, 4, 6, 2];
    faces[2].color = Color::new(255, 0, 0, 255);
    faces[2].pts = [0, 1, 5, 4];
    faces[3].color = Color::new(255, 255, 0, 255);
    faces[3].pts = [2, 3, 7, 6];
    faces[4].color = Color::new(255, 0, 255, 255);
    faces[4].pts = [4, 5, 7, 6];
    faces[5].color = Color::new(255, 255, 255, 0);
    faces[5].pts = [3, 1, 5, 7];

    let mut r = Vector3::new(0.0, 0.0, 0.0);
    let mut rs: Vec<Vector3<f64>> = vec![r;8];

/*     
    for i in 0..8{
        r = rg + m*points[i];
        pts = rg + points[i];
        println!("x = {}  , x = {}", pts[0], r[0]);
        println!("y = {}  , y = {}", pts[1], r[1]);
        println!("z = {}  , z = {}", pts[2], r[2]);    
        println!("Hello, world!");    
    }
*/


    loop {
        // Find out if collision occured
        
    // Find "closest" corner
        let mut index: usize = 0;
        for i in 0..8{
            r = rg + m*points[i];
            rs[i] = r;
        }

        let mut closest: f64 = rs[index][2];
        for i in 1..8{
            if rs[i][2] < closest { 
                closest = rs[i][2];
                index = i;
            }
        }
    // Find closest faces. ie the faces that holds corners with index i
        let mut corners = Vec::new();

        for i in 0.. 6 {
            if faces[i].pts.iter().any(|&p| p==index) {
                corners.push(i);
            } 
        }
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));

        for i in 0..3{
            let mut pb = PathBuilder::new();

            pb.move_to((rs[faces[corners[i]].pts[0]][0]) as f32, (rs[faces[corners[i]].pts[0]][1]) as f32);
            for j in 1..4 {
                pb.line_to((rs[faces[corners[i]].pts[j]][0]) as f32, (rs[faces[corners[i]].pts[j]][1]) as f32);

            }               
            let path = pb.finish();
            dt.fill(&path, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, faces[corners[i]].color.r(), faces[corners[i]].color.g(), faces[corners[i]].color.b())), &DrawOptions::new());
            window.update_with_buffer(dt.get_data(), WIDTH, HEIGHT).unwrap();
        }
    /*       

            let mut pb = PathBuilder::new();

            pb.move_to((points[faces[4].pts[0]].x) as f32 + rg.x, (points[faces[4].pts[0]].y) as f32 + rg.y);
            pb.line_to((points[faces[4].pts[1]].x) as f32 + rg.x, (points[faces[4].pts[1]].y) as f32 + rg.y);
            pb.line_to((points[faces[4].pts[2]].x) as f32 + rg.x, (points[faces[4].pts[2]].y) as f32 + rg.y);
            pb.line_to((points[faces[4].pts[3]].x) as f32 + rg.x, (points[faces[4].pts[3]].y) as f32 + rg.y);
            let path = pb.finish();

            dt.fill(&path, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, faces[4].color.r(), faces[4].color.g(), faces[4].color.b())), &DrawOptions::new());
            window.update_with_buffer(dt.get_data(), WIDTH, HEIGHT).unwrap();
    */
    std::thread::sleep(ten_millis);

        dr[0] += 0.02;
        dr[1] += 0.04;
        dr[2] += 0.08;


    ca = dr[2].cos();
    sa = dr[2].sin();
    cb = dr[1].cos();
    sb = dr[1].sin();
    cg = dr[0].cos();
    sg = dr[0].sin();


    m = Matrix3::new(
        ca*cb, ca*sb*sg - sa*cg, ca*sb*cg + sa*sg, 
        sa*cb, sa*sb*sg + ca*cg, sa*sb*cg - ca*sg, 
        -sb, cb*sg, cb*cg);
    }


}

