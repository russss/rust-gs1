use gs1::epc::{decode_binary, EPCValue, GS1};
use hex;


#[test]
fn test_decode() {
    let data = [48, 57, 96, 98, 195, 161, 168, 0, 0, 107, 51, 244];
    let result = decode_binary(&data).unwrap();
    assert_eq!(result.to_uri(), "urn:epc:id:sgtin:360843.0951968.7025652");

    let val = match result.get_value() {
        EPCValue::SGTIN96(a) => a,
        _ => { panic!("Invalid type") }
    };

    assert_eq!(val.company, 360843);
    assert_eq!(val.item, 951968);
    assert_eq!(val.to_gs1(), "(01) 03608439519680 (21) 7025652");

    let data = [0, 176, 122, 20, 12, 95, 156, 81, 64, 0, 3, 238];
    let result = decode_binary(&data).unwrap();
    let _val = match result.get_value() {
        EPCValue::Unprogrammed(a) => a,
        _ => { panic!("Invalid type") }
    };

    let data = [226, 0, 0, 25, 6, 12, 2, 9, 6, 144, 211, 194];
    match decode_binary(&data) {
        Err(_msg) => assert!(true),
        _ => assert!(false)
    };
}

// Examples from GS1 EPC E.3
#[test]
fn test_examples() {
    // SGTIN-96
    let data = decode_binary(&hex::decode("3074257BF7194E4000001A85").unwrap()).unwrap();
    assert_eq!(data.to_uri(), "urn:epc:id:sgtin:0614141.812345.6789");
    assert_eq!(data.to_tag_uri(), "urn:epc:tag:sgtin-96:3.0614141.812345.6789");

    let data = match data.get_value() {
        EPCValue::SGTIN96(val) => val,
        _ => { panic!("Invalid type") }
    };
    assert_eq!(data.to_gs1(), "(01) 80614141123458 (21) 6789");


    // SGTIN-198
    let data = decode_binary(&hex::decode("3674257BF6B7A659B2C2BF100000000000000000000000000000").unwrap()).unwrap();
    assert_eq!(data.to_uri(), "urn:epc:id:sgtin:0614141.712345.32a%2Fb");
    assert_eq!(data.to_tag_uri(), "urn:epc:tag:sgtin-198:3.0614141.712345.32a%2Fb");

    let data = match data.get_value() {
        EPCValue::SGTIN198(val) => val,
        _ => { panic!("Invalid type") }
    };
    assert_eq!(data.to_gs1(), "(01) 70614141123451 (21) 32a/b");

    // SSCC-96
    let data = decode_binary(&hex::decode("3174257BF4499602D2000000").unwrap()).unwrap();
    assert_eq!(data.to_uri(), "urn:epc:id:sscc:0614141.1234567890");
    assert_eq!(data.to_tag_uri(), "urn:epc:tag:sscc-96:3.0614141.1234567890");

    let data = match data.get_value() {
        EPCValue::SSCC96(val) => val,
        _ => { panic!("Invalid type") }
    };
    assert_eq!(data.to_gs1(), "(00) 106141412345678908");
}
