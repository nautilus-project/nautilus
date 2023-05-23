import { PublicKey } from "@solana/web3.js"

export type NautilusIdl = {
  version: string
  name: string
  instructions: NautilusInstructionIdl[]
  accounts: NautilusAccountIdl[]
  metadata: { origin: string }
}

export type NautilusInstructionIdl = {
  name: string
  accounts: NautilusInstructionAccountIdl[]
  args: NautilusIdlTypeField[]
  discriminant: {
    type: string,
    value: number
  }
}

export type NautilusInstructionAccountIdl = {
  name: string,
  isMut: boolean
  isSigner: boolean
  type: string
  desc: string
}

export type IdlType =
  | "bool"
  | "u8"
  | "i8"
  | "u16"
  | "i16"
  | "u32"
  | "i32"
  | "f32"
  | "u64"
  | "i64"
  | "f64"
  | "u128"
  | "i128"
  | "u256"
  | "i256"
  | "bytes"
  | "string"
  | "publicKey"
// | IdlTypeDefined
// | IdlTypeOption
// | IdlTypeCOption
// | IdlTypeVec
// | IdlTypeArray;

// User defined type.
export type IdlTypeDefined = {
  defined: string;
};

export type IdlTypeOption = {
  option: IdlType;
};

export type IdlTypeCOption = {
  coption: IdlType;
};

export type IdlTypeVec = {
  vec: IdlType;
};

export type IdlTypeArray = {
  array: [idlType: IdlType, size: number];
};

export type IdlTypeLookup = {
  "bool": boolean
  "u8": number
  "i8": number
  "u16": number
  "i16": number
  "u32": number
  "i32": number
  "f32": number
  "u64": number
  "i64": number
  "f64": number
  "u128": number
  "i128": number
  "u256": number
  "i256": number
  "bytes": Uint8Array
  "string": string
  "publicKey": PublicKey
  // IdlTypeDefined: object
  // IdlTypeOption: object | undefined
  // IdlTypeCOption: object | undefined
  // IdlTypeVec: object[]
  // IdlTypeArray: object[]
}

export type NautilusIdlTypeField = {
  name: string
  type: IdlType
}

export type NautilusAccountIdl = {
  name: string
  type: NautilusAccountIdlType
  config?: NautilusAccountIdlConfig
}

export type NautilusAccountIdlType = {
  kind: string,
  fields: NautilusIdlTypeField[]
}

export type NautilusAccountIdlConfig = {
  authorities: string[]
  tableName?: string
  primaryKey?: string
  autoincrement?: boolean
  discriminatorStr?: string
  seeds?: NautilusAccountIdlConfigSeed[]
}

export type NautilusAccountIdlConfigSeed = {
  lit?: {
    value: string
  }
  field: {
    key: string
  }
}

export type NautilusTableIdl = {
  name: string
  type: NautilusAccountIdlType
  config: NautilusTableConfigIdl
}

export type NautilusTableConfigIdl = {
  tableName: string
  primaryKey: string
  autoincrement: boolean
  authorities: string[]
  defaultInstructions?: NautilusTableIdlInstruction[]
}

export type NautilusTableIdlInstruction = {
  Create?: string
  Update?: string
  Delete?: string
}