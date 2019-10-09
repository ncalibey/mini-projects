import { OutputTarget } from '../Summary';

// ConsoleReport reports data to the console.
export class ConsoleReport implements OutputTarget {
  print(report: string): void {
    console.log(report);
  }
}
