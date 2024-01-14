pub trait HasComponent<T> {
    fn get_component_raw(&self) -> &T;
    fn get_component_mut_raw(&mut self) -> &mut T;
}

pub fn component<C, E: HasComponent<C>>(entity: &E) -> &C {
    HasComponent::<C>::get_component_raw(entity)
}

pub fn component_mut<C, E: HasComponent<C>>(entity: &mut E) -> &mut C {
    HasComponent::<C>::get_component_mut_raw(entity)
}

pub trait AwareOfComponent<T> {
    fn try_get_component_raw(&self) -> Option<&T>;
    fn try_get_component_mut_raw(&mut self) -> Option<&mut T>;
}

pub fn try_component<C, E: AwareOfComponent<C>>(entity: &E) -> Option<&C> {
    AwareOfComponent::<C>::try_get_component_raw(entity)
}

pub fn try_component_mut<C, E: AwareOfComponent<C>>(entity: &mut E) -> Option<&mut C> {
    AwareOfComponent::<C>::try_get_component_mut_raw(entity)
}

pub trait CanRegister<T> {
    fn register(&mut self, entity: T);
}
