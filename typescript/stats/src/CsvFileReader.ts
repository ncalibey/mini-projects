import fs from 'fs';

// CSVFileReader implements the DataReader interface for CSV files.
export class CsvFileReader {
  // impl data.
  data: string[][] = [];

  constructor(public filename: string) { }

  // impl read
  read(): void {
    this.data = fs
      .readFileSync(this.filename, { encoding: 'utf-8' })
      .split('\n')
      .map((row: string): string[] => row.split(','));
  }
}
