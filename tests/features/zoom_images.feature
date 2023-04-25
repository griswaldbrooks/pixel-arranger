Feature: Zoom images
  As a user
  I want to zoom in and out of images using the mouse scroll wheel
  So that I can view images in more detail

  Scenario: Zoom in on an image
    Given I have a canvas with an image
    When I scroll up with the mouse wheel over the image
    Then the image should be zoomed in

  Scenario: Zoom out of an image
    Given I have a canvas with an image
    When I scroll down with the mouse wheel over the image
    Then the image should be zoomed out
