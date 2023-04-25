Feature: Add and remove images
  As a user
  I want to add and remove images using a context menu
  So that I can manage the images on the canvas

  Scenario: Add an image using the context menu
    Given I have an empty canvas
    When I open the context menu and select "Add Image"
    And I choose an image from the file picker dialog
    Then the canvas should display the chosen image

  Scenario: Remove an image using the context menu
    Given I have a canvas with an image
    When I open the context menu on the image and select "Remove Image"
    Then the canvas should no longer display the image
