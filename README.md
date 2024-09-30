# prepare_dump_for_local

Prepares a PostgreSQL dump file (text format) for restoration by commenting out lines that could cause errors while restoring the database.

## Motive

In some cases, PostgreSQL database dump files create from cloud databases also contain lines that grant access or set ownership to users that don't exist in local environments. While this can be resolved by simply commenting out those lines in a text editor, some text editors and IDEs can be slow when saving large text files (can take several minutes in VS Code).
This tool can perform said tasks in a matter of seconds.
