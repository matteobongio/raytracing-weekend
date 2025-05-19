use std::io::Write;
pub trait WriteColor<W: Write> {
    fn write_color(&self, writer: &mut W) -> std::io::Result<()>;
}
