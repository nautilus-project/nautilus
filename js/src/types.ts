import { NautilusTable } from "./sql"

export type NautilusProgram = {
  tables: { [tableName: string]: string }
}

export type NautilusProgramTables =  { [tableName: string]: NautilusTable }

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
}

export type NautilusAccountIdlType = {
  kind: string,
  fields: NautilusIdlTypeField[]
  config?: NautilusAccountIdlConfig
}

export type NautilusAccountIdlConfig = {
  discriminatorStr: string
  authorities: string[]
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