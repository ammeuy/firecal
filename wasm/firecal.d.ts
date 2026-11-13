/**
 * The browser contract of `firecal-wasm`: plain exports of `firecal.wasm`, no glue code.
 * Numbers are `u32`; a function that cannot do what was asked returns `INVALID` instead of trapping.
 * Changing this file is a contract change (see docs/engineering.md).
 */

/** Returned by an export that could not do what was asked: `u32::MAX`. */
export const INVALID = 4294967295;

export interface FirecalExports {
  /** The month index of `month` (1 to 12) of `year` (2000 or later), or `INVALID`. */
  fc_month_index(year: number, month: number): number;
  /** The number of months from `first` to `last` inclusive, or `INVALID`. */
  fc_month_span(first: number, last: number): number;
}
