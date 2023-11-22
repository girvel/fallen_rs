pub trait HasComponent<T> {
    fn get_component_raw(&self) -> &T;
}

pub fn component<C, E: HasComponent<C>>(entity: &E) -> &C {
    HasComponent::<C>::get_component_raw(entity)
}

pub trait AwareOfComponent<T> {
    fn try_get_component_raw(&self) -> Option<&T>;
}

pub fn try_component<C, E: AwareOfComponent<C>>(entity: &E) -> Option<&C> {
    AwareOfComponent::<C>::try_get_component_raw(entity)
}