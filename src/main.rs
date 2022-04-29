extern crate dxf;
extern crate svg;
extern crate libm;
use std::env::{args, Args};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use std::vec::Vec;
use dxf::Drawing;
use dxf::entities::*;

//declaration of public struct
pub struct Object {
    layer_name: String,
    vecx: Vec<f64>,
    vecy: Vec<f64>,
}

pub struct Arc {
    layer_name: String,
    x: f64,
    y: f64,
    r: f64,
    start_angle: f64,
    end_angle: f64,
}

pub struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let mut args: Args = args();
    let path = args.nth(1).unwrap();

    let mut dxf_file = Drawing::new();
    let mut objects_read: Vec<Object> = Vec::new();
    let mut arcs_read: Vec<Arc> = Vec::new();

    println!("\nOpening file from {}", path);

    read_dxf(path, &mut dxf_file).expect("Could not read file");

    ex(&mut dxf_file, &mut objects_read, &mut arcs_read);

    /*for obj in objects_read {
        println!("{:?}", obj.vecx);
    }*/

   svg(&mut objects_read, &mut arcs_read);

}

//dxf file read from the path provided as a parameter
fn read_dxf(_path:String, _dxf_file: &mut Drawing) -> dxf::DxfResult<()> {
    *_dxf_file = Drawing::load_file(_path)?;
    Ok(())
}

fn ex(_dxf_file: &mut Drawing, table: &mut Vec<Object>, table_arc: &mut Vec<Arc>) {
    //loop for inserted objects
    /*for x in _dxf_file.blocks() {
        println!("{:?}", x);
        for e in &x.entities {
            let _layer_name = &e.common.layer;
            let mut _vecx = Vec::new();
            let mut _vecy = Vec::new();

            match e.specific {
                EntityType::Line(ref line) => {
                    println!("Found line");
                    _vecx.push(line.p1.x);
                    _vecx.push(line.p2.x);
                    _vecy.push(line.p1.y);
                    _vecy.push(line.p2.y);
                },
            _ => (),
            }
            let obj = Object {
                layer_name: _layer_name.to_string(),
                vecx: _vecx,
                vecy: _vecy,
             };
             table.push(obj);
        }
    }*/

    //loop for "normal" objects
    for e in _dxf_file.entities() {
        let _layer_name = &e.common.layer;
        let mut _vecx = Vec::new();
        let mut _vecy = Vec::new();
        let mut _arc = Arc {
            layer_name: "".to_string(),
            x: 0.0,
            y: 0.0,
            r: 0.0,
            start_angle: 0.0,
            end_angle: 0.0,
        };

        match e.specific {
            EntityType::LwPolyline(ref lwpolyline) => {
                println!("Found polyline");
                for polyline in &lwpolyline.vertices {
                    _vecx.push(polyline.x);
                    _vecy.push(polyline.y);
                }
            },
            EntityType::Line(ref line) => {
                println!("Found line");
                _vecx.push(line.p1.x);
                _vecx.push(line.p2.x);
                _vecy.push(line.p1.y);
                _vecy.push(line.p2.y);
            },
            EntityType::Arc(ref arc) => {
                println!("Found arc");
                //println!("{:?}", arc.center.x);
                _arc.layer_name = _layer_name.to_string();
                _arc.x = arc.center.x;
                _arc.y = arc.center.y;
                _arc.r = arc.radius;
                _arc.start_angle = arc.start_angle;
                _arc.end_angle = arc.end_angle;
                table_arc.push(_arc);
            },
            _ => (),
        }

        let obj = Object {
            layer_name: _layer_name.to_string(),
            vecx: _vecx,
            vecy: _vecy,
        };

        table.push(obj);
    }
}

fn svg(table: &mut Vec<Object>, table_arcs: &mut Vec<Arc>) {
    let mut data_vec: Vec<Data> = Vec::new();
    let mut paths_vec: Vec<Path> = Vec::new();

    let mut max_value_x = table[0].vecx[0];
    let mut min_value_x = table[0].vecx[0];
    let mut max_value_y = table[0].vecy[0];
    let mut min_value_y = table[0].vecy[0];

    let mut min_y_rotation = 0.0;

    for entities in table.iter() {
        let range = entities.vecx.len();
        if range != 0 {
            min_y_rotation = entities.vecy[0];
            for y in entities.vecy.iter() {
                if y < &mut min_y_rotation { min_y_rotation = *y; }
            }
        }
    }

    for entites in table.iter() {
        let range = entites.vecx.len();
        if range != 0 {
            if entites.vecx[0] < min_value_x {min_value_x = entites.vecx[0];}
            if entites.vecy[0] < min_value_y {min_value_y = entites.vecy[0];}

            //calculating minimum and maximum values of x for the sake of plain creation
            for x in entites.vecx.iter() {
                if x < &mut min_value_x { min_value_x = *x; }
                if x > &mut max_value_x { max_value_x = *x; }
            }

            //calculating minimum and maximum values of y for the sake of plain creation
            for y in entites.vecy.iter() {
                if y < &mut min_value_y { min_value_y = *y; }
                if y > &mut max_value_y { max_value_y = *y; }
            }

            //creating data basing on points
            let mut data = Data::new();
            let mut difference = entites.vecy[0] - min_y_rotation;
            data = data.move_to((entites.vecx[0], entites.vecy[0] - 2.0 * difference));
            for n in 1..range {
                difference = entites.vecy[n] - min_y_rotation;
                data = data.line_to((entites.vecx[n], entites.vecy[n] - 2.0 * difference));
            }
            data = data.close();

            data_vec.push(data);
        }
    }

    let table_arcs_length = table_arcs.len();

    for entites in table_arcs.iter() {
        if table_arcs_length != 0 {
            let mut data = Data::new();
            let center_x = entites.x;
            let center_y = entites.y;
            let radius = entites.r;
            let start_angle = entites.start_angle;
            let mut end_angle = entites.end_angle;
            let mut current_angle = entites.start_angle;
            let mut points_of_arc: Vec<Point> = Vec::new();
            let mut angle_between = end_angle - start_angle;

            if end_angle == 0.0 {
                end_angle = 360.0;
                angle_between = end_angle - start_angle;
            }
            else if end_angle < 90.0 && end_angle > 0.0 && start_angle > 270.0 && start_angle < 359.9 {
                angle_between = 360.0 - start_angle + end_angle;
            }

            println!("{:?}, {:?}, {:?}", start_angle, end_angle, angle_between);

            let angle_to_jump_by = angle_between / 5.0;

            println!("{:?}", angle_to_jump_by);

            if start_angle < end_angle {
                while current_angle <= end_angle {
                    polar_to_cartesian(center_x, center_y, radius, current_angle, &mut points_of_arc);
                    println!("Before increment: {:?}", current_angle);
                    current_angle += angle_to_jump_by;
                    println!("After increment: {:?}", current_angle);
                }
            }
            else {
                while current_angle != end_angle {
                    println!("{:?}, {:?}", current_angle, end_angle);
                    polar_to_cartesian(center_x, center_y, radius, current_angle, &mut points_of_arc);
                    if current_angle + angle_to_jump_by > 360.0 {
                        current_angle = current_angle + angle_between - 360.0;
                    }
                    else {
                        current_angle += angle_to_jump_by;
                    }
                }
            }

            let range = points_of_arc.len();
            if range != 0 {
                let mut difference = points_of_arc[0].y - min_y_rotation;
                data = data.move_to((points_of_arc[0].x, points_of_arc[0].y - 2.0 * difference));
                for n in 1..range {
                    difference = points_of_arc[n].y - min_y_rotation;
                    data = data.line_to((points_of_arc[n].x, points_of_arc[n].y - 2.0 * difference));
                }

                data_vec.push(data);
            }
        }
    }

    //adding data to paths
    for v in data_vec {
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.5)
            .set("d", v);
        paths_vec.push(path);
    }

    let mut length = 0.0;
    let mut height = 0.0;

    calculate_dimensions(max_value_x, min_value_x, max_value_y, min_value_y, &mut length, &mut height);

    //adding paths to document
    let mut document = Document::new()
        //.set("viewBox", (min_value_x - 0.1 * length, min_value_y - height - 0.1 * height , 1.2 * length, 1.2 * height));
        .set("viewBox", (min_value_x - 0.1 * length, min_value_y - 0.1 * height , 1.2 * length, 1.2 * height));
    for p in paths_vec {
        document = document.add(p);
    }

//using previously created document to create actual svg file
svg::save("image.svg", &document).unwrap();
}

//transform polar coordinates to cartesian coordinates
fn polar_to_cartesian(_center_x: f64, _center_y: f64, _radius: f64, _angle: f64, _table_of_points: &mut Vec<Point>) {
    let angle_in_radians = _angle * std::f64::consts::PI / 180.0;
    let _x = _center_x + _radius * libm::cos(angle_in_radians);
    let _y = _center_y + _radius * libm::sin(angle_in_radians);
    let point = Point {
        x: _x,
        y: _y,
    };

    _table_of_points.push(point);
}

//calculate dimensions of the viewBox
fn calculate_dimensions(_max_x: f64, _min_x: f64, _max_y: f64, _min_y: f64, _length: &mut f64, _height: &mut f64) {
    //calculating length
    if _max_x > 0.0 && _min_x > 0.0 {
        *_length = _max_x - _min_x;
    }
    else if _max_x > 0.0 && _min_x < 0.0 {
        *_length = _max_x + (-1.0 * _min_x);
    }
    else {
        *_length = (-1.0 * _max_x) + (-1.0 * _min_x)
    }

    //calculating height
    if _max_y > 0.0 && _min_y > 0.0 {
        *_height = _max_y - _min_y;
    }
    else if _max_y > 0.0 && _min_x < 0.0 {
        *_height = _max_y + (-1.0 * _min_y);
    }
    else {
        *_height = (-1.0 * _max_y) + (-1.0 * _min_y)
    }
}

