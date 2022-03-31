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

fn main() {
    let mut args: Args = args();
    let path = args.nth(1).unwrap();

    let mut dxf_file = Drawing::new();
    let mut objects_read: Vec<Object> = Vec::new();

    println!("\nOpening file from {}", path);

    read_dxf(path, &mut dxf_file).expect("Could not read file");

    ex(&mut dxf_file, &mut objects_read);

    /*for obj in objects_read {
        println!("{:?}", obj.vecx);
    }*/

    svg(&mut objects_read);

}

//dxf file read from the path provided as a parameter
fn read_dxf(_path:String, _dxf_file: &mut Drawing) -> dxf::DxfResult<()> {
    *_dxf_file = Drawing::load_file(_path)?;
    Ok(())
}

fn ex(_dxf_file: &mut Drawing, table: &mut Vec<Object>) {
    //loop for inserted objects
    for x in _dxf_file.blocks() {
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
    }

    //loop for "normal" objects
    for e in _dxf_file.entities() {
        let _layer_name = &e.common.layer;
        let mut _vecx = Vec::new();
        let mut _vecy = Vec::new();

        match e.specific {
            EntityType::LwPolyline(ref lwpolyline) => {
                println!("Found polyline");
                for polyline in &lwpolyline.vertices {
                    _vecx.push(polyline.x);
                    _vecy.push(polyline.y);
                }
            },
            EntityType::Line(ref line) => {
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
}

fn svg(table: &mut Vec<Object>) {
    let mut data_vec: Vec<Data> = Vec::new();
    let mut paths_vec: Vec<Path> = Vec::new();
    let mut max_value_x = 0.0;
    let mut max_value_y = 0.0;
    let mut min_value_x = 0.0;
    let mut min_value_y = 0.0;

    for entites in table.iter(){
        let range = entites.vecx.len();
        if range != 0 {
            min_value_x = entites.vecx[0];
            min_value_y = entites.vecy[0];

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

            data = data.move_to((entites.vecx[0], entites.vecy[0]));
            for n in 1..range {
                data = data.line_to((entites.vecx[n], entites.vecy[n]));
            }
            data = data.close();

            data_vec.push(data);
        }
    }

    //adding data to paths
    for v in data_vec {
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", v);
        paths_vec.push(path);
    }

    //adding paths to document
    let mut document = Document::new()
        .set("viewBox", (min_value_x - 2.0, min_value_y - 2.0, 100, 100));
    for p in paths_vec {
        document = document.add(p);
    }

//using previously created document to create actual svg file
svg::save("image.svg", &document).unwrap();
}


