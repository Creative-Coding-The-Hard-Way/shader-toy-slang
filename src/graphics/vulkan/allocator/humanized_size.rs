/// Used to pretty-print the size of a block.
pub struct HumanizedSize(pub u64);

impl std::fmt::Debug for HumanizedSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0;
        f.write_fmt(format_args!("{}", size))?;
        let power = (size as f64).log(1024.0).floor() as i32;
        let div = 1024.0_f64.powi(power);
        let human_size = size as f64 / div;
        match power {
            1 => {
                f.write_fmt(format_args!(" ({:.2} kb)", human_size))?;
            }
            2 => {
                f.write_fmt(format_args!(" ({:.2} mb)", human_size))?;
            }
            3 => {
                f.write_fmt(format_args!(" ({:.2} gb)", human_size))?;
            }
            _ => {}
        }
        Ok(())
    }
}
