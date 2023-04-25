Feature: Move images
  As a user
  I want to move images around the canvas
  So that I can arrange them as desired

  Scenario: Move an image by clicking and dragging
    Given I have a canvas with an image
    When I click and drag the image to a new position
    Then the image should be displayed at the new position
