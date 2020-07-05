import 'package:moor/moor.dart';
import 'package:moor_flutter/moor_flutter.dart';

// Moor works by source gen. This file will contain all the generated code.
part 'moor_database.g.dart';

// The name of the database table is "tasks".
// By default, the name of the generated data class will be "Task" (sans "s").
class Tasks extends Table {
  // `autoIncrement` automatically sets this to be the primary key.
  IntColumn get id => integer().autoIncrement()();
  // FK to the tags table. We still call `nullable` despite the `customConstraint` overriding it
  // so that the `tagName` field won't be required for the generated `Tag` data class.
  TextColumn get tagName =>
      text().nullable().customConstraint('NULL REFERENCES tags(name)')();
  // If the length constraint is not fulfilled, the `Task` will not
  // be inserted into the database and an exception will be thrown.
  TextColumn get name => text().withLength(min: 1, max: 50)();
  // `DateTime` is not natively supported by SQLite.
  // Moor converts it to & from UNIX seconds.
  DateTimeColumn get dueDate => dateTime().nullable()();
  // `Booleans` are not supported as well, Moor converts them to integers.
  // Simple default values are specified as Constants.
  BoolColumn get completed => boolean().withDefault(Constant(false))();
}

// This annotation tells the code generator which tables this DB works with.
@UseMoor(tables: [Tasks, Tags], daos: [TaskDao, TagDao])
// _$AppDatabase is the name of the generated class.
class AppDatabase extends _$AppDatabase {
  AppDatabase()
      // Specify the location of the database file.
      : super((FlutterQueryExecutor.inDatabaseFolder(
          path: 'db.sqlite',
          // Good for debugging - prints SQL in the console.
          logStatements: true,
        )));

  // Bump this when changing tables and columns.
  // Migrations will be covered in the next part.
  @override
  int get schemaVersion => 2;

  @override
  MigrationStrategy get migration => MigrationStrategy(
        // Runs if the database has already been opened on the device with a lower version.
        onUpgrade: (migrator, from, to) async {
          if (from == 1) {
            await migrator.addColumn(tasks, tasks.tagName);
            await migrator.createTable(tags);
          }
        },
        // Runs after all the migrations but BEFORE any queries have a chance to execute.
        beforeOpen: (db, details) async {
          await db.customStatement('PRAGMA foreign_keys = ON');
        },
      );
}

// Denote which table can use this DAO.
@UseDao(
  tables: [Tasks, Tags],
  queries: {
    // An implementation of this query will be generated inside the _$TaskDaoMixin
    // Both completeTasksGenerated() and watchCompletedTasksGenerated() will be created.
    'completedTasksGenerated':
        'SELECT * FROM tasks WHERE completed = 1 ORDER BY due_date DESC, name;'
  },
)
class TaskDao extends DatabaseAccessor<AppDatabase> with _$TaskDaoMixin {
  final AppDatabase db;

  TaskDao(this.db) : super(db);

  // All tables have getters in the generated class - we can select the tasks table.
  Future<List<Task>> getAllTasks() => select(tasks).get();
  // Moor supports Streams which emit elements when the watched data changes.
  // Updated to use ORDER BY.
  Stream<List<TaskWithTag>> watchAllTasks() {
    return (select(tasks)
          // Statements like `orderBy` and `where` return void => the need to use a cascading ".." operator
          ..orderBy(
            ([
              // Primary sorting by due date.
              (t) =>
                  OrderingTerm(expression: t.dueDate, mode: OrderingMode.asc),
              // Secondary alphabetical sorting.
              (t) => OrderingTerm(expression: t.name),
            ]),
          ))
        // As opposed to orderBy or where, join returns a value. This is what we want to watch/get.
        .join(
          [
            // Join all the tasks with their tags.
            // It's important that we use equalsExp and not just equals.
            // This way, we can join using all tag names in the tasks table, not just a specific one.
            leftOuterJoin(tags, tags.name.equalsExp(tasks.tagName)),
          ],
        )
        // watch the whole SELECT statement.
        .watch()
        .map(
          (rows) => rows
              .map((row) => TaskWithTag(
                  task: row.readTable(tasks), tag: row.readTable(tags)))
              .toList(),
        );
  }

  Stream<List<Task>> watchCompletedTasks() {
    // `where` returns `void`, need to use the cascading operator.
    return (select(tasks)
          ..orderBy([
            (t) => OrderingTerm(expression: t.dueDate),
            (t) => OrderingTerm(expression: t.name),
          ])
          ..where((t) => t.completed.equals(true)))
        .watch();
  }

  // Watching complete tasks with a custom query.
  Stream<List<Task>> watchCompletedTasksCustom() {
    return customSelectStream(
      'SELECT * FROM tasks WHERE completed = 1 ORDER BY due_date DESC, name;',
      readsFrom: {tasks},
    ).map((rows) => rows.map((row) => Task.fromData(row.data, db)).toList());
  }

  Future insertTask(Insertable<Task> task) => into(tasks).insert(task);
  // Updates a Task with a matching primary key.
  Future updateTask(Insertable<Task> task) => update(tasks).replace(task);
  Future deleteTask(Insertable<Task> task) => delete(tasks).delete(task);
}

class Tags extends Table {
  TextColumn get name => text().withLength(min: 1, max: 10)();
  IntColumn get color => integer()();

  // Making name as the primary key of a tag requires names to be UNIQUE.
  @override
  Set<Column> get primaryKey => {name};
}

@UseDao(tables: [Tags])
class TagDao extends DatabaseAccessor<AppDatabase> with _$TagDaoMixin {
  final AppDatabase db;

  TagDao(this.db) : super(db);

  Stream<List<Tag>> watchTags() => select(tags).watch();
  Future insertTag(Insertable<Tag> tag) => into(tags).insert(tag);
}

// We have to group tasks with tags manually. This class will be used for the table join.
class TaskWithTag {
  final Task task;
  final Tag tag;

  TaskWithTag({
    @required this.task,
    @required this.tag,
  });
}
