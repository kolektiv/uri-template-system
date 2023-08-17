use crate::util::satisfy::Satisfy;

impl<S1, S2, S3> Satisfy for (S1, S2, S3)
where
    S1: Satisfy,
    S2: Satisfy,
    S3: Satisfy,
{
    fn satisfy(&self, input: &str) -> usize {
        let mut pos = 0;
        let mut exhausted = (true, true, true);

        loop {
            if input[pos..].is_empty() {
                break;
            }

            if exhausted.1 {
                match self.0.satisfy(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.1 = true;
                        exhausted.2 = true;
                    }
                    _ => {}
                }

                exhausted.0 = false;
            }

            if exhausted.1 {
                match self.1.satisfy(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.0 = true;
                        exhausted.2 = true;
                    }
                    _ => {}
                }

                exhausted.1 = false;
            }

            if exhausted.2 {
                match self.2.satisfy(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.0 = true;
                        exhausted.1 = true;
                    }
                    _ => {}
                }

                exhausted.2 = false;
            }

            if !(exhausted.0 || exhausted.1 || exhausted.2) {
                break;
            }
        }

        pos
    }
}
