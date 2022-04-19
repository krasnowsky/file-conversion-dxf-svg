extern crate dxf;
extern crate svg;
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
                println!("{:?}", arc.center.x);
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

    /*let mut max_value_x = 0.0;
    let mut min_value_x = 0.0;
    let mut max_value_y = 0.0;
    let mut min_value_y = 0.0;*/

    let mut min_y_rotation = 0.0;

    //caluclate maxes and mins here bitch
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
            println!("Drawing arc");
            let mut data = Data::new();
            let difference = entites.y + entites.r - min_y_rotation;
            data = data.move_to((entites.x, entites.y + entites.r - difference));
            //data = data.quadratic_curve_to((entites.x - entites.r, entites.y - difference, 50, 50));
            data = data.quadratic_curve_to((entites.x - entites.r, entites.y + entites.r - difference, entites.x - entites.r, entites.y - difference));
            print!("{:?}", data);
            data_vec.push(data);
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

    /*println!("{:?}, {:?}", max_value_x, min_value_x);
    println!("{:?}, {:?}", max_value_y, min_value_y);
    println!("{:?}, {:?}", length_1, height_1);

    let length = max_value_x - min_value_x;
    let height = max_value_y - min_value_y;

    println!("{:?}, {:?}", length, height);*/

    //adding paths to document
    let mut document = Document::new()
        .set("viewBox", (min_value_x - 0.1 * length, min_value_y - height - 0.1 * height , 1.2 * length, 1.2 * height));
    for p in paths_vec {
        document = document.add(p);
    }

//using previously created document to create actual svg file
svg::save("image.svg", &document).unwrap();
}

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

