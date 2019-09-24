use num_enum::IntoPrimitive;

// GS1 General Specifications, Figure 3.2-1
#[repr(u16)]
#[derive(Debug, IntoPrimitive)]
pub(crate) enum ApplicationIdentifier {
    SSCC = 0,
    GTIN = 1,
    GTINContent = 2,
    Batch = 10,
    ProductionDate = 11,
    DueDate = 12,
    PackagingDate = 13,
    BestBeforeDate = 15,
    SellByDate = 16,
    ExpirationDate = 17,
    InternalProductVariant = 20,
    SerialNumber = 21
}
