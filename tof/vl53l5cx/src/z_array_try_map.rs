/*
* Array '::try_map', until it's available in stable.
*
* Turns a fixed-size array into another, of the same type, handling 'Result' conversion along the way.
*
* References:
*   - "Tracking issue for `array::try_map`" (Dec'20)
*       -> https://github.com/rust-lang/rust/issues/79711
*   - "How do I collect into an array?" (SO; Nov'14)
*       -> https://stackoverflow.com/questions/26757355/how-do-i-collect-into-an-array
*/
use vl53l5cx_uld::Result;

// ArrayVec
#[cfg(feature = "flock")]
pub(crate) fn turn_to_something<A,B, const N: usize>(aa: [A;N], f: impl FnMut(A) -> Result<B>) -> Result<[B;N]> {
    use arrayvec::ArrayVec;
    let bs_av = aa.into_iter().map(f).collect::<Result<ArrayVec<B,N>>>() ?;

    Ok(bs_av.into_inner().ok().unwrap())
}

/*
* With 'MaybeUninit'
*/
#[cfg(not(all()))]
pub(crate) fn turn_to_something<A,B, const N: usize>(aa: [A;N], f: impl FnMut(A) -> Result<B>) -> Result<[B;N]> {
    use core::mem::MaybeUninit;
    use core::mem::transmute;

    // Ref -> https://doc.rust-lang.org/beta/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
    //
    let tmp: [B;N] = {
        let mut uns: [MaybeUninit<B>; N] = [const { MaybeUninit::uninit() }; N];

        for (i,a) in aa.into_iter().enumerate() {
            uns[i] = match f(a) {
                Ok(b) => {
                    let un = MaybeUninit::new(b); unsafe{ un.assume_init() }
                },
                Err(e) => {
                    //break (e, i-1)    // "break with a value not allowed in for loops" :(

                    // Drop the already collected, transformed entries, and fail.
                    for mut x in &uns[..i] {
                        unsafe { x.assume_init_drop() }
                    }
                    return Err(e);
                }
            }
        }

        // "Everything is initialized. Transmute the array to the initialized type."
        //
        unsafe { transmute::<_, [B;N]>(uns) }
    };
    Ok(tmp)
}

/***
* NOTE: THIS SHOULD BE A NO-BRAINER. What am I doing wrong..?
*
*   <<
* error[E0277]: an array of type `[B; N]` cannot be built directly from an iterator
* = help: the trait `FromIterator<B>` is not implemented for `[B; N]`, which is required by `Result<[B; N], vl53l5cx_uld::Error>: FromIterator<Result<B, vl53l5cx_uld::Error>>`
*   = help: the trait `FromIterator<Result<A, E>>` is implemented for `Result<V, E>`
*   = note: required for `Result<[B; N], vl53l5cx_uld::Error>` to implement `FromIterator<Result<B, vl53l5cx_uld::Error>>`
*   <<
***/
#[cfg(not(all()))]
pub(crate) fn turn_to_something<A,B, const N: usize>(aa: [A;N], f: impl FnMut(A) -> Result<B>) -> Result<[B;N]> {
    let tmp= aa.into_iter().map(f)
        .collect::<Result<[B;N]>>();
    tmp
}

#[cfg(not(all()))]
pub(crate) fn turn_to_something<A,B, const N: usize>(aa: [A;N], f: impl FnMut(A) -> Result<B>) -> Result<[B;N]> {
    let mut iter = aa.into_iter();

    let tmp: [_; N] = core::array::from_fn(|_| f(iter.next()));
    tmp
}
