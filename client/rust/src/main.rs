mod bindings;
use bindings::*;
use windows::core::*;

fn main() -> Result<()> {
    let reader = Reader::new()?;
    assert_eq!(reader.P0()?, "P0");
    assert_eq!(reader.P1()?, "P1");

    let writer = Writer::new()?;
    writer.SetP0(h!("V0"))?;
    assert_eq!(writer.P0()?, "V0");

    Ok(())
}
