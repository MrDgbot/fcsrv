use fcsrv::{model::ModelType, BootArgs};
use ort::AllocatorType;

fn main() {
    fcsrv::model::init_predictor(&BootArgs {
        debug: false,
        bind: "0.0.0.0:8000".parse().unwrap(),
        tls_cert: None,
        tls_key: None,
        api_key: None,
        multi_image_limit: 1,
        update_check: false,
        model_dir: None,
        num_threads: 4,
        allocator: AllocatorType::Arena,
    })
    .unwrap();

    let predictor = fcsrv::model::get_predictor(ModelType::M3dRollballAnimals).unwrap();

    // Read image file images/3d_rollball_animals/0bcc74b7-487c-4db4-8d48-7d2d2091ae23_3.jpg
    let image_file =
        std::fs::read("images/3d_rollball_animals/0bcc74b7-487c-4db4-8d48-7d2d2091ae23_3.jpg")
            .unwrap();
    let guess = predictor
        .predict(image::load_from_memory(&image_file).unwrap())
        .unwrap();
    assert_eq!(guess, 3);

    // Read image file images/3d_rollball_animals/1a03913c-61e1-4c95-a9c6-e45bbc419ee4-0_3.jpg
    let image_file =
        std::fs::read("images/3d_rollball_animals/1a03913c-61e1-4c95-a9c6-e45bbc419ee4-0_3.jpg")
            .unwrap();
    let guess = predictor
        .predict(image::load_from_memory(&image_file).unwrap())
        .unwrap();
    assert_eq!(guess, 3);

    // Read image file images/3d_rollball_animals/1a03913c-61e1-4c95-a9c6-e45bbc419ee4-1_3.jpg
    let image_file =
        std::fs::read("images/3d_rollball_animals/1a03913c-61e1-4c95-a9c6-e45bbc419ee4-1_3.jpg")
            .unwrap();
    let guess = predictor
        .predict(image::load_from_memory(&image_file).unwrap())
        .unwrap();
    assert_eq!(guess, 3);

    // Read image file images/3d_rollball_animals/1a03913c-61e1-4c95-a9c6-e45bbc419ee4-2_2.jpg
    let image_file =
        std::fs::read("images/3d_rollball_animals/1a03913c-61e1-4c95-a9c6-e45bbc419ee4-2_2.jpg")
            .unwrap();
    let guess = predictor
        .predict(image::load_from_memory(&image_file).unwrap())
        .unwrap();
    assert_eq!(guess, 2);
}
