// Copyright 2016–2017 Pablo Couto

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// version 3 as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License version 3 for more details.

// You should have received a copy of the GNU Lesser General Public
// License version 3 along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

use hyper;
use std::error::Error as StdError;
use tokio_timer;
use url;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        Hyper(hyper::Error);
        UrlParse(url::ParseError);
    }

    errors {
        Timeout {
            description("Operation timed out")
        }
        TokioTimer(error: Box<StdError + Send>) {
            description("Error at ‘tokio-timer’`")
            display("Error at ‘tokio-timer’: {}", error.description())
        }
        UnexpectedStatus(recv: hyper::StatusCode, req: hyper::StatusCode) {
            description("Unexpected status received")
            display("Expected status ‘{}’; received ‘{}’", req, recv)
        }
    }
}

// Make a clear distinction between an error at the `tokio-timer`
// crate, and the timing-out of an operation. The latter is
// represented in this crate as an error type of its own.
impl<T> From<tokio_timer::TimeoutError<T>> for Error {
    fn from(error: tokio_timer::TimeoutError<T>) -> Self {
        let error = match error {
            tokio_timer::TimeoutError::Timer(_, e) => ErrorKind::TokioTimer(Box::new(e)),
            tokio_timer::TimeoutError::TimedOut(_) => ErrorKind::Timeout,
        };
        Self::from(error)
    }
}
