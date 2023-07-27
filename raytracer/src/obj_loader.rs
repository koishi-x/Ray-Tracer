use crate::*;
use tobj::{self, load_mtl, LoadOptions};

/*
pub fn load_steve_obj(path: &str, scale: f64) -> HittableList {
    let load_option = tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ignore_lines: true,
        ignore_points: true,
        #[cfg(feature = "merging")]
        merge_identical_points: false,
        #[cfg(feature = "reordering")]
        reorder_data: false,
    };
    let cornell_box = tobj::load_obj(path, &load_option);
    assert!(cornell_box.is_ok());
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");
    let materials = materials.expect("Failed to load MTL file");

    let mut objects = HittableList::new();
    for model in models {
        let mesh = model.mesh;
        let material_id = mesh.material_id.unwrap();
        let mut boxes = HittableList::new();

        let mat = Lambertian::new_texture(ImageTexture::new(
            materials[material_id].diffuse_texture.as_ref().unwrap(),
        ));

        assert!(!mesh.texcoords.is_empty());
        assert!(mesh.positions.len() % 3 == 0);
        assert!(mesh.indices.len() % 4 == 0);
        for i in 0..mesh.indices.len() / 4 {
            //println!("{i}\n");
            let i1 = mesh.indices[i * 4] as usize;
            let i2 = mesh.indices[i * 4 + 1] as usize;
            let i3 = mesh.indices[i * 4 + 2] as usize;
            let i4 = mesh.indices[i * 4 + 3] as usize;
            let pos1 = Vec3 {
                x: mesh.positions[i1 * 3] as f64 * scale,
                y: mesh.positions[i1 * 3 + 1] as f64 * scale,
                z: mesh.positions[i1 * 3 + 2] as f64 * scale,
            };
            let texcoord1 = (
                mesh.texcoords[i1 * 2] as f64,
                mesh.texcoords[i1 * 2 + 1] as f64,
            );

            let pos2 = Vec3 {
                x: mesh.positions[i2 * 3] as f64 * scale,
                y: mesh.positions[i2 * 3 + 1] as f64 * scale,
                z: mesh.positions[i2 * 3 + 2] as f64 * scale,
            };
            let texcoord2 = (
                mesh.texcoords[i2 * 2] as f64,
                mesh.texcoords[i2 * 2 + 1] as f64,
            );

            let pos3 = Vec3 {
                x: mesh.positions[i3 * 3] as f64 * scale,
                y: mesh.positions[i3 * 3 + 1] as f64 * scale,
                z: mesh.positions[i3 * 3 + 2] as f64 * scale,
            };
            let texcoord3 = (
                mesh.texcoords[i3 * 2] as f64,
                mesh.texcoords[i3 * 2 + 1] as f64,
            );

            let pos4 = Vec3 {
                x: mesh.positions[i4 * 3] as f64 * scale,
                y: mesh.positions[i4 * 3 + 1] as f64 * scale,
                z: mesh.positions[i4 * 3 + 2] as f64 * scale,
            };
            let texcoord4 = (
                mesh.texcoords[i4 * 2] as f64,
                mesh.texcoords[i4 * 2 + 1] as f64,
            );

            let mut cur1 = Triangle::new(
                pos1,
                pos2,
                pos3,
                mat.clone(),
                texcoord1,
                texcoord2,
                texcoord3,
            );
            let mut cur2 = Triangle::new(
                pos2,
                pos4,
                pos3,
                mat.clone(),
                texcoord2,
                texcoord4,
                texcoord3,
            );
            boxes.add(Arc::new(cur1));
            boxes.add(Arc::new(cur2));
        }
        println!("{}", boxes.objects.len());
        objects.add(Arc::new(BvhNode::new_hittablelist(boxes, 0.0, 1.0)));
    }
    objects
}
*/

pub fn load_obj_without_mat(path: &str, scale: f64) -> HittableList {
    let cornell_box = tobj::load_obj(
        path,
        &LoadOptions {
            triangulate: true,
            single_index: false,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    assert!(cornell_box.is_ok());
    let (models, _) = cornell_box.expect("Failed to load OBJ file");
    //let materials = materials.expect("Failed to load MTL file");

    let mut objects = HittableList::new();

    for model in models {
        let mesh = model.mesh;
        //let material_id = mesh.material_id.unwrap();
        let mut boxes = HittableList::new();

        let mat = Lambertian::new(Color {
            x: 0.2,
            y: 0.7,
            z: 0.2,
        });
        assert!(!mesh.texcoords.is_empty());
        assert!(mesh.positions.len() % 3 == 0);
        assert!(mesh.indices.len() % 3 == 0);
        for i in 0..mesh.indices.len() / 3 {
            //println!("{i}\n");
            let i1 = mesh.indices[i * 3] as usize;
            let i2 = mesh.indices[i * 3 + 1] as usize;
            let i3 = mesh.indices[i * 3 + 2] as usize;
            let ti1 = mesh.texcoord_indices[i * 3] as usize;
            let ti2 = mesh.texcoord_indices[i * 3 + 1] as usize;
            let ti3 = mesh.texcoord_indices[i * 3 + 2] as usize;

            let pos1 = Vec3 {
                x: mesh.positions[i1 * 3] as f64 * scale,
                y: mesh.positions[i1 * 3 + 1] as f64 * scale,
                z: mesh.positions[i1 * 3 + 2] as f64 * scale,
            };
            let texcoord1 = (
                mesh.texcoords[ti1 * 2] as f64,
                mesh.texcoords[ti1 * 2 + 1] as f64,
            );

            let pos2 = Vec3 {
                x: mesh.positions[i2 * 3] as f64 * scale,
                y: mesh.positions[i2 * 3 + 1] as f64 * scale,
                z: mesh.positions[i2 * 3 + 2] as f64 * scale,
            };
            let texcoord2 = (
                mesh.texcoords[ti2 * 2] as f64,
                mesh.texcoords[ti2 * 2 + 1] as f64,
            );

            let pos3 = Vec3 {
                x: mesh.positions[i3 * 3] as f64 * scale,
                y: mesh.positions[i3 * 3 + 1] as f64 * scale,
                z: mesh.positions[i3 * 3 + 2] as f64 * scale,
            };
            let texcoord3 = (
                mesh.texcoords[ti3 * 2] as f64,
                mesh.texcoords[ti3 * 2 + 1] as f64,
            );

            let cur = Triangle::new(
                pos1,
                pos2,
                pos3,
                mat.clone(),
                texcoord1,
                texcoord2,
                texcoord3,
            );

            // let na = Vec3 {
            //     x: mesh.normals[i1 * 3] as f64,
            //     y: mesh.normals[i1 * 3 + 1] as f64,
            //     z: mesh.normals[i1 * 3 + 2] as f64,
            // };
            // let nb = Vec3 {
            //     x: mesh.normals[i2 * 3] as f64,
            //     y: mesh.normals[i2 * 3 + 1] as f64,
            //     z: mesh.normals[i2 * 3 + 2] as f64,
            // };
            // let nc = Vec3 {
            //     x: mesh.normals[i3 * 3] as f64,
            //     y: mesh.normals[i3 * 3 + 1] as f64,
            //     z: mesh.normals[i3 * 3 + 2] as f64,
            // };

            // cur.set_normal(na, nb, nc);

            boxes.add(Arc::new(cur));
        }

        //objects.add(Arc::new(BvhNode::new_hittablelist(boxes, 0.0, 1.0)));
        objects.add(Arc::new(BvhNode::new_hittablelist(boxes, 0.0, 1.0)));
        //objects.add(Arc::new(boxes));
    }
    objects
}

pub fn load_obj(path: &str, scale: f64) -> HittableList {
    let cornell_box = tobj::load_obj(
        path,
        &LoadOptions {
            triangulate: true,
            single_index: false,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    assert!(cornell_box.is_ok());
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");
    let materials = materials.expect("Failed to load MTL file");

    let mut objects = HittableList::new();
    let mut cnt = 0;
    let leng = models.len();
    for model in models {
        cnt += 1;
        println!("{}/{}\n", cnt, leng);
        let mesh = model.mesh;
        let material_id = mesh.material_id.unwrap();
        let mut boxes = HittableList::new();

        let mat_ = Lambertian::new_texture(ImageTexture::new(
            materials[material_id].diffuse_texture.as_ref().unwrap(),
        ));

        let mat = Lambertian::new(Color {
            x: 0.2,
            y: 0.7,
            z: 0.2,
        });
        assert!(!mesh.texcoords.is_empty());
        assert!(mesh.positions.len() % 3 == 0);
        assert!(mesh.indices.len() % 3 == 0);
        for i in 0..mesh.indices.len() / 3 {
            //println!("{i}\n");
            let i1 = mesh.indices[i * 3] as usize;
            let i2 = mesh.indices[i * 3 + 1] as usize;
            let i3 = mesh.indices[i * 3 + 2] as usize;
            let ti1 = mesh.texcoord_indices[i * 3] as usize;
            let ti2 = mesh.texcoord_indices[i * 3 + 1] as usize;
            let ti3 = mesh.texcoord_indices[i * 3 + 2] as usize;

            let pos1 = Vec3 {
                x: mesh.positions[i1 * 3] as f64 * scale,
                y: mesh.positions[i1 * 3 + 1] as f64 * scale,
                z: mesh.positions[i1 * 3 + 2] as f64 * scale,
            };
            let texcoord1 = (
                mesh.texcoords[ti1 * 2] as f64,
                mesh.texcoords[ti1 * 2 + 1] as f64,
            );

            let pos2 = Vec3 {
                x: mesh.positions[i2 * 3] as f64 * scale,
                y: mesh.positions[i2 * 3 + 1] as f64 * scale,
                z: mesh.positions[i2 * 3 + 2] as f64 * scale,
            };
            let texcoord2 = (
                mesh.texcoords[ti2 * 2] as f64,
                mesh.texcoords[ti2 * 2 + 1] as f64,
            );

            let pos3 = Vec3 {
                x: mesh.positions[i3 * 3] as f64 * scale,
                y: mesh.positions[i3 * 3 + 1] as f64 * scale,
                z: mesh.positions[i3 * 3 + 2] as f64 * scale,
            };
            let texcoord3 = (
                mesh.texcoords[ti3 * 2] as f64,
                mesh.texcoords[ti3 * 2 + 1] as f64,
            );

            let cur = Triangle::new(
                pos1,
                pos2,
                pos3,
                mat.clone(),
                texcoord1,
                texcoord2,
                texcoord3,
            );

            // let na = Vec3 {
            //     x: mesh.normals[i1 * 3] as f64,
            //     y: mesh.normals[i1 * 3 + 1] as f64,
            //     z: mesh.normals[i1 * 3 + 2] as f64,
            // };
            // let nb = Vec3 {
            //     x: mesh.normals[i2 * 3] as f64,
            //     y: mesh.normals[i2 * 3 + 1] as f64,
            //     z: mesh.normals[i2 * 3 + 2] as f64,
            // };
            // let nc = Vec3 {
            //     x: mesh.normals[i3 * 3] as f64,
            //     y: mesh.normals[i3 * 3 + 1] as f64,
            //     z: mesh.normals[i3 * 3 + 2] as f64,
            // };

            // cur.set_normal(na, nb, nc);

            boxes.add(Arc::new(cur));
        }

        //objects.add(Arc::new(BvhNode::new_hittablelist(boxes, 0.0, 1.0)));
        objects.add(Arc::new(BvhNode::new_hittablelist_with_mat(
            boxes, 0.0, 1.0, mat_,
        )));
        //objects.add(Arc::new(boxes));
    }
    objects
}
