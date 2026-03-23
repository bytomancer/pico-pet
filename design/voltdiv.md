# Circuit notes

I had assumed having a voltage divider on the system would be a parasitic battery drain.
However, checking impedence of the typical pico board shows a moderate impedence as a base.
Thus, so long as the voltage divider doesn't raise that impedence,
we'll not raise our parasitic draw beyond that baseline.

385k ohm VIN -> gnd w/ no vdiv at all
55k ohm VIN-> gnd w/ straight vdiv of 100k+100k (that's weird?)
It's also 385k ohm if you just put GP22 as the last leg of the vdiv -_-
It's also 385k ohm if you let the vdiv last leg float

There's therefore no real reason to add any mosfet/transistor.

# Measuring voltage

Measurements w/ GP22 set low as the last leg:
4.46V = 2700 (USB w/ LDO backfeed)
4.24V = 2520 (max battery)
3.71V = 2190
3.64V = 2140
3.53V = 2120
3.50V = 2080
3.49V = 2060
3.32V = 1990 (no issues yet)
3.28V = 1970 (screen might be slightly dimmed?)
3.22V = 1920 (same as before, reading falling FAST)
3.13V = 1870 (definitely dimming)
3.06V = 1810 (still brighter than half brightness)I apparently ran
????V = 1740 (died shortly beneath this reading)
2.86V = 1680 (boots then dies within 10 seconds)
1.80V = ???? (does not boot, speaker clicks repeatedly, likely boot cycling)
float = 0630 (reading when GP26 disconnected)

# Battery System
States: 100% 75% 50% 25% charging disconnected

At boot, check the power once, and apply the following rule:
```
If the charging signal is high, state is charging.
Else, one shot vsense check:
2400+ => 100%
2200-2399 => 75%
2000-2199 => 50%
1000-1999 => 25%
<1000 => disconnected
```

Now, every second, re-test the vsense & charge detect.

```
If the charging signal is high, state is charging no matter what.
If the vsense is <1000, go to disconnected no matter what.
Else, proceed with these tests.

From charging or vsense, run the oneshot rules above.

From 100%:
  if vsense < 2300, move to 75%

From 75%:
  if vsense > 2500, move to 100%
  elif vsense < 2100, move to 50%

From 50%:
  if vsense > 2300, move to 75%
  elif vsense < 1900, move to 25%

From 25%:
  if vsense > 2100, move to 50%
```

This way we won't experience any real bouncing.
