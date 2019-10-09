import fs from 'fs';
import { OutputTarget } from '../Summary';

// HtmlReport reports data by generating a small HTML snippet.
export class HtmlReport implements OutputTarget {
  print(report: string): void {
    const html =`
      <div>
        <h1>Analysis Output</h1>
        <div>${report}</div>
      </div>
    `;

    fs.writeFileSync('report.html', html);
  }
}
