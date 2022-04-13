use thunk_simple::*;

mod test_intrinsic_methods {
    use std::rc::Rc;
    use std::cell::RefCell;
    use thunk_simple::*;

    #[test]
    fn thunk_new() {
        let external_state = Rc::new(RefCell::new(true));
        let external_state_ptr = Rc::clone(&external_state);
        let thunk = Thunk::<'_,&'static str>::new(move || {
            *external_state_ptr.borrow_mut() = false;
            "Output"
        });
        assert_eq!(*external_state.borrow(),true,"Thunk modified external state before its unwrapping");
        assert_eq!(thunk.unwrap(),"Output","Thunk output wrong string");
        assert_eq!(*external_state.borrow(),false,"Thunk didn't modify state after its unwrapping");
    }

    #[test]
    fn thunk_new_const() {
        assert_eq!(Thunk::<'_,bool>::new_const(true).unwrap(),true,"Thunk returned wrong value");
    }

    #[test]
    fn thunk_map() {
        let mut thunk = Thunk::<'_,&'static str>::new_const("Output");
        thunk = thunk.map(|val| &val[1..5]);
        assert_eq!(thunk.unwrap(),"utpu","Thunk did not correctly apply mapping function");
    }

    #[test]
    #[should_panic]
    fn thunk_map_panic() {
        let thunk_dangerous = Thunk::<'_,bool>::new_const(true);
        assert_eq!(thunk_dangerous.map(|v| v || false).unwrap(),false);
    }

    #[test]
    fn thunk_map_lazy() {
        let mut thunk = Thunk::<'_,&'static str>::new_const("Output");
        thunk = thunk.map_lazy(|val| &(val.unwrap()[1..5]));
        assert_eq!(thunk.unwrap(),"utpu","Thunk did not correctly apply lazy mapping function");

        let thunk_dangerous = Thunk::<'_,bool>::new_const(true);
        let thunk_safe = thunk_dangerous.map_lazy(|_| false); // Doesn't panic because original thunk is never unwrapped
        assert_eq!(thunk_safe.unwrap(),false,"Thunk returned wrong value");
    }

    #[test]
    #[should_panic]
    fn thunk_map_lazy_panic() {
        let thunk_dangerous = Thunk::<'_,bool>::new_const(true);
        assert_eq!(thunk_dangerous.map_lazy(|prev_val| prev_val.unwrap() || false).unwrap(),false); // Panics, because it unwraps previous thunk
    }
}

mod thunk_from_iter {
    use thunk_simple::*;

    #[test]
    fn thunk_from_iter() {
        let thunk = Thunk::<'_,u32>::from(vec![1,2,3].into_iter());
        assert_eq!(thunk.unwrap(),1,"Thunk did not correctly convert iterator into itself");
        // Should not panic because thunk is never unwrapped
        let thunk_potential_panic: Thunk<u32> = Thunk::from(vec![].into_iter());
        let thunk_now_safe = thunk_potential_panic.map_lazy(|_| true);
        assert_eq!(thunk_now_safe.unwrap(),true,"Thunk did not return correct value");
    }
    #[test]
    #[should_panic]
    fn thunk_from_iter_panic() {
        let thunk = Thunk::<'_,u32>::from(vec![].into_iter());
        thunk.unwrap();
    }
}

#[test]
fn thunk_into_iter() {
    let thunk = Thunk::<'_,bool>::new_const(true);
    let collected_vec = thunk.into_iter().take(3).collect::<Vec<bool>>();
    assert_eq!(collected_vec,vec![true]);

    let thunk_identity = Thunk::<'_,bool>::new_const(true);
    let reformed_thunk = Thunk::from(thunk_identity.into_iter());
    assert_eq!(reformed_thunk.unwrap(),true,"Thunk returned wrong value");
}