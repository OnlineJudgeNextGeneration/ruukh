use ruukh::{component::Status, prelude::*};
use std::{cell::RefCell, rc::Rc};

#[test]
fn should_impl_lifecycle() {
    impl Component for Button {
        type Props = ();
        type Events = ();
        type State = ();

        fn init(_: Self::Props, _: Self::Events, _: Status<Self::State>) -> Self {
            unimplemented!()
        }

        fn update(&mut self, _: Self::Props, _: Self::Events) -> Option<Self::Props> {
            unimplemented!()
        }

        fn refresh_state(&mut self) {
            unimplemented!()
        }

        fn status(&self) -> Option<&Rc<RefCell<Status<Self::State>>>> {
            unimplemented!()
        }

        fn set_state(&self, _: impl FnMut(&mut Self::State)) {
            unimplemented!()
        }
    }

    #[derive(Lifecycle)]
    struct Button;

    Button.created();
}
