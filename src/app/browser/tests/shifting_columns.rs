// SPDX-License-Identifier: MIT

use super::*;

fn shifting_browser() -> Rc<Browser> {
    let browser = Browser::new(Rc::new(FakeFileSource));
    browser.set_shifting_columns(true);
    browser.navigate(Location::local("/fixture"));
    browser
}

#[test]
fn a_fresh_location_opens_with_its_parent_beside_it() {
    let browser = shifting_browser();

    assert_eq!(browser.location_at(0), Some(Location::local("/")));
    assert_eq!(browser.location_at(1), Some(Location::local("/fixture")));
    assert_eq!(browser.active_depth(), Some(1));
}

#[test]
fn free_growing_columns_still_open_a_single_column() {
    let browser = Browser::new(Rc::new(FakeFileSource));

    browser.navigate(Location::local("/fixture"));

    assert_eq!(browser.location_at(0), Some(Location::local("/fixture")));
    assert_eq!(browser.location_at(1), None);
    assert_eq!(browser.active_depth(), Some(0));
}

#[test]
fn the_preview_shows_the_focused_folder_without_entering_it() {
    let browser = shifting_browser();
    browser.select(1, 0);

    browser.sync_child_preview();

    assert_eq!(
        browser.location_at(2),
        Some(Location::local("/fixture/child"))
    );
    assert_eq!(browser.active_depth(), Some(1));
    assert_eq!(browser.active_location(), Some(Location::local("/fixture")));
    assert!(!browser.can_go_back());
}

#[test]
fn the_preview_arrives_with_its_first_row_selected() {
    let browser = shifting_browser();
    browser.select(1, 0);

    browser.sync_child_preview();

    assert_eq!(browser.selected_positions(2), vec![0]);
}

#[test]
fn entering_a_column_lands_on_its_first_row() {
    let browser = shifting_browser();
    browser.select(1, 0);
    browser.sync_child_preview();

    browser.activate_focused();

    assert_eq!(browser.active_depth(), Some(2));
    assert_eq!(browser.selected_positions(2), vec![0]);
}

#[test]
fn entering_the_preview_records_navigation() {
    let browser = shifting_browser();
    browser.select(1, 0);
    browser.sync_child_preview();

    browser.activate_focused();

    assert_eq!(browser.active_depth(), Some(2));
    assert_eq!(
        browser.active_location(),
        Some(Location::local("/fixture/child"))
    );
    assert!(browser.can_go_back());
}

#[test]
fn moving_left_goes_up_a_level_and_keeps_a_parent_beside_it() {
    let browser = shifting_browser();
    browser.select(1, 0);
    browser.sync_child_preview();
    browser.activate_focused();
    browser.sync_child_preview();

    browser.focus_parent();

    assert_eq!(browser.active_location(), Some(Location::local("/fixture")));
    assert_eq!(browser.location_at(0), Some(Location::local("/")));
    assert_eq!(
        browser
            .selected_entries()
            .first()
            .map(|entry| &entry.location),
        Some(&Location::local("/fixture/child"))
    );
}

#[test]
fn the_preview_stays_out_of_the_path_after_entering_and_leaving() {
    let browser = shifting_browser();
    browser.select(1, 0);
    browser.sync_child_preview();
    browser.activate_focused();
    browser.sync_child_preview();

    browser.focus_parent();
    browser.sync_child_preview();

    assert_eq!(browser.active_depth(), Some(1));
    assert_eq!(browser.location_at(3), None);
}

#[test]
fn turning_the_preference_off_closes_the_preview() {
    let browser = shifting_browser();
    browser.select(1, 0);
    browser.sync_child_preview();

    browser.set_shifting_columns(false);

    assert_eq!(browser.location_at(2), None);
    assert_eq!(browser.active_depth(), Some(1));
}
