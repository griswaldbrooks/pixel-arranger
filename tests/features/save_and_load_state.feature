Feature: Save and load application state
  As a user
  I want to save and load the application state
  So that I can continue where I left off

  Scenario: Save the application state to a JSON file
    Given I have a canvas with images and window dimensions
    When I close the application
    Then the application state should be saved to a JSON file

  Scenario: Load the application state from a JSON file
    Given I have a saved application state in a JSON file
    When I start the application
    Then the application should load the state from the JSON file
