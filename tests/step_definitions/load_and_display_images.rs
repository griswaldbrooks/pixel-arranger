use cucumber_rust::{Steps, t};

steps!(crate::MyWorld => {
    given regex r#"^I have an empty canvas$"# (crate::MyWorld, _matches, _step) {
        // Set up an empty canvas
        world.image_states = vec![];
    }

    when regex r#"^I add an image to the canvas$"# (crate::MyWorld, _matches, _step) {
        // Simulate adding an image to the canvas
        world.add_image_to_canvas("image_path");
    }

    then regex r#"^the canvas should display the image$"# (crate::MyWorld, _matches, _step) {
        // Check that the canvas displays the image
        assert!(world.image_was_added("image_path"));
    }
});

