/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// # Example
///
/// ```
/// ensure!(vec.len() > 0, EmptyVecError::new());
/// ```
///
/// is equivalent to
///
/// ```
/// if !(vec.len() > 0) {
///     return Err(EmptyVecError::new().into());
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e.into());
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error {
    (cond: $cond:expr, $($arg:tt)+) => (
        if $cond {
            error!($($arg)+)
        }
    );
    ($($arg:tt)+) => (
        ::tracing::error!($($arg)+)
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! warn {
    (cond: $cond:expr, $($arg:tt)+) => (
        if $cond {
            warn!($($arg)+)
        }
    );
    ($($arg:tt)+) => (
        ::tracing::warn!($($arg)+)
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! info {
    (cond: $cond:expr, $($arg:tt)+) => (
        if $cond {
            info!($($arg)+)
        }
    );
    ($($arg:tt)+) => (
        ::tracing::info!($($arg)+)
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    (cond: $cond:expr, $($arg:tt)+) => (
        if $cond {
            debug!($($arg)+)
        }
    );
    ($($arg:tt)+) => (
        ::tracing::debug!($($arg)+)
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace {
    (cond: $cond:expr, $($arg:tt)+) => (
        if $cond {
            trace!($($arg)+)
        }
    );
    ($($arg:tt)+) => (
        ::tracing::trace!($($arg)+)
    )
}
