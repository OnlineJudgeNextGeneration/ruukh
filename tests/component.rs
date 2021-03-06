#![feature(proc_macro_gen, decl_macro)]

use ruukh::prelude::*;

#[test]
fn should_build_a_component_from_unit_struct() {
    #[component]
    struct Unit;
}

#[test]
fn should_build_a_component_from_struct() {
    #[component]
    struct Button {
        prop_a: i32,
    }

    let _ = ButtonProps!(prop_a: 5);
}

#[test]
fn should_build_a_component_with_props_only() {
    #[component]
    struct Button {
        prop_a: i32,

        #[prop]
        prop_b: i32,

        #[prop]
        prop_c: i32,

        #[prop(default = 4)]
        prop_d: i32,

        #[prop(default)]
        prop_e: i32,
    }

    let _ = ButtonProps {
        prop_a: 5,
        prop_b: 5,
        prop_c: 5,
        prop_d: 5,
        prop_e: 5,
    };

    let props = ButtonProps!(prop_a: 1, prop_b: 2, prop_c: 3, prop_d: 4);

    assert_eq!(props.prop_a, 1);
    assert_eq!(props.prop_b, 2);
    assert_eq!(props.prop_c, 3);
    assert_eq!(props.prop_d, 4);
}

#[test]
fn should_build_a_component_with_state_only() {
    #[component]
    struct Button {
        #[state]
        state_a: i32,
        #[state]
        state_b: bool,
        #[state(default = 5)]
        state_c: i32,
    }

    let _ = ButtonState {
        state_a: 5,
        state_b: false,
        state_c: 5,
    };

    let def = ButtonState::default();
    assert_eq!(def.state_a, 0);
    assert_eq!(def.state_b, false);
    assert_eq!(def.state_c, 5);
}

#[test]
fn should_build_a_component_with_state_and_props() {
    #[component]
    struct Button {
        #[prop]
        prop_a: bool,
        #[state]
        state_a: i32,
        #[state(default = 5)]
        state_c: i32,
    }

    let _ = ButtonProps!(prop_a: false);
    let _ = ButtonState::default();
}

#[test]
fn should_build_a_component_with_event() {
    #[component]
    #[events(
        fn save (&self, num: i32);
    )]
    struct Button;
}

#[test]
fn should_build_a_component_with_events() {
    #[component]
    #[events(
        fn save (&self, num: i32);

        fn click (&self) -> i32;
    )]
    struct Button;
}

#[test]
fn should_build_a_component_with_events_in_separate_attrs() {
    #[component]
    #[events(
        fn save (&self, num: i32);
    )]
    #[events(
        fn click (&self) -> i32;
    )]
    struct Button;
}

#[test]
fn should_build_a_component_with_optional_event() {
    #[component]
    #[events(
        #[optional]
        fn save(&self, num: i32);
    )]
    struct Button;
}

#[test]
fn should_build_a_component_with_everything() {
    #[component]
    #[events(
        #[optional]
        fn save(&self, num: i32);
    )]
    struct Button {
        prop_a: bool,
        #[state]
        state_b: bool,
    }
}

#[test]
fn should_build_a_component_with_default_option_props() {
    #[component]
    struct Button {
        prop_a: Option<bool>,
        #[prop(default = Some(5))]
        prop_b: Option<i32>,
    }

    let props = ButtonProps!();
    assert!(props.prop_a.is_none());
    assert_eq!(props.prop_b, Some(5));

    let props = ButtonProps!(prop_a: Some(false), prop_b: Some(3));
    assert_eq!(props.prop_a, Some(false));
    assert_eq!(props.prop_b, Some(3));
}
