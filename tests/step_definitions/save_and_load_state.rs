use cucumber_rust::{Steps, t};

steps!(crate::MyWorld => {
    given regex r#"^I have a canvas with images and window dimensions$"# (crate::MyWorld, _matches, _step) {
        // Set up a canvas with images and window dimensions
        world.window_dimensions = (800, 600);
        world.image_states = vec![
            pixel_arranger::app::ImageState {
                id: 0,
                image_path: "".to_string(),
                position: egui::Pos2::new(0.0, 0.0),
                original_size: egui::Vec2::new(100.0, 100.0),
                current_size: egui::Vec2::new(100.0, 100.0),
            },
        ];
    }

    when regex r#"^I close the application$"# (crate::MyWorld, _matches, _step) {
        // Save the application state to a JSON file
        world.save_state_to_file();
    }

    then regex r#"^the application state should be saved to a JSON file$"# (crate::MyWorld, _matches, _step) {
        // Check that the application state was saved to a JSON file
        assert!(world.state_file_exists());
    }

    given regex r#"^I have a saved application state in a JSON file$"# (crate::MyWorld, _matches, _step) {
        // Create a JSON file with the saved application state
        world.create_saved_state_file();
    }

    when regex r#"^I start the application$"# (crate::MyWorld, _matches, _step) {
        // Load the application state from the JSON file
        world.load_state_from_file();
    }

    then regex r#"^the application should load the state from the JSON file$"# (crate::MyWorld, _matches, _step) {
        // Check that the application state was loaded from the JSON file
        assert!(world.state_was_loaded());
    }
});
