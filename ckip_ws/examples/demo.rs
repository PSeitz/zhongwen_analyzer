fn main() -> pyo3::PyResult<()> {
    let tokens = ckip_ws::segment("我愛傳統中文")?;
    println!("{tokens:?}");
    Ok(())
}
