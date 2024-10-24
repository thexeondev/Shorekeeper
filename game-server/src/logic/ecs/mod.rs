pub mod component;
pub mod entity;
pub mod world;

#[macro_export]
macro_rules! find_component {
    ($comps:expr, $comp:ident) => {
        $comps.iter().find_map(|comp| {
            let r = comp.try_borrow_mut().ok()?;
            if matches!(&*r, ComponentContainer::$comp(_)) {
                Some(::std::cell::RefMut::map(r, |r| {
                    let ComponentContainer::$comp(comp_inner) = r else {
                        unreachable!()
                    };
                    comp_inner
                }))
            } else {
                None
            }
        })
    };
}

// Query specified components from all entities (and)
#[macro_export]
macro_rules! query_with {
    ($world_entitys:expr, $($comp:ident),*) => {
        $world_entitys.components().iter().filter(|(_, comps)| {
            $(comps.iter().any(|comp| matches!(&*comp.borrow(), ComponentContainer::$comp(_))) && )
            * true
        })
        .map(|(e, comps)| {
            (*e,
                $(
                    $crate::find_component!(comps, $comp).unwrap(),
                )*
            )
        })
        .collect::<Vec<_>>()
    };
}

// Query specified components from all entities (or)
#[macro_export]
macro_rules! query_hn_with {
    ($world_entitys:expr, $($comp:ident),*) => {
        $world_entitys.components().iter().filter(|(_, comps)| {
            $(comps.iter().any(|comp| matches!(&*comp.borrow(), ComponentContainer::$comp(_))) || )
            * false
        })
        .map(|(e, comps)| {
            (*e,
                $(
                    $crate::find_component!(comps, $comp),
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
    ($world_entitys:expr, $entity_id:expr, $($comp:ident),*) => {
        $world_entitys.components().iter().find(|(id, _)| $entity_id == i64::from(**id))
        .map(|(_, comps)| {
            ($(
               $crate::find_component!(comps, $comp),
            )*)
        })
        .unwrap_or_else(|| ($( $crate::ident_as_none!($comp), )*))
    };
}

#[macro_export]
macro_rules! modify_component {
    ($comps:expr, $comp:ident, $modifier:expr) => {
        $comps.iter_mut().for_each(|comp| {
            if let ComponentContainer::$comp(ref mut inner_comp) = &mut **comp {
                $modifier(inner_comp);
            }
        });
    };
}
