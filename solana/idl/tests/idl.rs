use nautilus_idl::*;

#[test]
fn idl() {
    let (name, version) = parse_cargo_toml("Cargo.toml").unwrap();
    let metadata = IdlMetadata::new("some-program-id");

    let types = vec![IdlType::new(
        "CustomArgs",
        IdlTypeType::new(
            "struct",
            vec![
                IdlTypeTypeField::new("string1", "string"),
                IdlTypeTypeField::new("string2", "string"),
            ],
        ),
    )];

    let accounts = vec![
        IdlAccount::new(
            "Hero",
            IdlTypeType::new(
                "struct",
                vec![
                    IdlTypeTypeField::new("id", "u8"),
                    IdlTypeTypeField::new("name", "string"),
                    IdlTypeTypeField::new("authority", "publicKey"),
                ],
            ),
        ),
        IdlAccount::new(
            "Villain",
            IdlTypeType::new(
                "struct",
                vec![
                    IdlTypeTypeField::new("id", "u8"),
                    IdlTypeTypeField::new("name", "string"),
                    IdlTypeTypeField::new("authority", "publicKey"),
                ],
            ),
        ),
    ];

    let instructions = vec![
        IdlInstruction::new(
            "CreateHero",
            vec![
                IdlInstructionAccount::new(
                    "autoincAccount",
                    true,
                    false,
                    "The autoincrement account.",
                ),
                IdlInstructionAccount::new("newAccount", true, false, "The account to be created."),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
                IdlInstructionAccount::new("systemProgram", false, false, "The System Program"),
            ],
            vec![IdlInstructionArg::new(
                "hero",
                IdlInstructionArgType::new("Hero"),
            )],
            IdlInstructionDiscriminant::new(0),
        ),
        IdlInstruction::new(
            "DeleteHero",
            vec![
                IdlInstructionAccount::new(
                    "targetAccount",
                    true,
                    false,
                    "The account to be deleted.",
                ),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
            ],
            vec![],
            IdlInstructionDiscriminant::new(1),
        ),
        IdlInstruction::new(
            "UpdateHero",
            vec![
                IdlInstructionAccount::new(
                    "targetAccount",
                    true,
                    false,
                    "The account to be updated.",
                ),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
                IdlInstructionAccount::new("systemProgram", false, false, "The System Program"),
            ],
            vec![IdlInstructionArg::new(
                "hero",
                IdlInstructionArgType::new("Hero"),
            )],
            IdlInstructionDiscriminant::new(2),
        ),
        IdlInstruction::new(
            "CreateVillain",
            vec![
                IdlInstructionAccount::new(
                    "autoincAccount",
                    true,
                    false,
                    "The autoincrement account.",
                ),
                IdlInstructionAccount::new("newAccount", true, false, "The account to be created."),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
                IdlInstructionAccount::new("systemProgram", false, false, "The System Program"),
            ],
            vec![IdlInstructionArg::new(
                "villain",
                IdlInstructionArgType::new("Villain"),
            )],
            IdlInstructionDiscriminant::new(3),
        ),
        IdlInstruction::new(
            "DeleteVillain",
            vec![
                IdlInstructionAccount::new(
                    "targetAccount",
                    true,
                    false,
                    "The account to be deleted.",
                ),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
            ],
            vec![],
            IdlInstructionDiscriminant::new(4),
        ),
        IdlInstruction::new(
            "UpdateVillain",
            vec![
                IdlInstructionAccount::new(
                    "targetAccount",
                    true,
                    false,
                    "The account to be updated.",
                ),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
                IdlInstructionAccount::new("systemProgram", false, false, "The System Program"),
            ],
            vec![IdlInstructionArg::new(
                "villain",
                IdlInstructionArgType::new("Villain"),
            )],
            IdlInstructionDiscriminant::new(5),
        ),
        IdlInstruction::new(
            "CustomInstruction",
            vec![
                IdlInstructionAccount::new(
                    "targetAccount",
                    true,
                    false,
                    "The account to be used as a test.",
                ),
                IdlInstructionAccount::new(
                    "authority",
                    true,
                    true,
                    "One of the authorities specified for this account.",
                ),
                IdlInstructionAccount::new("feePayer", true, true, "Fee payer"),
                IdlInstructionAccount::new("systemProgram", false, false, "The System Program"),
            ],
            vec![IdlInstructionArg::new(
                "customArgs",
                IdlInstructionArgType::new("CustomArgs"),
            )],
            IdlInstructionDiscriminant::new(6),
        ),
    ];

    let idl = Idl::new(&version, &name, instructions, accounts, types, metadata);

    idl.write_to_json("./target/idl");
}
