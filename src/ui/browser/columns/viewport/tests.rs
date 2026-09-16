// SPDX-License-Identifier: MIT

use super::*;

#[test]
fn the_current_slot_is_the_widest() {
    for (visible, has_child, current) in [(2, false, 1), (3, false, 2), (3, true, 1)] {
        let ratios = slot_ratios(visible, has_child);
        let widest = ratios
            .iter()
            .enumerate()
            .max_by_key(|(_, ratio)| **ratio)
            .map(|(index, _)| index);
        assert_eq!(
            widest,
            Some(current),
            "visible {visible}, child {has_child}"
        );
    }
}

#[test]
fn slots_cover_the_viewport_exactly() {
    for viewport in [1600, 1601, 1602, 1603, 1920] {
        let widths = slot_widths(viewport, &slot_ratios(3, true)).expect("divisible viewport");
        assert_eq!(widths.iter().sum::<i32>(), viewport);
    }
}

#[test]
fn viewports_too_narrow_to_divide_keep_free_growing_columns() {
    assert!(slot_widths(600, &slot_ratios(3, true)).is_none());
    assert!(slot_widths(0, &slot_ratios(3, true)).is_none());
}
