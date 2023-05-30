export type ProgramNautilusType = {
  "version": "0.1.0",
  "name": "program-nautilus",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "nautilus_index",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "nautilus_index"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 0
      }
    },
    {
      "name": "createPerson",
      "accounts": [
        {
          "name": "index",
          "isMut": true,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "new_person",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "new_person"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "authority",
          "type": "publicKey"
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 1
      }
    },
    {
      "name": "readPerson",
      "accounts": [
        {
          "name": "index",
          "isMut": false,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "person",
          "isMut": false,
          "isSigner": false,
          "type": "account",
          "desc": "person"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 2
      }
    },
    {
      "name": "createHome",
      "accounts": [
        {
          "name": "index",
          "isMut": true,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "new_home",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "new_home"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u8"
        },
        {
          "name": "house_number",
          "type": "u8"
        },
        {
          "name": "street",
          "type": "string"
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 3
      }
    },
    {
      "name": "readHome",
      "accounts": [
        {
          "name": "index",
          "isMut": false,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "home",
          "isMut": false,
          "isSigner": false,
          "type": "account",
          "desc": "home"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 4
      }
    },
    {
      "name": "createCar",
      "accounts": [
        {
          "name": "index",
          "isMut": true,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "new_car",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "new_car"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [
        {
          "name": "make",
          "type": "string"
        },
        {
          "name": "model",
          "type": "string"
        },
        {
          "name": "purchase_authority",
          "type": "publicKey"
        },
        {
          "name": "operating_authority",
          "type": "publicKey"
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 5
      }
    },
    {
      "name": "readCar",
      "accounts": [
        {
          "name": "index",
          "isMut": false,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "car",
          "isMut": false,
          "isSigner": false,
          "type": "account",
          "desc": "car"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 6
      }
    }
  ],
  "accounts": [
    {
      "name": "Person",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "authority",
            "type": "publicKey"
          }
        ]
      },
      "config": {
        "tableName": "person",
        "primaryKey": "id",
        "autoincrement": true,
        "authorities": [
          "authority"
        ]
      }
    },
    {
      "name": "Home",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "house_number",
            "type": "u8"
          },
          {
            "name": "street",
            "type": "string"
          }
        ]
      },
      "config": {
        "tableName": "home",
        "primaryKey": "id",
        "autoincrement": false,
        "authorities": []
      }
    },
    {
      "name": "Car",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "make",
            "type": "string"
          },
          {
            "name": "model",
            "type": "string"
          },
          {
            "name": "purchase_authority",
            "type": "publicKey"
          },
          {
            "name": "operating_authority",
            "type": "publicKey"
          }
        ]
      },
      "config": {
        "tableName": "car",
        "primaryKey": "id",
        "autoincrement": true,
        "authorities": [
          "purchase_authority",
          "operating_authority"
        ],
        "defaultInstructions": [
          {
            "Create": "Car"
          },
          {
            "Delete": "Car"
          },
          {
            "Update": "Car"
          }
        ]
      }
    }
  ],
  "metadata": {
    "origin": "nautilus"
  }
}
export const IDL = {
  "version": "0.1.0",
  "name": "program-nautilus",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "nautilus_index",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "nautilus_index"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 0
      }
    },
    {
      "name": "createPerson",
      "accounts": [
        {
          "name": "index",
          "isMut": true,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "new_person",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "new_person"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "authority",
          "type": "publicKey"
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 1
      }
    },
    {
      "name": "readPerson",
      "accounts": [
        {
          "name": "index",
          "isMut": false,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "person",
          "isMut": false,
          "isSigner": false,
          "type": "account",
          "desc": "person"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 2
      }
    },
    {
      "name": "createHome",
      "accounts": [
        {
          "name": "index",
          "isMut": true,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "new_home",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "new_home"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u8"
        },
        {
          "name": "house_number",
          "type": "u8"
        },
        {
          "name": "street",
          "type": "string"
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 3
      }
    },
    {
      "name": "readHome",
      "accounts": [
        {
          "name": "index",
          "isMut": false,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "home",
          "isMut": false,
          "isSigner": false,
          "type": "account",
          "desc": "home"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 4
      }
    },
    {
      "name": "createCar",
      "accounts": [
        {
          "name": "index",
          "isMut": true,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "new_car",
          "isMut": true,
          "isSigner": false,
          "type": "account",
          "desc": "new_car"
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true,
          "type": "feePayer",
          "desc": "The transaction fee payer"
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "type": "sysvar",
          "desc": "The Sysvar: Rent"
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "type": "systemProgram",
          "desc": "The System Program"
        }
      ],
      "args": [
        {
          "name": "make",
          "type": "string"
        },
        {
          "name": "model",
          "type": "string"
        },
        {
          "name": "purchase_authority",
          "type": "publicKey"
        },
        {
          "name": "operating_authority",
          "type": "publicKey"
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 5
      }
    },
    {
      "name": "readCar",
      "accounts": [
        {
          "name": "index",
          "isMut": false,
          "isSigner": false,
          "type": "index",
          "desc": "The Nautilus Index for this program"
        },
        {
          "name": "car",
          "isMut": false,
          "isSigner": false,
          "type": "account",
          "desc": "car"
        }
      ],
      "args": [],
      "discriminant": {
        "type": "u8",
        "value": 6
      }
    }
  ],
  "accounts": [
    {
      "name": "Person",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "authority",
            "type": "publicKey"
          }
        ]
      },
      "config": {
        "tableName": "person",
        "primaryKey": "id",
        "autoincrement": true,
        "authorities": [
          "authority"
        ]
      }
    },
    {
      "name": "Home",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "house_number",
            "type": "u8"
          },
          {
            "name": "street",
            "type": "string"
          }
        ]
      },
      "config": {
        "tableName": "home",
        "primaryKey": "id",
        "autoincrement": false,
        "authorities": []
      }
    },
    {
      "name": "Car",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "make",
            "type": "string"
          },
          {
            "name": "model",
            "type": "string"
          },
          {
            "name": "purchase_authority",
            "type": "publicKey"
          },
          {
            "name": "operating_authority",
            "type": "publicKey"
          }
        ]
      },
      "config": {
        "tableName": "car",
        "primaryKey": "id",
        "autoincrement": true,
        "authorities": [
          "purchase_authority",
          "operating_authority"
        ],
        "defaultInstructions": [
          {
            "Create": "Car"
          },
          {
            "Delete": "Car"
          },
          {
            "Update": "Car"
          }
        ]
      }
    }
  ],
  "metadata": {
    "origin": "nautilus"
  }
}