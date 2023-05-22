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

export type NautilusIdlTypeField = {
  name: string
  type: string
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