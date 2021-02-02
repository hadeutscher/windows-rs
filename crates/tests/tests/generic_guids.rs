#[test]
fn generic_guids() -> windows::Result<()> {
    type A = windows::foundation::collections::IIterable<windows::foundation::IStringable>;
    type B = windows::foundation::collections::IKeyValuePair<
        windows::HString,
        windows::foundation::IAsyncOperationWithProgress<A, f32>,
    >;

    //
    // Generated Windows.Foundation GUIDs
    //
    assert_eq!(
        windows::foundation::IAsyncActionWithProgress::<A>::IID,
        com::sys::GUID::from("DD725452-2DA3-5103-9C7D-22EE9BB14AD3")
    );

    assert_eq!(
        windows::foundation::IAsyncOperationWithProgress::<A, B>::IID,
        com::sys::GUID::from("94645425-B9E5-5B91-B509-8DA4DF6A8916")
    );

    assert_eq!(
        windows::foundation::IAsyncOperation::<A>::IID,
        com::sys::GUID::from("2BD35EE6-72D9-5C5D-9827-05EBB81487AB")
    );

    assert_eq!(
        windows::foundation::IReferenceArray::<A>::IID,
        com::sys::GUID::from("4A33FE03-E8B9-5346-A124-5449913ECA57")
    );

    assert_eq!(
        windows::foundation::IReference::<A>::IID,
        com::sys::GUID::from("F9E4006C-6E8C-56DF-811C-61F9990EBFB0")
    );

    assert_eq!(
        windows::foundation::AsyncActionProgressHandler::<A>::IID,
        com::sys::GUID::from("C261D8D0-71BA-5F38-A239-872342253A18")
    );

    assert_eq!(
        windows::foundation::AsyncActionWithProgressCompletedHandler::<A>::IID,
        com::sys::GUID::from("9A0D211C-0374-5D23-9E15-EAA3570FAE63")
    );

    assert_eq!(
        windows::foundation::AsyncOperationCompletedHandler::<A>::IID,
        com::sys::GUID::from("9D534225-231F-55E7-A6D0-6C938E2D9160")
    );

    assert_eq!(
        windows::foundation::AsyncOperationProgressHandler::<A, B>::IID,
        com::sys::GUID::from("264F1E0C-ABE4-590B-9D37-E1CC118ECC75")
    );

    assert_eq!(
        windows::foundation::AsyncOperationWithProgressCompletedHandler::<A, B>::IID,
        com::sys::GUID::from("C2D078D8-AC47-55AB-83E8-123B2BE5BC5A")
    );

    assert_eq!(
        windows::foundation::EventHandler::<A>::IID,
        com::sys::GUID::from("FA0B7D80-7EFA-52DF-9B69-0574CE57ADA4")
    );

    assert_eq!(
        windows::foundation::TypedEventHandler::<A, B>::IID,
        com::sys::GUID::from("EDB31843-B4CF-56EB-925A-D4D0CE97A08D")
    );

    //
    // Generated Windows.Foundation.Collections GUIDs
    //

    assert_eq!(
        windows::foundation::collections::IIterable::<A>::IID,
        com::sys::GUID::from("96565EB9-A692-59C8-BCB5-647CDE4E6C4D")
    );

    assert_eq!(
        windows::foundation::collections::IIterator::<A>::IID,
        com::sys::GUID::from("3C9B1E27-8357-590B-8828-6E917F172390")
    );

    assert_eq!(
        windows::foundation::collections::IKeyValuePair::<A, B>::IID,
        com::sys::GUID::from("89336CD9-8B66-50A7-9759-EB88CCB2E1FE")
    );

    assert_eq!(
        windows::foundation::collections::IMapChangedEventArgs::<A>::IID,
        com::sys::GUID::from("E1AA5138-12BD-51A1-8558-698DFD070ABE")
    );

    assert_eq!(
        windows::foundation::collections::IMapView::<A, B>::IID,
        com::sys::GUID::from("B78F0653-FA89-59CF-BA95-726938AAE666")
    );

    assert_eq!(
        windows::foundation::collections::IMap::<A, B>::IID,
        com::sys::GUID::from("9962CD50-09D5-5C46-B1E1-3C679C1C8FAE")
    );

    assert_eq!(
        windows::foundation::collections::IObservableMap::<A, B>::IID,
        com::sys::GUID::from("75F99E2A-137E-537E-A5B1-0B5A6245FC02")
    );

    assert_eq!(
        windows::foundation::collections::IObservableVector::<A>::IID,
        com::sys::GUID::from("D24C289F-2341-5128-AAA1-292DD0DC1950")
    );

    assert_eq!(
        windows::foundation::collections::IVectorView::<A>::IID,
        com::sys::GUID::from("5F07498B-8E14-556E-9D2E-2E98D5615DA9")
    );

    assert_eq!(
        windows::foundation::collections::IVector::<A>::IID,
        com::sys::GUID::from("0E3F106F-A266-50A1-8043-C90FCF3844F6")
    );

    assert_eq!(
        windows::foundation::collections::MapChangedEventHandler::<A, B>::IID,
        com::sys::GUID::from("19046F0B-CF81-5DEC-BBB2-7CC250DA8B8B")
    );

    assert_eq!(
        windows::foundation::collections::VectorChangedEventHandler::<A>::IID,
        com::sys::GUID::from("A1E9ACD7-E4DF-5A79-AEFA-DE07934AB0FB")
    );

    //
    // Generated primitive GUIDs
    //

    assert_eq!(
        windows::foundation::IReference::<bool>::IID,
        com::sys::GUID::from("3C00FD60-2950-5939-A21A-2D12C5A01B8A")
    );

    assert_eq!(
        windows::foundation::IReference::<i8>::IID,
        com::sys::GUID::from("95500129-FBF6-5AFC-89DF-70642D741990")
    );

    assert_eq!(
        windows::foundation::IReference::<i16>::IID,
        com::sys::GUID::from("6EC9E41B-6709-5647-9918-A1270110FC4E")
    );

    assert_eq!(
        windows::foundation::IReference::<i32>::IID,
        com::sys::GUID::from("548CEFBD-BC8A-5FA0-8DF2-957440FC8BF4")
    );

    assert_eq!(
        windows::foundation::IReference::<i64>::IID,
        com::sys::GUID::from("4DDA9E24-E69F-5C6A-A0A6-93427365AF2A")
    );

    assert_eq!(
        windows::foundation::IReference::<u8>::IID,
        com::sys::GUID::from("e5198cc8-2873-55f5-b0a1-84ff9e4aad62")
    );

    assert_eq!(
        windows::foundation::IReference::<u16>::IID,
        com::sys::GUID::from("5AB7D2C3-6B62-5E71-A4B6-2D49C4F238FD")
    );

    assert_eq!(
        windows::foundation::IReference::<u32>::IID,
        com::sys::GUID::from("513ef3af-e784-5325-a91e-97c2b8111cf3")
    );

    assert_eq!(
        windows::foundation::IReference::<u64>::IID,
        com::sys::GUID::from("6755e376-53bb-568b-a11d-17239868309e")
    );

    assert_eq!(
        windows::foundation::IReference::<f32>::IID,
        com::sys::GUID::from("719CC2BA-3E76-5DEF-9F1A-38D85A145EA8")
    );

    assert_eq!(
        windows::foundation::IReference::<f64>::IID,
        com::sys::GUID::from("2F2D6C29-5473-5F3E-92E7-96572BB990E2")
    );

    assert_eq!(
        windows::foundation::IReference::<com::sys::GUID>::IID,
        com::sys::GUID::from("7D50F649-632C-51F9-849A-EE49428933EA")
    );

    assert_eq!(
        windows::foundation::IReference::<windows::HString>::IID,
        com::sys::GUID::from("FD416DFB-2A07-52EB-AAE3-DFCE14116C05")
    );

    // TODO: etc.

    Ok(())
}
