use std::future::Future;

pub trait Handler<Args>: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
  impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
  where
      Func: Fn($($param),*) -> Fut + Clone + 'static,
      Fut: Future,
  {
      type Output = Fut::Output;
      type Future = Fut;

      #[inline]
      #[allow(non_snake_case)]
      fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
          (self)($($param,)*)
      }
  }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }
factory_tuple! { A B C D E F G H I J K L M }
factory_tuple! { A B C D E F G H I J K L M N }
factory_tuple! { A B C D E F G H I J K L M N O }
factory_tuple! { A B C D E F G H I J K L M N O P }

#[cfg(test)]
mod tests {
    use crate::extractor::FromContext;

    use super::*;

    fn assert_impl_handler<T: FromContext>(_: impl Handler<T>) {}

    #[test]
    fn arg_number() {
        async fn handler_min() {}

        #[rustfmt::skip]
        #[allow(clippy::too_many_arguments, clippy::just_underscores_and_digits, clippy::let_unit_value)]
        async fn handler_max(
            _01: (), _02: (), _03: (), _04: (), _05: (), _06: (),
            _07: (), _08: (), _09: (), _10: (), _11: (), _12: (),
            _13: (), _14: (), _15: (), _16: (),
        ) {}

        assert_impl_handler(handler_min);
        assert_impl_handler(handler_max);
    }
}
