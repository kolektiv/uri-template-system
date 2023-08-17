use crate::util::satisfy::Satisfy;

// =============================================================================
// 2-Tuple
// =============================================================================

// Implementation

impl<S1, S2> Satisfy for (S1, S2)
where
    S1: Satisfy,
    S2: Satisfy,
{
    fn satisfy(&self, input: &str) -> usize {
        let mut pos = 0;
        let mut exhausted = (true, true);

        loop {
            if input[pos..].is_empty() {
                break;
            }

            if exhausted.0 {
                match self.0.satisfy(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.1 = true;
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
                    }
                    _ => {}
                }

                exhausted.1 = false;
            }

            if !(exhausted.0 || exhausted.1) {
                break;
            }
        }

        pos
    }
}
