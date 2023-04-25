use cucumber_rust::{Steps, t};

steps!(crate::MyWorld => {
    given regex r#"^I have a canvas with an image$"# (crate::MyWorld, _matches, _step) {
        let width = 100.0;
        let height = 100.0;
        let size = egui::Vec2::new(width, height);

        world.image_states = vec![pixel_arranger::app::ImageState {
            id: 0,
            image_path: "".to_string(),
            position: egui::Pos2::new(0.0, 0.0),
            original_size: size,
            current_size: size,
        }];
    }

    when regex r#"^I scroll (up|down) with the mouse wheel over the image$"# (crate::MyWorld, matches, _step) {
        let direction = &matches[1];
        let factor = match direction.as_str() {
            "up" => 1.1,
            "down" => 0.9,
            _ => panic!("Invalid direction"),
        };

        let image_state = &mut world.image_states[0];
        image_state.current_size *= factor;
    }

    then regex r#"^the image should be (zoomed in|zoomed out)$"# (crate::MyWorld, matches, _step) {
        let zoom_state = &matches[1];
        let image_state = &world.image_states[0];

        match zoom_state.as_str() {
            "zoomed in" => assert!(image_state.current_size > image_state.original_size),
            "zoomed out" => assert!(image_state.current_size < image_state.original_size),
            _ => panic!("Invalid zoom state"),
        }
    }
});

