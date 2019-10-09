import { MatchResult } from './MatchResult';

// MatchData is a tuple that represents the structure of match result data from a CSV file.
export type MatchData = [Date, string, string, number, number, MatchResult, string];
