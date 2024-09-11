pub mod component;
pub mod entity;
pub mod world;

// Query specified components from all entities
#[macro_export]
macro_rules! query_with {
    ($world:expr, $($comp:ident),*) => {
        $world.components().iter().filter(|(_, comps)| {
            $(comps.iter().any(|comp| matches!(&*comp.borrow(), ComponentContainer::$comp(_))) && )* true
        })
        .map(|(e, comps)| {
            (*e,
                $(
                    comps.iter().find_map(|comp| {
                        let r = comp.try_borrow_mut().ok()?;
                        if matches!(&*r, ComponentContainer::$comp(_)) {
                            Some(::std::cell::RefMut::map(r, |r| {
                                let ComponentContainer::$comp(comp_inner) = r else { unreachable!() };
                                comp_inner
                            }))
                        }
                        else {
                            None
                        }
                    }).unwrap(),
                )*
            )
        })
        .collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! ident_as_none {
    ($t:ident) => {
        None
    };
}

// Query components of specified entity
#[macro_export]
macro_rules! query_components {
    ($world:expr, $entity_id:expr, $($comp:ident),*) => {
        $world.components().iter().find(|(id, _)| $entity_id == i64::from(**id))
        .map(|(_, comps)| {
            ($(
                comps.iter().find_map(|comp| {
                    let r = comp.try_borrow_mut().ok()?;
                    if matches!(&*r, ComponentContainer::$comp(_)) {
                        Some(::std::cell::RefMut::map(r, |r| {
                            let ComponentContainer::$comp(comp_inner) = r else { unreachable!() };
                            comp_inner
                        }))
                    }
                    else {
                        None
                    }
                }),
            )*)
        })
        .unwrap_or_else(|| ($( crate::ident_as_none!($comp), )*))
    };
}
