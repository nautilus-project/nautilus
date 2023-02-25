import NodeSQLParser, { From } from 'node-sql-parser';
import { 
    AST,
    Delete,
    Dual,
    Insert_Replace,
    Select,
    Update,
} from 'node-sql-parser';
import { Nautilus } from '..';
import { NautilusTable } from './table';

const SUPPORTED_ACTIONS = [
    "select",
    "insert",
    "delete",
    "update",
]

export class NautilusQuery {

    nautilus: Nautilus;
    nautilusTables: NautilusTable[] = [];
    ast: AST | AST[];

    constructor(
        nautilus: Nautilus,
        query: string,
    ) {
        this.nautilus = nautilus
        const parser = new NodeSQLParser.Parser()
        const ast = parser.astify(query)
        this.ast = ast
        const addTable =(astObj: AST) => {
            if (
                astObj.type &&
                SUPPORTED_ACTIONS.includes(astObj.type)
            ) {
                if (astObj.type === "delete") this.nautilusTables.push(
                    buildDeleteOperation(nautilus, astObj)
                )
                if (astObj.type === "insert") this.nautilusTables.push(
                    buildInsertOperation(nautilus, astObj)
                )
                if (astObj.type === "select") this.nautilusTables.push(
                    buildSelectOperation(nautilus, astObj)
                )
                if (astObj.type === "update") this.nautilusTables.push(
                    buildUpdateOperation(nautilus, astObj)
                )
            } else {
                unsupportedSqlError()
            }
        }
        if (Array.isArray(ast)) {
            ast.forEach((astObj) => addTable(astObj))
        } else {
            addTable(ast)
        }
    }

    dumpSql(): string {
        return new NodeSQLParser.Parser().sqlify(this.ast)
    }
}

interface TableIdlConfig {
    primaryKey: string,
    autoincrement: boolean,
    fields: string[],
    fieldsWithoutPrimaryKey: string[],
}

// TODO: Get this information from the IDL
function getIdlConfigsForTable(tableName: string): TableIdlConfig {
    const primaryKey = "id"
    const autoincrement = true
    const fields = ["id", "name", "authority"]
    const fieldsWithoutPrimaryKey = fields.filter((f) => f != primaryKey)
    return {
        primaryKey,
        autoincrement,
        fields,
        fieldsWithoutPrimaryKey,
    }
}

function buildData(tableName: string, columns: string[] | null, values: any[][]): any[] {
    const tableConfig = getIdlConfigsForTable(tableName)
    const data: any[] = []
    if (tableConfig.autoincrement && columns && columns.includes(tableConfig.primaryKey)) autoincrementBreachError()
    for (const val of values) {
        if (tableConfig.fields.length == val.length) autoincrementBreachError()
        const entries: any[][] = []
        val.forEach((v, i) => entries.push([tableConfig.fieldsWithoutPrimaryKey[i], v]))
        data.push(Object.fromEntries(entries))
    }
    return data
}

// TODO: Does not support "OR" yet 
function parseWhere(statement: any): [string, string, string][] {
    const where: [string, string, string][] = []
    if (statement.operator === "AND") {
        where.concat(parseWhere(statement.left)) 
        where.concat(parseWhere(statement.right)) 
    }
    else { 
        where.push([statement.left.column, statement.operator, statement.right.value]) 
    }
    return where
}

function buildDeleteOperation(nautilus: Nautilus, ast: Delete): NautilusTable {
    if (ast.table && Array.isArray(ast.table)) { 
        const tableName = ast.table[0].table
        const table = new NautilusTable(nautilus, tableName)
        if (ast.where) parseWhere(ast.where).forEach((w) => table.where(w[0], w[1], w[2]))
        return table.delete()
    } else {
        return sqlMissingError("source table")
    }
}

function buildInsertOperation(nautilus: Nautilus, ast: Insert_Replace): NautilusTable {
    if (ast.table && Array.isArray(ast.table)) { 
        const tableName = ast.table[0].table
        const columns = ast.columns
        const values: any[][] = ast.values.map((valueObj) => valueObj.value.map((v) => v.value))
        const data = buildData(tableName, columns, values)
        return new NautilusTable(nautilus, tableName).create(data)
    } else {
        return sqlMissingError("source table")
    }
}

function buildSelectOperation(nautilus: Nautilus, ast: Select): NautilusTable {
    if (ast.from && Array.isArray(ast.from)) { 
        const tableName = ast.from[0].table
        const returnFields = ast.columns == "*" ? 
            undefined
            : 
            ast.columns.map((c) => c.expr.column)
        const table = new NautilusTable(nautilus, tableName)
        if (returnFields) table.fields(returnFields)
        if (ast.where) parseWhere(ast.where).forEach((w) => table.where(w[0], w[1], w[2]))
        if (ast.orderby) ast.orderby.forEach((o) => table.orderBy(o.expr.column, o.type))
        return table
    } else {
        return sqlMissingError("source table")
    }
}

function buildUpdateOperation(nautilus: Nautilus, ast: Update): NautilusTable {
    if (ast.table && Array.isArray(ast.table)) { 
        if (!(ast.table[0] as From)) unsupportedSqlError()
        const tableName = (ast.table[0] as From).table
        const table = new NautilusTable(nautilus, tableName)
        if (ast.where) parseWhere(ast.where).forEach((w) => table.where(w[0], w[1], w[2]))
        const tableConfig = getIdlConfigsForTable(tableName)
        const data = Object.fromEntries(ast.set.map((s) => {
            if (s.column == tableConfig.primaryKey) immutablePrimaryKeyError()
            return [s.column, s.value.value]
        }))
        return table.update(data)
    } else {
        return sqlMissingError("source table")
    }
}

const unsupportedSqlError = () => {
    throw Error("SQL operation error: Operation not supported")
} 

const sqlMissingError = (missing: string) => {
    throw Error(`SQL operation error: SQL missing the following information: ${missing}`)
} 

const autoincrementBreachError = () => {
    throw Error("You cannot provide a value for the primary key if autoincrement is enabled")
}

const immutablePrimaryKeyError = () => {
    throw Error("You cannot change a primary key with an UPDATE operation")
}