use crate::error::Error;

pub(crate) fn data_len_check(
    data: &[u8],
    struct_len: usize,
    equal: bool,
) -> Result<(usize, usize), Error> {
    let actual = data.len();
    let expected = struct_len;
    if equal {
        if actual != expected {
            return Err(Error::InvalidLength { actual, expected });
        }
    } else if expected > actual {
        return Err(Error::InvalidLength { actual, expected });
    }

    Ok((actual, 0))
}
