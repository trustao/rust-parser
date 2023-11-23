#![allow(nonstandard_style)]
// Generated from SparkSql.g4 by ANTLR 4.8
use crate::tree::ParseTreeListener;
use super::sparksqlparser::*;

pub trait SparkSqlListener<'input> : ParseTreeListener<'input,SparkSqlParserContextType>{
/**
 * Enter a parse tree produced by {@link SparkSqlParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#singleStatement}.
 * @param ctx the parse tree
 */
fn enter_singleStatement(&mut self, _ctx: &SingleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#singleStatement}.
 * @param ctx the parse tree
 */
fn exit_singleStatement(&mut self, _ctx: &SingleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#emptyStatement}.
 * @param ctx the parse tree
 */
fn enter_emptyStatement(&mut self, _ctx: &EmptyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#emptyStatement}.
 * @param ctx the parse tree
 */
fn exit_emptyStatement(&mut self, _ctx: &EmptyStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#singleExpression}.
 * @param ctx the parse tree
 */
fn enter_singleExpression(&mut self, _ctx: &SingleExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#singleExpression}.
 * @param ctx the parse tree
 */
fn exit_singleExpression(&mut self, _ctx: &SingleExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#singleTableIdentifier}.
 * @param ctx the parse tree
 */
fn enter_singleTableIdentifier(&mut self, _ctx: &SingleTableIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#singleTableIdentifier}.
 * @param ctx the parse tree
 */
fn exit_singleTableIdentifier(&mut self, _ctx: &SingleTableIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#singleMultipartIdentifier}.
 * @param ctx the parse tree
 */
fn enter_singleMultipartIdentifier(&mut self, _ctx: &SingleMultipartIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#singleMultipartIdentifier}.
 * @param ctx the parse tree
 */
fn exit_singleMultipartIdentifier(&mut self, _ctx: &SingleMultipartIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#singleDataType}.
 * @param ctx the parse tree
 */
fn enter_singleDataType(&mut self, _ctx: &SingleDataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#singleDataType}.
 * @param ctx the parse tree
 */
fn exit_singleDataType(&mut self, _ctx: &SingleDataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#singleTableSchema}.
 * @param ctx the parse tree
 */
fn enter_singleTableSchema(&mut self, _ctx: &SingleTableSchemaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#singleTableSchema}.
 * @param ctx the parse tree
 */
fn exit_singleTableSchema(&mut self, _ctx: &SingleTableSchemaContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code statementDefault}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statementDefault(&mut self, _ctx: &StatementDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code statementDefault}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statementDefault(&mut self, _ctx: &StatementDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dmlStatement}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dmlStatement(&mut self, _ctx: &DmlStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dmlStatement}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dmlStatement(&mut self, _ctx: &DmlStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code use}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_use(&mut self, _ctx: &UseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code use}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_use(&mut self, _ctx: &UseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createNamespace(&mut self, _ctx: &CreateNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createNamespace(&mut self, _ctx: &CreateNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setNamespaceProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setNamespaceProperties(&mut self, _ctx: &SetNamespacePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setNamespaceProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setNamespaceProperties(&mut self, _ctx: &SetNamespacePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setNamespaceLocation}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setNamespaceLocation(&mut self, _ctx: &SetNamespaceLocationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setNamespaceLocation}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setNamespaceLocation(&mut self, _ctx: &SetNamespaceLocationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropNamespace(&mut self, _ctx: &DropNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropNamespace(&mut self, _ctx: &DropNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showNamespaces}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showNamespaces(&mut self, _ctx: &ShowNamespacesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showNamespaces}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showNamespaces(&mut self, _ctx: &ShowNamespacesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createTable(&mut self, _ctx: &CreateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createTable(&mut self, _ctx: &CreateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createHiveTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createHiveTable(&mut self, _ctx: &CreateHiveTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createHiveTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createHiveTable(&mut self, _ctx: &CreateHiveTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createTableLike}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createTableLike(&mut self, _ctx: &CreateTableLikeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createTableLike}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createTableLike(&mut self, _ctx: &CreateTableLikeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code replaceTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_replaceTable(&mut self, _ctx: &ReplaceTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code replaceTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_replaceTable(&mut self, _ctx: &ReplaceTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code analyze}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_analyze(&mut self, _ctx: &AnalyzeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code analyze}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_analyze(&mut self, _ctx: &AnalyzeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code addTableColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_addTableColumns(&mut self, _ctx: &AddTableColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code addTableColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_addTableColumns(&mut self, _ctx: &AddTableColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code renameTableColumn}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_renameTableColumn(&mut self, _ctx: &RenameTableColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code renameTableColumn}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_renameTableColumn(&mut self, _ctx: &RenameTableColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTableColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTableColumns(&mut self, _ctx: &DropTableColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTableColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTableColumns(&mut self, _ctx: &DropTableColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code renameTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_renameTable(&mut self, _ctx: &RenameTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code renameTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_renameTable(&mut self, _ctx: &RenameTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTableProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTableProperties(&mut self, _ctx: &SetTablePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTableProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTableProperties(&mut self, _ctx: &SetTablePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unsetTableProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_unsetTableProperties(&mut self, _ctx: &UnsetTablePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unsetTableProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_unsetTableProperties(&mut self, _ctx: &UnsetTablePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterTableAlterColumn}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterTableAlterColumn(&mut self, _ctx: &AlterTableAlterColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterTableAlterColumn}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterTableAlterColumn(&mut self, _ctx: &AlterTableAlterColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code hiveChangeColumn}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_hiveChangeColumn(&mut self, _ctx: &HiveChangeColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code hiveChangeColumn}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_hiveChangeColumn(&mut self, _ctx: &HiveChangeColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code hiveReplaceColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_hiveReplaceColumns(&mut self, _ctx: &HiveReplaceColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code hiveReplaceColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_hiveReplaceColumns(&mut self, _ctx: &HiveReplaceColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTableSerDe}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTableSerDe(&mut self, _ctx: &SetTableSerDeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTableSerDe}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTableSerDe(&mut self, _ctx: &SetTableSerDeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code addTablePartition}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_addTablePartition(&mut self, _ctx: &AddTablePartitionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code addTablePartition}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_addTablePartition(&mut self, _ctx: &AddTablePartitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code renameTablePartition}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_renameTablePartition(&mut self, _ctx: &RenameTablePartitionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code renameTablePartition}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_renameTablePartition(&mut self, _ctx: &RenameTablePartitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTablePartitions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTablePartitions(&mut self, _ctx: &DropTablePartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTablePartitions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTablePartitions(&mut self, _ctx: &DropTablePartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTableLocation}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTableLocation(&mut self, _ctx: &SetTableLocationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTableLocation}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTableLocation(&mut self, _ctx: &SetTableLocationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code recoverPartitions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_recoverPartitions(&mut self, _ctx: &RecoverPartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code recoverPartitions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_recoverPartitions(&mut self, _ctx: &RecoverPartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTable(&mut self, _ctx: &DropTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTable(&mut self, _ctx: &DropTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropView}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropView(&mut self, _ctx: &DropViewContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropView}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropView(&mut self, _ctx: &DropViewContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createView}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createView(&mut self, _ctx: &CreateViewContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createView}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createView(&mut self, _ctx: &CreateViewContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createTempViewUsing}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createTempViewUsing(&mut self, _ctx: &CreateTempViewUsingContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createTempViewUsing}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createTempViewUsing(&mut self, _ctx: &CreateTempViewUsingContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterViewQuery}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterViewQuery(&mut self, _ctx: &AlterViewQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterViewQuery}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterViewQuery(&mut self, _ctx: &AlterViewQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createFunction(&mut self, _ctx: &CreateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createFunction(&mut self, _ctx: &CreateFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropFunction(&mut self, _ctx: &DropFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropFunction(&mut self, _ctx: &DropFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code explain}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_explain(&mut self, _ctx: &ExplainContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code explain}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_explain(&mut self, _ctx: &ExplainContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showTables}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showTables(&mut self, _ctx: &ShowTablesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showTables}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showTables(&mut self, _ctx: &ShowTablesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showTable(&mut self, _ctx: &ShowTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showTable(&mut self, _ctx: &ShowTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showTblProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showTblProperties(&mut self, _ctx: &ShowTblPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showTblProperties}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showTblProperties(&mut self, _ctx: &ShowTblPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showColumns(&mut self, _ctx: &ShowColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showColumns}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showColumns(&mut self, _ctx: &ShowColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showViews}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showViews(&mut self, _ctx: &ShowViewsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showViews}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showViews(&mut self, _ctx: &ShowViewsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showPartitions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showPartitions(&mut self, _ctx: &ShowPartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showPartitions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showPartitions(&mut self, _ctx: &ShowPartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showFunctions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showFunctions(&mut self, _ctx: &ShowFunctionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showFunctions}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showFunctions(&mut self, _ctx: &ShowFunctionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showCreateTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showCreateTable(&mut self, _ctx: &ShowCreateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showCreateTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showCreateTable(&mut self, _ctx: &ShowCreateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showCurrentNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showCurrentNamespace(&mut self, _ctx: &ShowCurrentNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showCurrentNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showCurrentNamespace(&mut self, _ctx: &ShowCurrentNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeFunction(&mut self, _ctx: &DescribeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeFunction(&mut self, _ctx: &DescribeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeNamespace(&mut self, _ctx: &DescribeNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeNamespace(&mut self, _ctx: &DescribeNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeRelation}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeRelation(&mut self, _ctx: &DescribeRelationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeRelation}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeRelation(&mut self, _ctx: &DescribeRelationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeQuery}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeQuery(&mut self, _ctx: &DescribeQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeQuery}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeQuery(&mut self, _ctx: &DescribeQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code commentNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_commentNamespace(&mut self, _ctx: &CommentNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code commentNamespace}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_commentNamespace(&mut self, _ctx: &CommentNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code commentTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_commentTable(&mut self, _ctx: &CommentTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code commentTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_commentTable(&mut self, _ctx: &CommentTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code refreshTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_refreshTable(&mut self, _ctx: &RefreshTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code refreshTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_refreshTable(&mut self, _ctx: &RefreshTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code refreshFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_refreshFunction(&mut self, _ctx: &RefreshFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code refreshFunction}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_refreshFunction(&mut self, _ctx: &RefreshFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code refreshResource}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_refreshResource(&mut self, _ctx: &RefreshResourceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code refreshResource}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_refreshResource(&mut self, _ctx: &RefreshResourceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code cacheTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_cacheTable(&mut self, _ctx: &CacheTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code cacheTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_cacheTable(&mut self, _ctx: &CacheTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code uncacheTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_uncacheTable(&mut self, _ctx: &UncacheTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code uncacheTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_uncacheTable(&mut self, _ctx: &UncacheTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code clearCache}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_clearCache(&mut self, _ctx: &ClearCacheContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code clearCache}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_clearCache(&mut self, _ctx: &ClearCacheContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code loadData}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_loadData(&mut self, _ctx: &LoadDataContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code loadData}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_loadData(&mut self, _ctx: &LoadDataContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code truncateTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_truncateTable(&mut self, _ctx: &TruncateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code truncateTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_truncateTable(&mut self, _ctx: &TruncateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code repairTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_repairTable(&mut self, _ctx: &RepairTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code repairTable}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_repairTable(&mut self, _ctx: &RepairTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code manageResource}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_manageResource(&mut self, _ctx: &ManageResourceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code manageResource}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_manageResource(&mut self, _ctx: &ManageResourceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code failNativeCommand}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_failNativeCommand(&mut self, _ctx: &FailNativeCommandContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code failNativeCommand}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_failNativeCommand(&mut self, _ctx: &FailNativeCommandContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTimeZone}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTimeZone(&mut self, _ctx: &SetTimeZoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTimeZone}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTimeZone(&mut self, _ctx: &SetTimeZoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setQuotedConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setQuotedConfiguration(&mut self, _ctx: &SetQuotedConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setQuotedConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setQuotedConfiguration(&mut self, _ctx: &SetQuotedConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setConfiguration(&mut self, _ctx: &SetConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setConfiguration(&mut self, _ctx: &SetConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code resetQuotedConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_resetQuotedConfiguration(&mut self, _ctx: &ResetQuotedConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code resetQuotedConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_resetQuotedConfiguration(&mut self, _ctx: &ResetQuotedConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code resetConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_resetConfiguration(&mut self, _ctx: &ResetConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code resetConfiguration}
 * labeled alternative in {@link SparkSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_resetConfiguration(&mut self, _ctx: &ResetConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#configKey}.
 * @param ctx the parse tree
 */
fn enter_configKey(&mut self, _ctx: &ConfigKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#configKey}.
 * @param ctx the parse tree
 */
fn exit_configKey(&mut self, _ctx: &ConfigKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#unsupportedHiveNativeCommands}.
 * @param ctx the parse tree
 */
fn enter_unsupportedHiveNativeCommands(&mut self, _ctx: &UnsupportedHiveNativeCommandsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#unsupportedHiveNativeCommands}.
 * @param ctx the parse tree
 */
fn exit_unsupportedHiveNativeCommands(&mut self, _ctx: &UnsupportedHiveNativeCommandsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#createTableHeader}.
 * @param ctx the parse tree
 */
fn enter_createTableHeader(&mut self, _ctx: &CreateTableHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#createTableHeader}.
 * @param ctx the parse tree
 */
fn exit_createTableHeader(&mut self, _ctx: &CreateTableHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#replaceTableHeader}.
 * @param ctx the parse tree
 */
fn enter_replaceTableHeader(&mut self, _ctx: &ReplaceTableHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#replaceTableHeader}.
 * @param ctx the parse tree
 */
fn exit_replaceTableHeader(&mut self, _ctx: &ReplaceTableHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#bucketSpec}.
 * @param ctx the parse tree
 */
fn enter_bucketSpec(&mut self, _ctx: &BucketSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#bucketSpec}.
 * @param ctx the parse tree
 */
fn exit_bucketSpec(&mut self, _ctx: &BucketSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#skewSpec}.
 * @param ctx the parse tree
 */
fn enter_skewSpec(&mut self, _ctx: &SkewSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#skewSpec}.
 * @param ctx the parse tree
 */
fn exit_skewSpec(&mut self, _ctx: &SkewSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#locationSpec}.
 * @param ctx the parse tree
 */
fn enter_locationSpec(&mut self, _ctx: &LocationSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#locationSpec}.
 * @param ctx the parse tree
 */
fn exit_locationSpec(&mut self, _ctx: &LocationSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#commentSpec}.
 * @param ctx the parse tree
 */
fn enter_commentSpec(&mut self, _ctx: &CommentSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#commentSpec}.
 * @param ctx the parse tree
 */
fn exit_commentSpec(&mut self, _ctx: &CommentSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#query}.
 * @param ctx the parse tree
 */
fn enter_query(&mut self, _ctx: &QueryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#query}.
 * @param ctx the parse tree
 */
fn exit_query(&mut self, _ctx: &QueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertOverwriteTable}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertOverwriteTable(&mut self, _ctx: &InsertOverwriteTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertOverwriteTable}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertOverwriteTable(&mut self, _ctx: &InsertOverwriteTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertIntoTable}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertIntoTable(&mut self, _ctx: &InsertIntoTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertIntoTable}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertIntoTable(&mut self, _ctx: &InsertIntoTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertOverwriteHiveDir}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertOverwriteHiveDir(&mut self, _ctx: &InsertOverwriteHiveDirContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertOverwriteHiveDir}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertOverwriteHiveDir(&mut self, _ctx: &InsertOverwriteHiveDirContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertOverwriteDir}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertOverwriteDir(&mut self, _ctx: &InsertOverwriteDirContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertOverwriteDir}
 * labeled alternative in {@link SparkSqlParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertOverwriteDir(&mut self, _ctx: &InsertOverwriteDirContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#partitionSpecLocation}.
 * @param ctx the parse tree
 */
fn enter_partitionSpecLocation(&mut self, _ctx: &PartitionSpecLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#partitionSpecLocation}.
 * @param ctx the parse tree
 */
fn exit_partitionSpecLocation(&mut self, _ctx: &PartitionSpecLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#partitionSpec}.
 * @param ctx the parse tree
 */
fn enter_partitionSpec(&mut self, _ctx: &PartitionSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#partitionSpec}.
 * @param ctx the parse tree
 */
fn exit_partitionSpec(&mut self, _ctx: &PartitionSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#partitionVal}.
 * @param ctx the parse tree
 */
fn enter_partitionVal(&mut self, _ctx: &PartitionValContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#partitionVal}.
 * @param ctx the parse tree
 */
fn exit_partitionVal(&mut self, _ctx: &PartitionValContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#namespace}.
 * @param ctx the parse tree
 */
fn enter_namespace(&mut self, _ctx: &NamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#namespace}.
 * @param ctx the parse tree
 */
fn exit_namespace(&mut self, _ctx: &NamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#describeFuncName}.
 * @param ctx the parse tree
 */
fn enter_describeFuncName(&mut self, _ctx: &DescribeFuncNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#describeFuncName}.
 * @param ctx the parse tree
 */
fn exit_describeFuncName(&mut self, _ctx: &DescribeFuncNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#describeColName}.
 * @param ctx the parse tree
 */
fn enter_describeColName(&mut self, _ctx: &DescribeColNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#describeColName}.
 * @param ctx the parse tree
 */
fn exit_describeColName(&mut self, _ctx: &DescribeColNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#ctes}.
 * @param ctx the parse tree
 */
fn enter_ctes(&mut self, _ctx: &CtesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#ctes}.
 * @param ctx the parse tree
 */
fn exit_ctes(&mut self, _ctx: &CtesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#namedQuery}.
 * @param ctx the parse tree
 */
fn enter_namedQuery(&mut self, _ctx: &NamedQueryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#namedQuery}.
 * @param ctx the parse tree
 */
fn exit_namedQuery(&mut self, _ctx: &NamedQueryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tableProvider}.
 * @param ctx the parse tree
 */
fn enter_tableProvider(&mut self, _ctx: &TableProviderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tableProvider}.
 * @param ctx the parse tree
 */
fn exit_tableProvider(&mut self, _ctx: &TableProviderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#createTableClauses}.
 * @param ctx the parse tree
 */
fn enter_createTableClauses(&mut self, _ctx: &CreateTableClausesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#createTableClauses}.
 * @param ctx the parse tree
 */
fn exit_createTableClauses(&mut self, _ctx: &CreateTableClausesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tablePropertyList}.
 * @param ctx the parse tree
 */
fn enter_tablePropertyList(&mut self, _ctx: &TablePropertyListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tablePropertyList}.
 * @param ctx the parse tree
 */
fn exit_tablePropertyList(&mut self, _ctx: &TablePropertyListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tableProperty}.
 * @param ctx the parse tree
 */
fn enter_tableProperty(&mut self, _ctx: &TablePropertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tableProperty}.
 * @param ctx the parse tree
 */
fn exit_tableProperty(&mut self, _ctx: &TablePropertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tablePropertyKey}.
 * @param ctx the parse tree
 */
fn enter_tablePropertyKey(&mut self, _ctx: &TablePropertyKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tablePropertyKey}.
 * @param ctx the parse tree
 */
fn exit_tablePropertyKey(&mut self, _ctx: &TablePropertyKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tablePropertyValue}.
 * @param ctx the parse tree
 */
fn enter_tablePropertyValue(&mut self, _ctx: &TablePropertyValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tablePropertyValue}.
 * @param ctx the parse tree
 */
fn exit_tablePropertyValue(&mut self, _ctx: &TablePropertyValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#constantList}.
 * @param ctx the parse tree
 */
fn enter_constantList(&mut self, _ctx: &ConstantListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#constantList}.
 * @param ctx the parse tree
 */
fn exit_constantList(&mut self, _ctx: &ConstantListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#nestedConstantList}.
 * @param ctx the parse tree
 */
fn enter_nestedConstantList(&mut self, _ctx: &NestedConstantListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#nestedConstantList}.
 * @param ctx the parse tree
 */
fn exit_nestedConstantList(&mut self, _ctx: &NestedConstantListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#createFileFormat}.
 * @param ctx the parse tree
 */
fn enter_createFileFormat(&mut self, _ctx: &CreateFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#createFileFormat}.
 * @param ctx the parse tree
 */
fn exit_createFileFormat(&mut self, _ctx: &CreateFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tableFileFormat}
 * labeled alternative in {@link SparkSqlParser#fileFormat}.
 * @param ctx the parse tree
 */
fn enter_tableFileFormat(&mut self, _ctx: &TableFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tableFileFormat}
 * labeled alternative in {@link SparkSqlParser#fileFormat}.
 * @param ctx the parse tree
 */
fn exit_tableFileFormat(&mut self, _ctx: &TableFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code genericFileFormat}
 * labeled alternative in {@link SparkSqlParser#fileFormat}.
 * @param ctx the parse tree
 */
fn enter_genericFileFormat(&mut self, _ctx: &GenericFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code genericFileFormat}
 * labeled alternative in {@link SparkSqlParser#fileFormat}.
 * @param ctx the parse tree
 */
fn exit_genericFileFormat(&mut self, _ctx: &GenericFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#storageHandler}.
 * @param ctx the parse tree
 */
fn enter_storageHandler(&mut self, _ctx: &StorageHandlerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#storageHandler}.
 * @param ctx the parse tree
 */
fn exit_storageHandler(&mut self, _ctx: &StorageHandlerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#resource}.
 * @param ctx the parse tree
 */
fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#resource}.
 * @param ctx the parse tree
 */
fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleInsertQuery}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_singleInsertQuery(&mut self, _ctx: &SingleInsertQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleInsertQuery}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_singleInsertQuery(&mut self, _ctx: &SingleInsertQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiInsertQuery}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_multiInsertQuery(&mut self, _ctx: &MultiInsertQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiInsertQuery}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_multiInsertQuery(&mut self, _ctx: &MultiInsertQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code deleteFromTable}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_deleteFromTable(&mut self, _ctx: &DeleteFromTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code deleteFromTable}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_deleteFromTable(&mut self, _ctx: &DeleteFromTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code updateTable}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_updateTable(&mut self, _ctx: &UpdateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code updateTable}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_updateTable(&mut self, _ctx: &UpdateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code mergeIntoTable}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_mergeIntoTable(&mut self, _ctx: &MergeIntoTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code mergeIntoTable}
 * labeled alternative in {@link SparkSqlParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_mergeIntoTable(&mut self, _ctx: &MergeIntoTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#queryOrganization}.
 * @param ctx the parse tree
 */
fn enter_queryOrganization(&mut self, _ctx: &QueryOrganizationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#queryOrganization}.
 * @param ctx the parse tree
 */
fn exit_queryOrganization(&mut self, _ctx: &QueryOrganizationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#multiInsertQueryBody}.
 * @param ctx the parse tree
 */
fn enter_multiInsertQueryBody(&mut self, _ctx: &MultiInsertQueryBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#multiInsertQueryBody}.
 * @param ctx the parse tree
 */
fn exit_multiInsertQueryBody(&mut self, _ctx: &MultiInsertQueryBodyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code queryTermDefault}
 * labeled alternative in {@link SparkSqlParser#queryTerm}.
 * @param ctx the parse tree
 */
fn enter_queryTermDefault(&mut self, _ctx: &QueryTermDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code queryTermDefault}
 * labeled alternative in {@link SparkSqlParser#queryTerm}.
 * @param ctx the parse tree
 */
fn exit_queryTermDefault(&mut self, _ctx: &QueryTermDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setOperation}
 * labeled alternative in {@link SparkSqlParser#queryTerm}.
 * @param ctx the parse tree
 */
fn enter_setOperation(&mut self, _ctx: &SetOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setOperation}
 * labeled alternative in {@link SparkSqlParser#queryTerm}.
 * @param ctx the parse tree
 */
fn exit_setOperation(&mut self, _ctx: &SetOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code queryPrimaryDefault}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_queryPrimaryDefault(&mut self, _ctx: &QueryPrimaryDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code queryPrimaryDefault}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_queryPrimaryDefault(&mut self, _ctx: &QueryPrimaryDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fromStmt}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_fromStmt(&mut self, _ctx: &FromStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fromStmt}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_fromStmt(&mut self, _ctx: &FromStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code table}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code table}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineTableDefault1}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_inlineTableDefault1(&mut self, _ctx: &InlineTableDefault1Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineTableDefault1}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_inlineTableDefault1(&mut self, _ctx: &InlineTableDefault1Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code subquery}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_subquery(&mut self, _ctx: &SubqueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subquery}
 * labeled alternative in {@link SparkSqlParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_subquery(&mut self, _ctx: &SubqueryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#sortItem}.
 * @param ctx the parse tree
 */
fn enter_sortItem(&mut self, _ctx: &SortItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#sortItem}.
 * @param ctx the parse tree
 */
fn exit_sortItem(&mut self, _ctx: &SortItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#fromStatement}.
 * @param ctx the parse tree
 */
fn enter_fromStatement(&mut self, _ctx: &FromStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#fromStatement}.
 * @param ctx the parse tree
 */
fn exit_fromStatement(&mut self, _ctx: &FromStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#fromStatementBody}.
 * @param ctx the parse tree
 */
fn enter_fromStatementBody(&mut self, _ctx: &FromStatementBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#fromStatementBody}.
 * @param ctx the parse tree
 */
fn exit_fromStatementBody(&mut self, _ctx: &FromStatementBodyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code transformQuerySpecification}
 * labeled alternative in {@link SparkSqlParser#querySpecification}.
 * @param ctx the parse tree
 */
fn enter_transformQuerySpecification(&mut self, _ctx: &TransformQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code transformQuerySpecification}
 * labeled alternative in {@link SparkSqlParser#querySpecification}.
 * @param ctx the parse tree
 */
fn exit_transformQuerySpecification(&mut self, _ctx: &TransformQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code regularQuerySpecification}
 * labeled alternative in {@link SparkSqlParser#querySpecification}.
 * @param ctx the parse tree
 */
fn enter_regularQuerySpecification(&mut self, _ctx: &RegularQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code regularQuerySpecification}
 * labeled alternative in {@link SparkSqlParser#querySpecification}.
 * @param ctx the parse tree
 */
fn exit_regularQuerySpecification(&mut self, _ctx: &RegularQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#transformClause}.
 * @param ctx the parse tree
 */
fn enter_transformClause(&mut self, _ctx: &TransformClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#transformClause}.
 * @param ctx the parse tree
 */
fn exit_transformClause(&mut self, _ctx: &TransformClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#selectClause}.
 * @param ctx the parse tree
 */
fn enter_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#selectClause}.
 * @param ctx the parse tree
 */
fn exit_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#setClause}.
 * @param ctx the parse tree
 */
fn enter_setClause(&mut self, _ctx: &SetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#setClause}.
 * @param ctx the parse tree
 */
fn exit_setClause(&mut self, _ctx: &SetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#matchedClause}.
 * @param ctx the parse tree
 */
fn enter_matchedClause(&mut self, _ctx: &MatchedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#matchedClause}.
 * @param ctx the parse tree
 */
fn exit_matchedClause(&mut self, _ctx: &MatchedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#notMatchedClause}.
 * @param ctx the parse tree
 */
fn enter_notMatchedClause(&mut self, _ctx: &NotMatchedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#notMatchedClause}.
 * @param ctx the parse tree
 */
fn exit_notMatchedClause(&mut self, _ctx: &NotMatchedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#matchedAction}.
 * @param ctx the parse tree
 */
fn enter_matchedAction(&mut self, _ctx: &MatchedActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#matchedAction}.
 * @param ctx the parse tree
 */
fn exit_matchedAction(&mut self, _ctx: &MatchedActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#notMatchedAction}.
 * @param ctx the parse tree
 */
fn enter_notMatchedAction(&mut self, _ctx: &NotMatchedActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#notMatchedAction}.
 * @param ctx the parse tree
 */
fn exit_notMatchedAction(&mut self, _ctx: &NotMatchedActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#assignmentList}.
 * @param ctx the parse tree
 */
fn enter_assignmentList(&mut self, _ctx: &AssignmentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#assignmentList}.
 * @param ctx the parse tree
 */
fn exit_assignmentList(&mut self, _ctx: &AssignmentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#whereClause}.
 * @param ctx the parse tree
 */
fn enter_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#whereClause}.
 * @param ctx the parse tree
 */
fn exit_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#havingClause}.
 * @param ctx the parse tree
 */
fn enter_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#havingClause}.
 * @param ctx the parse tree
 */
fn exit_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#hint}.
 * @param ctx the parse tree
 */
fn enter_hint(&mut self, _ctx: &HintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#hint}.
 * @param ctx the parse tree
 */
fn exit_hint(&mut self, _ctx: &HintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#hintStatement}.
 * @param ctx the parse tree
 */
fn enter_hintStatement(&mut self, _ctx: &HintStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#hintStatement}.
 * @param ctx the parse tree
 */
fn exit_hintStatement(&mut self, _ctx: &HintStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#fromClause}.
 * @param ctx the parse tree
 */
fn enter_fromClause(&mut self, _ctx: &FromClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#fromClause}.
 * @param ctx the parse tree
 */
fn exit_fromClause(&mut self, _ctx: &FromClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#aggregationClause}.
 * @param ctx the parse tree
 */
fn enter_aggregationClause(&mut self, _ctx: &AggregationClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#aggregationClause}.
 * @param ctx the parse tree
 */
fn exit_aggregationClause(&mut self, _ctx: &AggregationClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#groupingSet}.
 * @param ctx the parse tree
 */
fn enter_groupingSet(&mut self, _ctx: &GroupingSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#groupingSet}.
 * @param ctx the parse tree
 */
fn exit_groupingSet(&mut self, _ctx: &GroupingSetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#pivotClause}.
 * @param ctx the parse tree
 */
fn enter_pivotClause(&mut self, _ctx: &PivotClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#pivotClause}.
 * @param ctx the parse tree
 */
fn exit_pivotClause(&mut self, _ctx: &PivotClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#pivotColumn}.
 * @param ctx the parse tree
 */
fn enter_pivotColumn(&mut self, _ctx: &PivotColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#pivotColumn}.
 * @param ctx the parse tree
 */
fn exit_pivotColumn(&mut self, _ctx: &PivotColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#pivotValue}.
 * @param ctx the parse tree
 */
fn enter_pivotValue(&mut self, _ctx: &PivotValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#pivotValue}.
 * @param ctx the parse tree
 */
fn exit_pivotValue(&mut self, _ctx: &PivotValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#lateralView}.
 * @param ctx the parse tree
 */
fn enter_lateralView(&mut self, _ctx: &LateralViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#lateralView}.
 * @param ctx the parse tree
 */
fn exit_lateralView(&mut self, _ctx: &LateralViewContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#setQuantifier}.
 * @param ctx the parse tree
 */
fn enter_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#setQuantifier}.
 * @param ctx the parse tree
 */
fn exit_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#relation}.
 * @param ctx the parse tree
 */
fn enter_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#relation}.
 * @param ctx the parse tree
 */
fn exit_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#joinRelation}.
 * @param ctx the parse tree
 */
fn enter_joinRelation(&mut self, _ctx: &JoinRelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#joinRelation}.
 * @param ctx the parse tree
 */
fn exit_joinRelation(&mut self, _ctx: &JoinRelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#joinType}.
 * @param ctx the parse tree
 */
fn enter_joinType(&mut self, _ctx: &JoinTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#joinType}.
 * @param ctx the parse tree
 */
fn exit_joinType(&mut self, _ctx: &JoinTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#joinCriteria}.
 * @param ctx the parse tree
 */
fn enter_joinCriteria(&mut self, _ctx: &JoinCriteriaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#joinCriteria}.
 * @param ctx the parse tree
 */
fn exit_joinCriteria(&mut self, _ctx: &JoinCriteriaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#sample}.
 * @param ctx the parse tree
 */
fn enter_sample(&mut self, _ctx: &SampleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#sample}.
 * @param ctx the parse tree
 */
fn exit_sample(&mut self, _ctx: &SampleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByPercentile}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByPercentile(&mut self, _ctx: &SampleByPercentileContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByPercentile}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByPercentile(&mut self, _ctx: &SampleByPercentileContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByRows}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByRows(&mut self, _ctx: &SampleByRowsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByRows}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByRows(&mut self, _ctx: &SampleByRowsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByBucket}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByBucket(&mut self, _ctx: &SampleByBucketContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByBucket}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByBucket(&mut self, _ctx: &SampleByBucketContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByBytes}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByBytes(&mut self, _ctx: &SampleByBytesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByBytes}
 * labeled alternative in {@link SparkSqlParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByBytes(&mut self, _ctx: &SampleByBytesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#identifierSeq}.
 * @param ctx the parse tree
 */
fn enter_identifierSeq(&mut self, _ctx: &IdentifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#identifierSeq}.
 * @param ctx the parse tree
 */
fn exit_identifierSeq(&mut self, _ctx: &IdentifierSeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#orderedIdentifierList}.
 * @param ctx the parse tree
 */
fn enter_orderedIdentifierList(&mut self, _ctx: &OrderedIdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#orderedIdentifierList}.
 * @param ctx the parse tree
 */
fn exit_orderedIdentifierList(&mut self, _ctx: &OrderedIdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#orderedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_orderedIdentifier(&mut self, _ctx: &OrderedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#orderedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_orderedIdentifier(&mut self, _ctx: &OrderedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#identifierCommentList}.
 * @param ctx the parse tree
 */
fn enter_identifierCommentList(&mut self, _ctx: &IdentifierCommentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#identifierCommentList}.
 * @param ctx the parse tree
 */
fn exit_identifierCommentList(&mut self, _ctx: &IdentifierCommentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#identifierComment}.
 * @param ctx the parse tree
 */
fn enter_identifierComment(&mut self, _ctx: &IdentifierCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#identifierComment}.
 * @param ctx the parse tree
 */
fn exit_identifierComment(&mut self, _ctx: &IdentifierCommentContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tableName}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tableName}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code aliasedQuery}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_aliasedQuery(&mut self, _ctx: &AliasedQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code aliasedQuery}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_aliasedQuery(&mut self, _ctx: &AliasedQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code aliasedRelation}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_aliasedRelation(&mut self, _ctx: &AliasedRelationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code aliasedRelation}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_aliasedRelation(&mut self, _ctx: &AliasedRelationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineTableDefault2}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_inlineTableDefault2(&mut self, _ctx: &InlineTableDefault2Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineTableDefault2}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_inlineTableDefault2(&mut self, _ctx: &InlineTableDefault2Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code tableValuedFunction}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_tableValuedFunction(&mut self, _ctx: &TableValuedFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tableValuedFunction}
 * labeled alternative in {@link SparkSqlParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_tableValuedFunction(&mut self, _ctx: &TableValuedFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#inlineTable}.
 * @param ctx the parse tree
 */
fn enter_inlineTable(&mut self, _ctx: &InlineTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#inlineTable}.
 * @param ctx the parse tree
 */
fn exit_inlineTable(&mut self, _ctx: &InlineTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#functionTable}.
 * @param ctx the parse tree
 */
fn enter_functionTable(&mut self, _ctx: &FunctionTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#functionTable}.
 * @param ctx the parse tree
 */
fn exit_functionTable(&mut self, _ctx: &FunctionTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tableAlias}.
 * @param ctx the parse tree
 */
fn enter_tableAlias(&mut self, _ctx: &TableAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tableAlias}.
 * @param ctx the parse tree
 */
fn exit_tableAlias(&mut self, _ctx: &TableAliasContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rowFormatSerde}
 * labeled alternative in {@link SparkSqlParser#rowFormat}.
 * @param ctx the parse tree
 */
fn enter_rowFormatSerde(&mut self, _ctx: &RowFormatSerdeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rowFormatSerde}
 * labeled alternative in {@link SparkSqlParser#rowFormat}.
 * @param ctx the parse tree
 */
fn exit_rowFormatSerde(&mut self, _ctx: &RowFormatSerdeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rowFormatDelimited}
 * labeled alternative in {@link SparkSqlParser#rowFormat}.
 * @param ctx the parse tree
 */
fn enter_rowFormatDelimited(&mut self, _ctx: &RowFormatDelimitedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rowFormatDelimited}
 * labeled alternative in {@link SparkSqlParser#rowFormat}.
 * @param ctx the parse tree
 */
fn exit_rowFormatDelimited(&mut self, _ctx: &RowFormatDelimitedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#multipartIdentifierList}.
 * @param ctx the parse tree
 */
fn enter_multipartIdentifierList(&mut self, _ctx: &MultipartIdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#multipartIdentifierList}.
 * @param ctx the parse tree
 */
fn exit_multipartIdentifierList(&mut self, _ctx: &MultipartIdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#multipartIdentifier}.
 * @param ctx the parse tree
 */
fn enter_multipartIdentifier(&mut self, _ctx: &MultipartIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#multipartIdentifier}.
 * @param ctx the parse tree
 */
fn exit_multipartIdentifier(&mut self, _ctx: &MultipartIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#tableIdentifier}.
 * @param ctx the parse tree
 */
fn enter_tableIdentifier(&mut self, _ctx: &TableIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#tableIdentifier}.
 * @param ctx the parse tree
 */
fn exit_tableIdentifier(&mut self, _ctx: &TableIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#namedExpression}.
 * @param ctx the parse tree
 */
fn enter_namedExpression(&mut self, _ctx: &NamedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#namedExpression}.
 * @param ctx the parse tree
 */
fn exit_namedExpression(&mut self, _ctx: &NamedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#namedExpressionSeq}.
 * @param ctx the parse tree
 */
fn enter_namedExpressionSeq(&mut self, _ctx: &NamedExpressionSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#namedExpressionSeq}.
 * @param ctx the parse tree
 */
fn exit_namedExpressionSeq(&mut self, _ctx: &NamedExpressionSeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#transformList}.
 * @param ctx the parse tree
 */
fn enter_transformList(&mut self, _ctx: &TransformListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#transformList}.
 * @param ctx the parse tree
 */
fn exit_transformList(&mut self, _ctx: &TransformListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code identityTransform}
 * labeled alternative in {@link SparkSqlParser#transform}.
 * @param ctx the parse tree
 */
fn enter_identityTransform(&mut self, _ctx: &IdentityTransformContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code identityTransform}
 * labeled alternative in {@link SparkSqlParser#transform}.
 * @param ctx the parse tree
 */
fn exit_identityTransform(&mut self, _ctx: &IdentityTransformContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code applyTransform}
 * labeled alternative in {@link SparkSqlParser#transform}.
 * @param ctx the parse tree
 */
fn enter_applyTransform(&mut self, _ctx: &ApplyTransformContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code applyTransform}
 * labeled alternative in {@link SparkSqlParser#transform}.
 * @param ctx the parse tree
 */
fn exit_applyTransform(&mut self, _ctx: &ApplyTransformContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#transformArgument}.
 * @param ctx the parse tree
 */
fn enter_transformArgument(&mut self, _ctx: &TransformArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#transformArgument}.
 * @param ctx the parse tree
 */
fn exit_transformArgument(&mut self, _ctx: &TransformArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalNot}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalNot(&mut self, _ctx: &LogicalNotContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalNot}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalNot(&mut self, _ctx: &LogicalNotContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code predicated}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_predicated(&mut self, _ctx: &PredicatedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code predicated}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_predicated(&mut self, _ctx: &PredicatedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code exists}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_exists(&mut self, _ctx: &ExistsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code exists}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_exists(&mut self, _ctx: &ExistsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalBinary}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalBinary(&mut self, _ctx: &LogicalBinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalBinary}
 * labeled alternative in {@link SparkSqlParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalBinary(&mut self, _ctx: &LogicalBinaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueExpressionDefault}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_valueExpressionDefault(&mut self, _ctx: &ValueExpressionDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueExpressionDefault}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_valueExpressionDefault(&mut self, _ctx: &ValueExpressionDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code comparison}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code comparison}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code arithmeticBinary}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_arithmeticBinary(&mut self, _ctx: &ArithmeticBinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arithmeticBinary}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_arithmeticBinary(&mut self, _ctx: &ArithmeticBinaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code arithmeticUnary}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_arithmeticUnary(&mut self, _ctx: &ArithmeticUnaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arithmeticUnary}
 * labeled alternative in {@link SparkSqlParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_arithmeticUnary(&mut self, _ctx: &ArithmeticUnaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code struct}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_struct(&mut self, _ctx: &StructContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code struct}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_struct(&mut self, _ctx: &StructContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dereference}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_dereference(&mut self, _ctx: &DereferenceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dereference}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_dereference(&mut self, _ctx: &DereferenceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code simpleCase}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_simpleCase(&mut self, _ctx: &SimpleCaseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code simpleCase}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_simpleCase(&mut self, _ctx: &SimpleCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code columnReference}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_columnReference(&mut self, _ctx: &ColumnReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code columnReference}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_columnReference(&mut self, _ctx: &ColumnReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rowConstructor}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_rowConstructor(&mut self, _ctx: &RowConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rowConstructor}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_rowConstructor(&mut self, _ctx: &RowConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code last}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_last(&mut self, _ctx: &LastContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code last}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_last(&mut self, _ctx: &LastContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code star}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_star(&mut self, _ctx: &StarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code star}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_star(&mut self, _ctx: &StarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code overlay}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_overlay(&mut self, _ctx: &OverlayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code overlay}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_overlay(&mut self, _ctx: &OverlayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code subscript}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_subscript(&mut self, _ctx: &SubscriptContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subscript}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_subscript(&mut self, _ctx: &SubscriptContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code subqueryExpression}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_subqueryExpression(&mut self, _ctx: &SubqueryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subqueryExpression}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_subqueryExpression(&mut self, _ctx: &SubqueryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code substring}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_substring(&mut self, _ctx: &SubstringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code substring}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_substring(&mut self, _ctx: &SubstringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code currentDatetime}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_currentDatetime(&mut self, _ctx: &CurrentDatetimeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code currentDatetime}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_currentDatetime(&mut self, _ctx: &CurrentDatetimeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code cast}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_cast(&mut self, _ctx: &CastContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code cast}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_cast(&mut self, _ctx: &CastContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code constantDefault}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_constantDefault(&mut self, _ctx: &ConstantDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code constantDefault}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_constantDefault(&mut self, _ctx: &ConstantDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code lambda}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_lambda(&mut self, _ctx: &LambdaContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code lambda}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_lambda(&mut self, _ctx: &LambdaContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parenthesizedExpression}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_parenthesizedExpression(&mut self, _ctx: &ParenthesizedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenthesizedExpression}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_parenthesizedExpression(&mut self, _ctx: &ParenthesizedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code extract}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_extract(&mut self, _ctx: &ExtractContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code extract}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_extract(&mut self, _ctx: &ExtractContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code trim}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_trim(&mut self, _ctx: &TrimContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code trim}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_trim(&mut self, _ctx: &TrimContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code functionCall}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code functionCall}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code searchedCase}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_searchedCase(&mut self, _ctx: &SearchedCaseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code searchedCase}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_searchedCase(&mut self, _ctx: &SearchedCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code position}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_position(&mut self, _ctx: &PositionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code position}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_position(&mut self, _ctx: &PositionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code first}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_first(&mut self, _ctx: &FirstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code first}
 * labeled alternative in {@link SparkSqlParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_first(&mut self, _ctx: &FirstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nullLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nullLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intervalLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_intervalLiteral(&mut self, _ctx: &IntervalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intervalLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_intervalLiteral(&mut self, _ctx: &IntervalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code typeConstructor}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_typeConstructor(&mut self, _ctx: &TypeConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code typeConstructor}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_typeConstructor(&mut self, _ctx: &TypeConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numericLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numericLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code booleanLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code booleanLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringLiteral}
 * labeled alternative in {@link SparkSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#comparisonOperator}.
 * @param ctx the parse tree
 */
fn enter_comparisonOperator(&mut self, _ctx: &ComparisonOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#comparisonOperator}.
 * @param ctx the parse tree
 */
fn exit_comparisonOperator(&mut self, _ctx: &ComparisonOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#arithmeticOperator}.
 * @param ctx the parse tree
 */
fn enter_arithmeticOperator(&mut self, _ctx: &ArithmeticOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#arithmeticOperator}.
 * @param ctx the parse tree
 */
fn exit_arithmeticOperator(&mut self, _ctx: &ArithmeticOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#predicateOperator}.
 * @param ctx the parse tree
 */
fn enter_predicateOperator(&mut self, _ctx: &PredicateOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#predicateOperator}.
 * @param ctx the parse tree
 */
fn exit_predicateOperator(&mut self, _ctx: &PredicateOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#booleanValue}.
 * @param ctx the parse tree
 */
fn enter_booleanValue(&mut self, _ctx: &BooleanValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#booleanValue}.
 * @param ctx the parse tree
 */
fn exit_booleanValue(&mut self, _ctx: &BooleanValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#interval}.
 * @param ctx the parse tree
 */
fn enter_interval(&mut self, _ctx: &IntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#interval}.
 * @param ctx the parse tree
 */
fn exit_interval(&mut self, _ctx: &IntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#errorCapturingMultiUnitsInterval}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingMultiUnitsInterval(&mut self, _ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#errorCapturingMultiUnitsInterval}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingMultiUnitsInterval(&mut self, _ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#multiUnitsInterval}.
 * @param ctx the parse tree
 */
fn enter_multiUnitsInterval(&mut self, _ctx: &MultiUnitsIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#multiUnitsInterval}.
 * @param ctx the parse tree
 */
fn exit_multiUnitsInterval(&mut self, _ctx: &MultiUnitsIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#errorCapturingUnitToUnitInterval}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingUnitToUnitInterval(&mut self, _ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#errorCapturingUnitToUnitInterval}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingUnitToUnitInterval(&mut self, _ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#unitToUnitInterval}.
 * @param ctx the parse tree
 */
fn enter_unitToUnitInterval(&mut self, _ctx: &UnitToUnitIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#unitToUnitInterval}.
 * @param ctx the parse tree
 */
fn exit_unitToUnitInterval(&mut self, _ctx: &UnitToUnitIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#intervalValue}.
 * @param ctx the parse tree
 */
fn enter_intervalValue(&mut self, _ctx: &IntervalValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#intervalValue}.
 * @param ctx the parse tree
 */
fn exit_intervalValue(&mut self, _ctx: &IntervalValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#colPosition}.
 * @param ctx the parse tree
 */
fn enter_colPosition(&mut self, _ctx: &ColPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#colPosition}.
 * @param ctx the parse tree
 */
fn exit_colPosition(&mut self, _ctx: &ColPositionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code complexDataType}
 * labeled alternative in {@link SparkSqlParser#dataType}.
 * @param ctx the parse tree
 */
fn enter_complexDataType(&mut self, _ctx: &ComplexDataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code complexDataType}
 * labeled alternative in {@link SparkSqlParser#dataType}.
 * @param ctx the parse tree
 */
fn exit_complexDataType(&mut self, _ctx: &ComplexDataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code primitiveDataType}
 * labeled alternative in {@link SparkSqlParser#dataType}.
 * @param ctx the parse tree
 */
fn enter_primitiveDataType(&mut self, _ctx: &PrimitiveDataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code primitiveDataType}
 * labeled alternative in {@link SparkSqlParser#dataType}.
 * @param ctx the parse tree
 */
fn exit_primitiveDataType(&mut self, _ctx: &PrimitiveDataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#qualifiedColTypeWithPositionList}.
 * @param ctx the parse tree
 */
fn enter_qualifiedColTypeWithPositionList(&mut self, _ctx: &QualifiedColTypeWithPositionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#qualifiedColTypeWithPositionList}.
 * @param ctx the parse tree
 */
fn exit_qualifiedColTypeWithPositionList(&mut self, _ctx: &QualifiedColTypeWithPositionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#qualifiedColTypeWithPosition}.
 * @param ctx the parse tree
 */
fn enter_qualifiedColTypeWithPosition(&mut self, _ctx: &QualifiedColTypeWithPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#qualifiedColTypeWithPosition}.
 * @param ctx the parse tree
 */
fn exit_qualifiedColTypeWithPosition(&mut self, _ctx: &QualifiedColTypeWithPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#colTypeList}.
 * @param ctx the parse tree
 */
fn enter_colTypeList(&mut self, _ctx: &ColTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#colTypeList}.
 * @param ctx the parse tree
 */
fn exit_colTypeList(&mut self, _ctx: &ColTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#colType}.
 * @param ctx the parse tree
 */
fn enter_colType(&mut self, _ctx: &ColTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#colType}.
 * @param ctx the parse tree
 */
fn exit_colType(&mut self, _ctx: &ColTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#complexColTypeList}.
 * @param ctx the parse tree
 */
fn enter_complexColTypeList(&mut self, _ctx: &ComplexColTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#complexColTypeList}.
 * @param ctx the parse tree
 */
fn exit_complexColTypeList(&mut self, _ctx: &ComplexColTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#complexColType}.
 * @param ctx the parse tree
 */
fn enter_complexColType(&mut self, _ctx: &ComplexColTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#complexColType}.
 * @param ctx the parse tree
 */
fn exit_complexColType(&mut self, _ctx: &ComplexColTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#whenClause}.
 * @param ctx the parse tree
 */
fn enter_whenClause(&mut self, _ctx: &WhenClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#whenClause}.
 * @param ctx the parse tree
 */
fn exit_whenClause(&mut self, _ctx: &WhenClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#windowClause}.
 * @param ctx the parse tree
 */
fn enter_windowClause(&mut self, _ctx: &WindowClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#windowClause}.
 * @param ctx the parse tree
 */
fn exit_windowClause(&mut self, _ctx: &WindowClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#namedWindow}.
 * @param ctx the parse tree
 */
fn enter_namedWindow(&mut self, _ctx: &NamedWindowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#namedWindow}.
 * @param ctx the parse tree
 */
fn exit_namedWindow(&mut self, _ctx: &NamedWindowContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code windowRef}
 * labeled alternative in {@link SparkSqlParser#windowSpec}.
 * @param ctx the parse tree
 */
fn enter_windowRef(&mut self, _ctx: &WindowRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code windowRef}
 * labeled alternative in {@link SparkSqlParser#windowSpec}.
 * @param ctx the parse tree
 */
fn exit_windowRef(&mut self, _ctx: &WindowRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code windowDef}
 * labeled alternative in {@link SparkSqlParser#windowSpec}.
 * @param ctx the parse tree
 */
fn enter_windowDef(&mut self, _ctx: &WindowDefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code windowDef}
 * labeled alternative in {@link SparkSqlParser#windowSpec}.
 * @param ctx the parse tree
 */
fn exit_windowDef(&mut self, _ctx: &WindowDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#windowFrame}.
 * @param ctx the parse tree
 */
fn enter_windowFrame(&mut self, _ctx: &WindowFrameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#windowFrame}.
 * @param ctx the parse tree
 */
fn exit_windowFrame(&mut self, _ctx: &WindowFrameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#frameBound}.
 * @param ctx the parse tree
 */
fn enter_frameBound(&mut self, _ctx: &FrameBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#frameBound}.
 * @param ctx the parse tree
 */
fn exit_frameBound(&mut self, _ctx: &FrameBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#qualifiedNameList}.
 * @param ctx the parse tree
 */
fn enter_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#qualifiedNameList}.
 * @param ctx the parse tree
 */
fn exit_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn enter_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#errorCapturingIdentifier}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingIdentifier(&mut self, _ctx: &ErrorCapturingIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#errorCapturingIdentifier}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingIdentifier(&mut self, _ctx: &ErrorCapturingIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code errorIdent}
 * labeled alternative in {@link SparkSqlParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn enter_errorIdent(&mut self, _ctx: &ErrorIdentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code errorIdent}
 * labeled alternative in {@link SparkSqlParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn exit_errorIdent(&mut self, _ctx: &ErrorIdentContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code realIdent}
 * labeled alternative in {@link SparkSqlParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn enter_realIdent(&mut self, _ctx: &RealIdentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code realIdent}
 * labeled alternative in {@link SparkSqlParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn exit_realIdent(&mut self, _ctx: &RealIdentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unquotedIdentifier}
 * labeled alternative in {@link SparkSqlParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_unquotedIdentifier(&mut self, _ctx: &UnquotedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unquotedIdentifier}
 * labeled alternative in {@link SparkSqlParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_unquotedIdentifier(&mut self, _ctx: &UnquotedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code quotedIdentifierAlternative}
 * labeled alternative in {@link SparkSqlParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_quotedIdentifierAlternative(&mut self, _ctx: &QuotedIdentifierAlternativeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code quotedIdentifierAlternative}
 * labeled alternative in {@link SparkSqlParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_quotedIdentifierAlternative(&mut self, _ctx: &QuotedIdentifierAlternativeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#quotedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_quotedIdentifier(&mut self, _ctx: &QuotedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#quotedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_quotedIdentifier(&mut self, _ctx: &QuotedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code exponentLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_exponentLiteral(&mut self, _ctx: &ExponentLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code exponentLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_exponentLiteral(&mut self, _ctx: &ExponentLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code decimalLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_decimalLiteral(&mut self, _ctx: &DecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code decimalLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_decimalLiteral(&mut self, _ctx: &DecimalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code legacyDecimalLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_legacyDecimalLiteral(&mut self, _ctx: &LegacyDecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code legacyDecimalLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_legacyDecimalLiteral(&mut self, _ctx: &LegacyDecimalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code integerLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code integerLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bigIntLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_bigIntLiteral(&mut self, _ctx: &BigIntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bigIntLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_bigIntLiteral(&mut self, _ctx: &BigIntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code smallIntLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_smallIntLiteral(&mut self, _ctx: &SmallIntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code smallIntLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_smallIntLiteral(&mut self, _ctx: &SmallIntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tinyIntLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_tinyIntLiteral(&mut self, _ctx: &TinyIntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tinyIntLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_tinyIntLiteral(&mut self, _ctx: &TinyIntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code doubleLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_doubleLiteral(&mut self, _ctx: &DoubleLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code doubleLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_doubleLiteral(&mut self, _ctx: &DoubleLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code floatLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code floatLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bigDecimalLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn enter_bigDecimalLiteral(&mut self, _ctx: &BigDecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bigDecimalLiteral}
 * labeled alternative in {@link SparkSqlParser#number}.
 * @param ctx the parse tree
 */
fn exit_bigDecimalLiteral(&mut self, _ctx: &BigDecimalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#alterColumnAction}.
 * @param ctx the parse tree
 */
fn enter_alterColumnAction(&mut self, _ctx: &AlterColumnActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#alterColumnAction}.
 * @param ctx the parse tree
 */
fn exit_alterColumnAction(&mut self, _ctx: &AlterColumnActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#ansiNonReserved}.
 * @param ctx the parse tree
 */
fn enter_ansiNonReserved(&mut self, _ctx: &AnsiNonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#ansiNonReserved}.
 * @param ctx the parse tree
 */
fn exit_ansiNonReserved(&mut self, _ctx: &AnsiNonReservedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#strictNonReserved}.
 * @param ctx the parse tree
 */
fn enter_strictNonReserved(&mut self, _ctx: &StrictNonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#strictNonReserved}.
 * @param ctx the parse tree
 */
fn exit_strictNonReserved(&mut self, _ctx: &StrictNonReservedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SparkSqlParser#nonReserved}.
 * @param ctx the parse tree
 */
fn enter_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SparkSqlParser#nonReserved}.
 * @param ctx the parse tree
 */
fn exit_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }

}

crate::coerce_from!{ 'input : SparkSqlListener<'input> }


