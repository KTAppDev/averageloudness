# Audio Loudness Analyzer

This simple command-line tool analyzes the loudness of an audio file in WAV format. It utilizes the [hound](https://docs.rs/hound/) crate for reading WAV files.

## Analysis Method

The analyzer calculates the average loudness in decibels (dB) based on the provided WAV file. The process involves:

1. Setting the reference amplitude for 16-bit audio.
2. Specifying a minimum amplitude threshold to include in the average loudness calculation (default: 4000.0).
3. Iterating through the audio samples, filtering out samples below the threshold.
4. Computing the dB value for each qualifying sample and accumulating the sum of dB values.
5\w. Calculating the average loudness in dB.

## Example Output

```bash
Sum (dB): -106622810.71366544
Average Loudness (dB): -11.46
