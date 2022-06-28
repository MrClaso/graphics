use std::{f64::consts::PI, time, thread};
use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Color};
use nalgebra::{Matrix3, Vector3};


const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[allow(non_upper_case_globals)]
const g: f64 = 9.82;

#[derive(Copy, Clone, Debug)]
struct Face {
    color: Color,
    pts: [usize;4]
}

fn main() {
//    let mut time_measure1 = [0;20];
//    let mut time_measure2 = [0;20];
    let time_step = 0.002;  // Time step in seconds
    let delta_t = time::Duration::from_millis(2);
    let side = 1.0;
    let h:f64 = 2.0*side;
    let min_y = 70.0; // Floor on screen
    let max_y = 800.0;  
    let factor = (max_y - min_y)/5.0/side;
    let a = side/2.0*factor;
    let scale = (HEIGHT as f64 - min_y  - (HEIGHT as f64 - max_y))/h;
    let mut v = 0.0;
    let mut v0 = 0.0;
    let mut t = 0.0;
    let mut lowest = 0.0;
    let mut t0 = 0.0;

    let mut window = Window::new("Raqote", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);


    let mut orientation:Vec<f64> = vec![2.0, 1.2, PI/4.0];
    let mut rg:Vector3<f64> = Vector3::new(500.0, HEIGHT as f64 - max_y, 0.0);

    let mut ca:f64 = orientation[2].cos();
    let mut sa:f64 = orientation[2].sin();
    let mut cb:f64 = orientation[1].cos();
    let mut sb:f64 = orientation[1].sin();
    let mut cg:f64 = orientation[0].cos();
    let mut sg:f64 = orientation[0].sin();


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

    let mut now = time::Instant::now();
    let mut h0 = h;

while t < 0.620 {
//    loop{

        for i in 0..8{
            r = rg + m*points[i];
            rs[i] = r;
            if r[1] > lowest {lowest = r[1]}
        }

        println!("t = {} ; v = {} ; lowest = {} ; yg = {}", t, v, lowest, rg[1]);
        // Find out if collision occured
        if lowest > (HEIGHT as f64 - min_y) { // The dice has hit the floor

            t0 = (t*t -2.0*(lowest - (HEIGHT as f64 - min_y))/scale/g).sqrt();
            v0 = g*t0;
            h0 = (rg[1] + (lowest - (HEIGHT as f64 - min_y)))/scale + 0.0001;
            println!("v0 = {} ; h0 = {} ; t0 = {}", v0, h0, t0);
            lowest = 0.0;
            t0 = t;
        }

    // Find "closest" corner
        let mut index: usize = 0;

        let mut closest: f64 = rs[index][2];
        for i in 1..8{
            if rs[i][2] < closest { 
                closest = rs[i][2];
                index = i;
            }
        }
    // Find closest faces. ie the faces that holds the corner with index i
        let mut corners = Vec::new();

        for i in 0.. 6 {
            if faces[i].pts.iter().any(|&p| p==index) {
                corners.push(i);
            } 
        }

//time_measure1[l] = now.elapsed().as_nanos();
//now = time::Instant::now();

        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        for i in 0..3{
            let mut pb = PathBuilder::new();

            pb.move_to((rs[faces[corners[i]].pts[0]][0]) as f32, (rs[faces[corners[i]].pts[0]][1]) as f32);
            for j in 1..4 {
                pb.line_to((rs[faces[corners[i]].pts[j]][0]) as f32, (rs[faces[corners[i]].pts[j]][1]) as f32);

            }               
            let path = pb.finish();
            dt.fill(&path, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, faces[corners[i]].color.r(), faces[corners[i]].color.g(), faces[corners[i]].color.b())), &DrawOptions::new());
        }
        window.update_with_buffer(dt.get_data(), WIDTH, HEIGHT).unwrap();


//        time_measure2[l] = now.elapsed().as_micros();
        while now.elapsed() < delta_t {};

        now = time::Instant::now();

        let rg_real = h0 - g*(t - t0)*(t - t0)/2.0 + v0*(t-t0);

        rg[1] = HEIGHT as f64 - min_y - scale*rg_real;
//        orientation[0] += 0.002;
//        orientation[1] += 0.004;
//        orientation[2] += 0.008;


        ca = orientation[2].cos();
        sa = orientation[2].sin();
        cb = orientation[1].cos();
        sb = orientation[1].sin();
        cg = orientation[0].cos();
        sg = orientation[0].sin();


        m = Matrix3::new(
            ca*cb,  ca*sb*sg - sa*cg,   ca*sb*cg + sa*sg, 
            sa*cb,  sa*sb*sg + ca*cg,   sa*sb*cg - ca*sg, 
            -sb,    cb*sg,              cb*cg);

        t += time_step as f64;
        v = v0 - g*(t - t0);
    }

//    println!(" Beräkningar (ns) : {:?}", time_measure1);
//    println!(" Ritande (µs) : {:?}", time_measure2);

}
