use cucumber_rust::{Steps, t};

steps!(crate::MyWorld => {
    given regex r#"^I have an empty canvas$"# (crate::MyWorld, _matches, _step) {
        // Set up an empty canvas
        world.image_states = vec![];
    }

    when regex r#"^I open the context menu and select "Add Image"$"# (crate::MyWorld, _matches, _step) {
        // Simulate opening the context menu and selecting "Add Image"
        world.simulate_context_menu_add_image();
    }

    and regex r#"^I choose an image from the file picker dialog$"# (crate::MyWorld, _matches, _step) {
        // Simulate choosing an image from the file picker dialog
        world.simulate_file_picker_dialog("image_path");
    }

    then regex r#"^the canvas should display the chosen image$"# (crate::MyWorld, _matches, _step) {
        // Check that the canvas displays the chosen image
        assert!(world.image_was_added("image_path"));
    }

    given regex r#"^I have a canvas with an image$"# (crate::MyWorld, _matches, _step) {
        // Set up a canvas with an image
        world.image_states = vec![
            pixel_arranger::app::ImageState {
                id: 0,
                image_path: "image_path".to_string(),
                position: egui::Pos2::new(0.0, 0.0),
                original_size: egui::Vec2::new(100.0, 100.0),
                current_size: egui::Vec2::new(100.0, 100.0),
            },
        ];
    }

    when regex r#"^I open the context menu on the image and select "Remove Image"$"# (crate::MyWorld, _matches, _step) {
        // Simulate opening the context menu on the image and selecting "Remove Image"
        world.simulate_context_menu_remove_image(0);
    }

    then regex r#"^the canvas should no longer display the image$"# (crate::MyWorld, _matches, _step) {
        // Check that the canvas no longer displays the image
        assert!(world.image_was_removed("image_path"));
    }
});

