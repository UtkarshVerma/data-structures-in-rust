pub fn two_crystal_balls(breaks: &[bool]) -> isize {
    let n = breaks.len();
    let step_size = (n as f32).sqrt() as usize;

    let mut first_break = n;

    for (i, b) in breaks.iter().enumerate().step_by(step_size) {
        if *b {
            first_break = i;
            break;
        }
    }

    for (i, b) in breaks
        .iter()
        .enumerate()
        .skip(first_break - step_size)
        .take(step_size)
    {
        if *b {
            return i as isize;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_two_crystal_balls() {
        let mut rng = rand::thread_rng();

        const N: usize = 1000;
        let idx = rng.gen_range(0..N);
        let mut data = [false; N];

        (idx..N).for_each(|i| {
            data[i] = true;
        });

        assert_eq!(two_crystal_balls(&data), idx as isize);
        assert_eq!(two_crystal_balls(&[false; 821]), -1);
    }
}
