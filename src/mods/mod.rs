pub mod secrets;

/// Modifier is able to modify a target of type T
#[async_trait::async_trait]
pub trait Modifier<T> {
    async fn modify(&mut self, target: T) -> T;
}

/// Modifiers is a collection of structs that implement the Modifier trait
///
/// Calling `reduce` performs the equivalent of a fold() on modifiers returning
/// the result of modifier.modify() on each fold
pub struct Modifiers<'a, T> {
    mods: Box<&'a mut [Box<dyn Modifier<T> + Send>]>
}

impl<'a, T> Modifiers<'a, T> {
    pub fn new(mods: Box<&'a mut [Box<dyn Modifier<T> + Send>]>) -> Self {
        Modifiers { mods }
    }

    pub async fn reduce(&mut self, target: T) -> T {
        if self.mods.is_empty() {
            return target;
        }

        let mut res = target;
        for m in self.mods.iter_mut() {
            res = m.modify(res).await;
        }
        res
    }
}