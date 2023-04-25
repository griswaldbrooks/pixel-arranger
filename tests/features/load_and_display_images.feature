Feature: Load and display multiple images
  As a user
  I want to load and display multiple images on a canvas
  So that I can view them together

  Scenario: Add images to the canvas
    Given I have an empty canvas
    When I add an image to the canvas
    Then the canvas should display the image
