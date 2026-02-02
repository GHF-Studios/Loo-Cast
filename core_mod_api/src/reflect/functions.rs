use crate::bevy::reflect::Reflect;

pub fn get_struct_field_ref<'a, T, F>(target: &'a T, field: &str) -> &'a F
where
    T: Reflect,
    F: Reflect + 'static,
{
    get_struct_field_ref_inner(target, field).expect("Failed to get struct field")
}

pub fn get_struct_field_mut<'a, T, F>(target: &'a mut T, field: &str) -> &'a mut F
where
    T: Reflect,
    F: Reflect + 'static,
{
    get_struct_field_mut_inner(target, field).expect("Failed to get mutable struct field")
}

fn get_struct_field_ref_inner<'a, T, F>(target: &'a T, field: &str) -> Option<&'a F>
where
    T: Reflect,
    F: Reflect + 'static,
{
    target.reflect_ref().as_struct().ok()?.field(field)?.try_downcast_ref::<F>()
}

fn get_struct_field_mut_inner<'a, T, F>(target: &'a mut T, field: &str) -> Option<&'a mut F>
where
    T: Reflect,
    F: Reflect + 'static,
{
    target.reflect_mut().as_struct().ok()?.field_mut(field)?.try_downcast_mut::<F>()
}
