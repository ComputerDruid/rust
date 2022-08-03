// edition:2018

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

trait T {
    async fn foo();
    // async fn bar(&self);
}

// Just used as an example to compare against
trait TExample {
    type Output;
    fn example() -> Self::Output;
}

struct F;

impl Future for F {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<()> {
        todo!()
    }
}

fn free() -> F {
    todo!()
}

struct Baz;

impl T for Baz {
    type FakeAsyncAssocItem = F;
    fn foo() -> F {
        F
    }
}

// Also used as an example to compare against
async fn af() {}

fn main() {}
