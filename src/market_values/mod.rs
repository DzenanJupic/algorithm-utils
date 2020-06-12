use ::std::ops::*;
use std::fmt::Debug;

pub use percent::*;
pub use points::*;
pub use price::*;

macro_rules! impl_ops {
    ($name:path, no_percent) => {
        use std::ops::*;
        use auto_ops::impl_op_ex;

        impl AsRef<f64> for $name {
            fn as_ref(&self) -> &f64 {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = f64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<f64> for $name {
            fn from(float: f64) -> Self {
                $name(float)
            }
        }

        impl From<&f64> for $name {
            fn from(float: &f64) -> Self {
                $name(*float)
            }
        }

        impl Into<f64> for $name {
            fn into(self) -> f64 {
                self.0
            }
        }

        // lhs: $name, rhs: $name
        impl_op_ex!(+ |a: &$name, b: &$name| -> $name { $name(a.0 + b.0) });
        impl_op_ex!(- |a: &$name, b: &$name| -> $name { $name(a.0 - b.0) });
        impl_op_ex!(* |a: &$name, b: &$name| -> $name { $name(a.0 * b.0) });
        impl_op_ex!(/ |a: &$name, b: &$name| -> $name { $name(a.0 / b.0) });

        impl_op_ex!(+= |a: &mut $name, b: &$name| { a.0 += b.0; });
        impl_op_ex!(-= |a: &mut $name, b: &$name| { a.0 -= b.0; });
        impl_op_ex!(*= |a: &mut $name, b: &$name| { a.0 *= b.0; });
        impl_op_ex!(/= |a: &mut $name, b: &$name| { a.0 /= b.0; });

        // lhs: $name, rhs: f64
        impl_op_ex!(+ |a: &$name, b: &f64| -> $name { $name(a.0 + b) });
        impl_op_ex!(- |a: &$name, b: &f64| -> $name { $name(a.0 - b) });
        impl_op_ex!(* |a: &$name, b: &f64| -> $name { $name(a.0 * b) });
        impl_op_ex!(/ |a: &$name, b: &f64| -> $name { $name(a.0 / b) });

        impl_op_ex!(+= |a: &mut $name, b: &f64| { a.0 += b; });
        impl_op_ex!(-= |a: &mut $name, b: &f64| { a.0 -= b; });
        impl_op_ex!(*= |a: &mut $name, b: &f64| { a.0 *= b; });
        impl_op_ex!(/= |a: &mut $name, b: &f64| { a.0 /= b; });
    };
    ($name:path) => {
        impl_ops!($name, no_percent);

        // lhs: $name, rhs: Percent
        impl_op_ex!(+ |a: &$name, b: &Percent| -> $name { $name(a.0 * (1.0 + b.0)) });
        impl_op_ex!(- |a: &$name, b: &Percent| -> $name { $name(a.0 * (1.0 - b.0)) });
        impl_op_ex!(* |a: &$name, b: &Percent| -> $name { $name(a.0 * b.0) });
        impl_op_ex!(/ |a: &$name, b: &Percent| -> $name { $name(a.0 * b.0) });

        impl_op_ex!(+= |a: &mut $name, b: &Percent| { a.0 *= 1.0 + b.0; });
        impl_op_ex!(-= |a: &mut $name, b: &Percent| { a.0 *= 1.0 - b.0; });
        impl_op_ex!(*= |a: &mut $name, b: &Percent| { a.0 *= b.0; });
        impl_op_ex!(/= |a: &mut $name, b: &Percent| { a.0 /= b.0; });
    };
}

pub mod percent;
pub mod points;
pub mod price;

type Momentum = f64;

macro_rules! get_cross_over {
    ($instances:ident) => {
        macro_rules! skip_eq {
            ($iter:ident) => {
                loop {
                    match $iter.next() {
                        Some((_, (instance1, instance2))) =>
                            if instance1 != instance2 { break; }
                        None => return None
                    }
                }
            };
        }

        loop {
            skip_eq!($instances);

            match $instances.next() {
                Some((_, (instance1_before, instance2_before))) => {
                    skip_eq!($instances);

                    match $instances.next() {
                        Some((i, (instance1_now, instance2_now))) => {
                            if instance1_before > instance2_before {
                                if instance1_now < instance2_now {
                                    return Some(i);
                                }
                            } else {
                                if instance1_now > instance2_now {
                                    return Some(i);
                                }
                            }
                        }
                        None => return None
                    }
                }
                None => return None
            };
        }
    };
}

pub trait MarketValue:
AsRef<f64> + Deref<Target=f64> + DerefMut<Target=f64> + From<f64> + Into<f64> +
Add<Self> + AddAssign<Self> + Sub<Self> + SubAssign<Self> + Mul<Self> + MulAssign<Self> + Div<Self> + DivAssign<Self> +
Add<Percent> + AddAssign<Percent> + Sub<Percent> + SubAssign<Percent> + Mul<Percent> + MulAssign<Percent> + Div<Percent> + DivAssign<Percent> +
Add<f64> + AddAssign<f64> + Sub<f64> + SubAssign<f64> + Mul<f64> + MulAssign<f64> + Div<f64> + DivAssign<f64> +
PartialEq + PartialOrd +
Debug + Clone + Copy {
    fn new(value: f64) -> Self;

    fn value(&self) -> &f64 { self.deref() }
    fn value_mut(&mut self) -> &mut f64 { self.deref_mut() }

    fn zero() -> Self { Self::new(0.0) }
    fn one() -> Self { Self::new(1.0) }
    fn minus_one() -> Self { Self::new(-1.0) }
    fn one_hundred() -> Self { Self::new(100.0) }
    fn minus_one_hundred() -> Self { Self::new(-100.0) }

    fn from_f64_slice(floats: &[f64]) -> Vec<Self> {
        let mut instances = Vec::with_capacity(floats.len() + 10);

        floats
            .iter()
            .for_each(
                |float| instances.push(Self::from(*float))
            );

        instances
    }

    // noinspection RsSelfConvention
    fn to_f64_vec(instances: &[Self]) -> Vec<f64> {
        let mut floats = Vec::with_capacity(instances.len() + 10);

        instances
            .iter()
            .for_each(
                |instance| floats.push((*instance).into())
            );

        floats
    }

    fn simple_average(instances: &[Self]) -> Self {
        let sum = instances
            .iter()
            .skip(1)
            .fold(
                instances[0],
                |mut prev, &curr| {
                    prev += curr;
                    prev
                });
        Self::new(*sum / instances.len() as f64)
    }

    fn simple_moving_average(instances: &[Self], interval: usize) -> Vec<Self> {
        if interval == 0 || instances.len() < interval {
            return Vec::new();
        }
        let mut averages = Vec::with_capacity(instances.len() - interval + 10);

        for i in interval..=instances.len() {
            let slice = &instances[i - interval..i];
            let average = Self::simple_average(slice);

            averages.push(average);
        }

        averages
    }

    fn momentum(instances: &[Self]) -> Momentum {
        if instances.len() <= 1 {
            0.0
        } else {
            let first = *instances[0];
            let last = *instances[instances.len() - 1];

            first - last
        }
    }

    fn average_momentum(instances: &[Self]) -> Momentum {
        if instances.len() <= 1 {
            return 0.0;
        }
        let mut momentums = Vec::with_capacity(instances.len() - 1);

        for i in 1..instances.len() {
            let first = *instances[i - 1];
            let last = *instances[i];

            momentums.push(first - last);
        }

        let sum = momentums
            .iter()
            .skip(1)
            .fold(
                momentums[0],
                |prev, curr| prev + *curr,
            );
        sum / momentums.len() as f64
    }

    fn moving_momentum(instances: &[Self], interval: usize) -> Vec<Momentum> {
        if interval == 0 || instances.len() < interval {
            return Vec::new();
        }
        let mut momentums = Vec::with_capacity(instances.len() - interval + 10);

        for i in interval..=instances.len() {
            let slice = &instances[i - interval..i];
            let momentum = Self::momentum(slice);

            momentums.push(momentum);
        }

        momentums
    }

    fn growth(instances: &[Self]) -> Percent {
        if instances.len() <= 1 {
            return Percent::zero();
        }

        Percent::growth(instances[0], instances[instances.len() - 1])
    }

    fn average_growth(instances: &[Self]) -> Percent {
        if instances.len() <= 1 {
            return Percent::zero();
        }
        let mut growths = Vec::with_capacity(instances.len() - 1);

        for i in 1..instances.len() {
            let growth = Percent::growth(instances[i - 1], instances[i]);
            growths.push(growth);
        }

        Percent::simple_average(&growths)
    }

    fn moving_growth(instances: &[Self], interval: usize) -> Vec<Percent> {
        if interval == 0 || instances.len() < interval {
            return Vec::new();
        }
        let mut growths = Vec::with_capacity(instances.len() - interval + 10);

        for i in interval..instances.len() {
            let growth = Percent::growth(instances[i - interval], instances[i]);
            growths.push(growth);
        }

        growths
    }

    //noinspection RsLiveness
    fn first_cross_over(instances1: &[Self], instances2: &[Self]) -> Option<usize> {
        if instances1.len() <= 1 || instances2.len() <= 1 { return None; }

        let mut instances =
            instances1
                .iter()
                .zip(instances2)
                .enumerate();

        get_cross_over!(instances);
    }

    //noinspection RsLiveness
    fn last_cross_over(instances1: &[Self], instances2: &[Self]) -> Option<usize> {
        if instances1.len() <= 1 || instances2.len() <= 1 { return None; }

        let mut instances =
            instances1
                .iter()
                .zip(instances2)
                .enumerate()
                .rev();

        get_cross_over!(instances);
    }

    fn all_cross_overs(mut instances1: &[Self], mut instances2: &[Self]) -> Vec<usize> {
        let mut cross_overs = Vec::new();
        if instances1.len() <= 1 || instances2.len() <= 1 { return cross_overs; }

        while let Some(i) = Self::first_cross_over(&instances1, &instances2) {
            cross_overs.push(i);

            instances1 = &instances1[i..];
            instances2 = &instances2[i..];
        }

        cross_overs
    }

    //noinspection DuplicatedCode
    fn min(instances: &[Self]) -> Self {
        instances
            .iter()
            .skip(1)
            .fold(
                instances[0],
                |prev, &curr| {
                    if prev > curr {
                        curr
                    } else {
                        prev
                    }
                },
            )
    }

    //noinspection DuplicatedCode
    fn max(instances: &[Self]) -> Self {
        instances
            .iter()
            .skip(1)
            .fold(
                instances[0],
                |prev, &curr| {
                    if prev < curr {
                        curr
                    } else {
                        prev
                    }
                },
            )
    }
}
