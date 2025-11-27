
# House pricing Kaggle competition


1. Path Reachable?
2. Path exists?
3. Path is a file?
4. File is a csv?
5. Csv is a good csv? (constant column lenght for all rows)
  If not:
  - For each non compliant record(row) save a file with:
    - Row id: which is the position or a particula ID column
    - Number of columns

1. Load data
  - File path is passed as command argument:
    - library used: clap
  - Validation:
    1. Path:
      1.1 Exist or 'path does not exist'
      1.2 Read permission allowed or 'System access error: {specific_error}'
    2. File:
      2.1 Is a file or 
