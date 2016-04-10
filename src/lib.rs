extern crate assimp;

#[test]
fn it_works() {
    let mut importer = assimp::Importer::new();
    let scene = importer.read_file("Barrel1.x").unwrap();
    panic!("{:?}", scene.num_meshes());
}
