use prometheus::Opts;

pub(super) trait MetricFromOpts: Sized {
    fn from_opts(opts: Opts) -> Result<Self, prometheus::Error>;
}

#[macro_export]
macro_rules! metrics {
    (
        $vis:vis struct $name:ident {
            $(
                #[doc = $help:expr]
                $(#[$meta:meta])*
                $metric_vis:vis $metric:ident: $ty:ty $([$($label:expr),* $(,)?])?
            ),* $(,)?
        }
        namespace: $namespace:expr,
    ) => {
        $vis struct $name {
            registry: prometheus::Registry,
            $(
                $(#[$meta])*
                $metric_vis $metric: $ty,
            )*
        }
        impl $name {
            $vis fn new() -> Result<Self, prometheus::Error> {
                use crate::metrics::macros::MetricFromOpts;

                let registry = prometheus::Registry::new();
                $(
                    $(#[$meta])*
                    let $metric = <$ty>::from_opts(
                        prometheus::Opts::new(stringify!($metric), $help)
                            .namespace($namespace)
                            $(.variable_labels(vec![$($label.into()),*]))?
                    )?;
                    $(#[$meta])*
                    registry.register(Box::new($metric.clone()))?;
                )*
                Ok(Self {
                    registry,
                    $(
                        $(#[$meta])*
                        $metric,
                    )*
                })
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($name))
            }
        }
    };
}

macro_rules! load_metric_type {
    ($name:ident as single) => {
        use prometheus::$name;
        impl MetricFromOpts for $name {
            fn from_opts(opts: Opts) -> Result<Self, prometheus::Error> {
                $name::with_opts(opts.into())
            }
        }
    };
    ($name:ident as vec) => {
        use prometheus::$name;
        impl MetricFromOpts for $name {
            fn from_opts(opts: Opts) -> Result<Self, prometheus::Error> {
                $name::new(
                    opts.clone().into(),
                    opts.variable_labels
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
            }
        }
    };
}

load_metric_type!(Counter as single);
load_metric_type!(CounterVec as vec);
load_metric_type!(IntCounter as single);
load_metric_type!(IntCounterVec as vec);
load_metric_type!(Gauge as single);
load_metric_type!(GaugeVec as vec);
load_metric_type!(IntGauge as single);
load_metric_type!(IntGaugeVec as vec);

// Histograms are defined in histogram.rs
