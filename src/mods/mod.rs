pub mod secrets;

#[async_trait:async_trait]
pub trait Modifier<T> {
    async fn modify(&mut self, target: T) -> T;
}

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