use crate::noise_fns::NoiseFn;
use std::cell::{Cell, RefCell};

/// Noise function that caches the last output value generated by the source
/// function.
///
/// If the input coordinates passed to `Cache::get` are equal to the previous
/// call, the function returns the cached result of the previous call to
/// `Source::get`. Otherwise, `Source::get` is called with the new coordinates,
/// overwriting the cache with the result, and returning the result to the
/// caller.
///
/// Caching a noise function is useful if it is used as a source function for
/// multiple noise functions. If a source function is not cached, the source
/// function will redundantly calculate the same output value once for each
/// noise function in which it is included.
#[derive(Clone, Debug)]
pub struct Cache<Source> {
    /// Outputs the value to be cached.
    pub source: Source,

    value: Cell<Option<f64>>,

    point: RefCell<Vec<f64>>,
}

impl<Source> Cache<Source> {
    pub fn new(source: Source) -> Self {
        Cache {
            source,
            value: Cell::new(None),
            point: RefCell::new(Vec::new()),
        }
    }
}

impl<Source, const DIM: usize> NoiseFn<f64, DIM> for Cache<Source>
where
    Source: NoiseFn<f64, DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        match self.value.get() {
            Some(value) if quick_eq(&*self.point.borrow(), &point) => value,
            Some(_) | None => {
                let value = self.source.get(point);
                self.value.set(Some(value));

                let mut cached_point = self.point.borrow_mut();
                cached_point.clear();
                cached_point.extend_from_slice(&point);

                value
            }
        }
    }
}

fn quick_eq(a: &[f64], b: &[f64]) -> bool {
    assert_eq!(a.len(), b.len());

    a.iter().eq(b)
}
