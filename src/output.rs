type Internal = Option<String>;

#[derive(Debug)]
pub struct I3Block
{
    pub name: Internal,
    pub instance: Internal,

    pub full_text: Internal,
    pub short_text: Internal,

    pub color: Internal,
    pub background: Internal,
    pub min_width: Internal,
    pub align: Internal,
    pub border: Internal,

    pub separator: Internal,
    pub separator_block_width: Internal,
    pub urgent: Internal,
    pub markup: Internal,
}

impl std::fmt::Display for I3Block {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        //unwrap_and_write(f, &self.name)?;
        //unwrap_and_write(f, &self.instance)?;

        //unwrap_and_write(f, &self.full_text)?;
        //unwrap_and_write(f, &self.short_text)?;
        //unwrap_and_write(f, &self.color)?;
        //unwrap_and_write(f, &self.background)?;
        //unwrap_and_write(f, &self.min_width)?;
        //unwrap_and_write(f, &self.align)?;
        //unwrap_and_write(f, &self.border)?;

        //unwrap_and_write(f, &self.separator)?;
        //unwrap_and_write(f, &self.separator_block_width)?;
        //unwrap_and_write(f, &self.urgent)?;
        //unwrap_and_write(f, &self.markup)?;
        Ok(())
    }

}

impl Default for I3Block
{
    fn default() -> Self
    {
        Self {
            name: None,
            instance: None,

            full_text: None,
            short_text: None,

            color: None,
            background: None,
            min_width: None,
            align: None,
            border: None,

            separator: None,
            separator_block_width: None,
            urgent: None,
            markup: None,
        }
    }
}

fn unwrap_and_write(f: &mut std::fmt::Formatter, value: &Internal) -> Result<(), std::fmt::Error> 
{
    if let Some(v) = value {
        write!(f, "{},", v)?;
    }
    Ok(())
}
