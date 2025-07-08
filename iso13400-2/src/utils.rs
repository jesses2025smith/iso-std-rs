use crate::Iso13400Error;

pub(crate) fn data_len_check(
    data: &[u8],
    struct_len: usize,
    equal: bool,
) -> Result<(usize, usize), Iso13400Error> {
    let actual = data.len();
    let expected = struct_len;
    if equal {
        if actual != expected {
            return Err(Iso13400Error::InvalidLength { actual, expected });
        }
    } else if expected > actual {
        return Err(Iso13400Error::InvalidLength { actual, expected });
    }

    Ok((actual, 0))
}
