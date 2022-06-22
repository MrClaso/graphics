
use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point as OtherPoint, Transform, StrokeStyle, Color};



const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32
}

impl Point {
    fn norm(&self) -> Point{
        let mut pt: Point = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
        let length: f32 =  (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        pt.x = self.x/length;
        pt.y = self.y/length;
        pt.z = self.z/length;
        return pt
    }
}
#[derive(Copy, Clone, Debug)]
struct Face {
    color: Color,
    pts: [usize;4]
}

fn main() {


    let mut window = Window::new("Raqote", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

    let a: f32 = 100.0; 

    let pt = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

    let dg: Point = Point { x: 1.0, y: 0.0, z: 0.0 };
    let dr: Point = Point { x: 1.0, y: 0.0, z: 0.0 };
    let rg: Point = Point { x: 500.0, y: 700.0, z: 0.0 };
    let mut points: [Point;8] = [pt;8];

    const BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];


    for i in 0..8 {
        points[i].x = (-1.0 + 2.0*(((i as u8) & BIT_MASK[0]) as f32)) * a;
        points[i].y = (-1.0 + 2.0*(((i as u8) & BIT_MASK[1]) as f32/2.0)) * a;
        points[i].z = (-1.0 + 2.0*(((i as u8) & BIT_MASK[2]) as f32/4.0)) * a;
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


// For printing stuff
println!("Punkt 3 normaliserad {}", points[3].norm().x);
/*
    for i in 0..8{
        println!("Punkt {}", i);
        println!("x = {0}, y = {1}, z = {2}", (points[i].x) as f32 + 100.0, (points[i].y) as f32 + 100.0, (points[i].z) as f32 + 100.0,);
       println!("x = {0}, y = {1}", points[faces[0].pts[i]].x, points[faces[0].pts[i]].y);

        println!("{}",(faces[0].pts[i]));

        println!("{}, {}, {}",points[i].x, points[i].y, points[i].z);
    }
*/

    loop {
// Find "closest" corner
        let mut index: usize = 0;
        let mut closest: f32 = points[index].z;
        for i in 1..8{
            if points[i].z < closest { 
                closest = points[i].z;
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

            pb.move_to((points[faces[corners[i]].pts[0]].x) as f32 + rg.x, (points[faces[corners[i]].pts[0]].y) as f32 + rg.y);
            for j in 1..4 {
                pb.line_to((points[faces[corners[i]].pts[j]].x) as f32 + rg.x, (points[faces[corners[i]].pts[j]].y) as f32 + rg.y);

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
    }


}
