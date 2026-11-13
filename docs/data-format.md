# Data format

The contracts between the pieces. Proposed on day one; changing one is a contract change (see `AGENTS.md`).
The shared Rust types in `firecal-types` read and write these formats; nothing else does.

## Grid and time

- Equirectangular, **0.25° cells**: 1440 columns × 720 rows. Cell `(row, col)` covers latitude
  `90 - 0.25*(row+1) .. 90 - 0.25*row` and longitude `-180 + 0.25*col .. -180 + 0.25*(col+1)`.
  `CellIndex = row * 1440 + col`.
- Time is **months**: `MonthIndex = (year - 2000) * 12 + (month - 1)`. The series starts at index 10
  (2000-11, first Terra/MODIS month in FIRMS) and runs to the latest complete month. Verify the start against
  the real archive before relying on it.
- Why 0.25°: coarse enough that VIIRS and MODIS counts per cell are comparable and files stay small; fine enough
  for a regional area. Open to change only before the first full pipeline run.

## Satellites

`Satellite` is an enum, stored as a `u8`: `0` Terra, `1` Aqua (MODIS); `2` S-NPP, `3` NOAA-20, `4` NOAA-21
(VIIRS). Kept apart until harmonization.

## Aggregate file `.fca` (ingest → harmonize)

One file per satellite per month: `data/aggregates/<satellite>/<yyyymm>.fca`. Little-endian.

| Offset | Type | Field |
|---|---|---|
| 0 | `u32` | magic `FCA1` |
| 4 | `u8` | satellite |
| 5 | `u8` | reserved, zero |
| 6 | `u16` | month index |
| 8 | `u32` | record count `n` |
| 12 | `n` × 16 bytes | records, sorted by cell |

Record: `cell: u32`, `count: u32` (detections kept), `frp_mw_sum: f32` (fire radiative power, megawatts),
`day_count: u32` (how many of the `count` were daytime).

Kept detections: **nominal and high confidence** for both sensors; low confidence is dropped. Day and night are
both kept. Decision recorded here so it is not argued twice.

## Tiles (harmonize → browser)

The world is cut into **10° × 10° tiles** (40 × 40 cells = 1600), 36 × 18 = 648 possible files. The browser
loads only the tiles an area touches.

```
tiles/index.json              months available, calibration version, planes, tile list, build date
tiles/t_<row10>_<col10>.bin   one tile; row10 and col10 are its row and column in the 18 × 36 tile grid
```

A tile with no hotspots in any month is not written.

### Tile file (little-endian)

| Offset | Type | Field |
|---|---|---|
| 0 | `u32` | magic `FCT1` |
| 4 | `u16` | `month_count` |
| 6 | `u16` | `first_month_index` |
| 8 | `u8` | `plane_count` (3) |
| 9 | 7 bytes | reserved, zero |
| 16 | planes, in order below | each `u16` × `month_count` × 1600, month-major: `[month][cell_row][cell_col]` |

Planes:

1. **count**: harmonized hotspot count, MODIS-equivalent, rounded to the nearest integer, saturated at 65535.
2. **sigma**: uncertainty of `count`, one standard deviation, in counts, rounded up, saturated at 65535.
3. **energy**: harmonized fire radiative power in megawatts, rounded and saturated. Harmonized the same way as
   count, on its own calibration.

Served gzip-compressed by the static host. Dense is deliberate: trivial to read in Rust and wasm, and it
compresses well because most cells are zero. Whether the whole world fits offline is **measured on the first
full run**; if not, the user pins regions.

## Harmonization

Unit: **MODIS-equivalent** hotspots per cell-month. VIIRS (375 m) sees many more small fires than MODIS (1 km),
so its raw counts are higher.

Method (to be validated by the harmonize owner; the figures it produces are also pitch material):

1. Aggregate each sensor to cell-month counts, keeping each satellite apart.
2. Fit a mapping VIIRS → MODIS on the **overlap period** (VIIRS S-NPP starts 2012; MODIS still runs), per 10°
   tile so it can differ between savanna and boreal forest. Start with quantile mapping on cell-month counts.
3. Apply it to VIIRS months after MODIS ends, and wherever MODIS is missing.
4. **Sigma** comes from the calibration's residuals over the overlap years in that tile, widened for months far
   outside the range the calibration saw. Where only MODIS exists, sigma is its own measurement floor.
5. Record the calibration version in `tiles/index.json` and ship the per-tile parameters, so any number on
   screen can be traced back.
6. Check, as a test: over the overlap, harmonized VIIRS vs MODIS monthly totals per tile agree within a
   tolerance written in the test with its reason, not guessed.
7. **Effort proxy (stretch):** a per-cell-month estimate of how much the satellites looked, from satellites in
   operation and their overflight frequency by latitude. Cut first if time runs out. It would be a fourth plane
   and a new format version.

## Series (what the core returns for an area)

For an area (lat/lon rectangle or polygon), a month range and a plane (count or energy):

```
months[n]        MonthIndex
value[n]         harmonized value summed over the area
sigma[n]         uncertainty of value; errors of cells add in quadrature (sqrt, no exp or ln)
median[12]       historical median of value for each calendar month, over the baseline years
anomaly[n]       (value - median[month]) / max(median[month], 1)
unusual[n]       true when |value - median[month]| > 2 * sigma[n]
analogs[k]       the k past years most similar to the season in progress: year, distance
```

Baseline years: all complete years before the current one, unless the UI picks others. Analog distance is the
sum over months of the difference in **rank within the calendar month** across years, so it does not need
logarithms and treats quiet and busy regions alike.

**Stub for the web owners:** until the core exists, `web/*` read `web/stub/series.json`, a hand-made `Series`
in this shape. The wasm call replaces it with no change to the UI.

## CSV export

`cells.csv`: `year,month,cell_lat,cell_lon,value,sigma` for every cell of the area and every month of the
range. `series.csv`: the `Series` above. Both are aggregates, not raw hotspots: raw per-hotspot history is out
of scope for a static site.

**Open question for the team:** recent raw hotspots would need a FIRMS API key, which cannot live in a static
site. Decide Saturday morning; the default is no.
