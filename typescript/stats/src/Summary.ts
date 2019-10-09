import { MatchData } from './MatchData';
import { WinsAnalysis } from './analyzers/WinsAnalysis';
import { HtmlReport } from './reportTargets/HtmlReport';

// Analyzer is an interface for analyzing match data.
export interface Analyzer {
  run(matches: MatchData[]): string;
}

// OutputTarget is an interface for outputting data to a specific target.
export interface OutputTarget {
  print(report: string): void;
}

// Summary analyzes match data and outputs it to a specified target.
export class Summary {
  static winsAnalysisWithHtmlReport(team: string): Summary {
    const wa = new WinsAnalysis(team);
    const hr = new HtmlReport();
    return new Summary(wa, hr)
  }

  constructor(public analyzer: Analyzer, public outputTarget: OutputTarget) {}

  buildAndPrintReport(matches: MatchData[]): void {
    const output = this.analyzer.run(matches);
    this.outputTarget.print(output);
  }
}
