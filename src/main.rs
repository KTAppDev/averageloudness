use hound::WavReader;

fn main() {
    // Specify the input file path
    let input_path = "output.wav";

    // Read the WAV file
    let mut reader = WavReader::open(input_path).expect("Failed to open input file");

    // Set the reference amplitude for 16-bit audio
    let reference_amplitude = i16::MAX as f64;

    // Set the minimum amplitude to include in the average
    // 4000 kinda matches lufs values in my daw
    let min_amplitude_threshold = 4000.0;

    // Calculate the sum of dB values and the count of samples
    let (sum_db, count) = reader
        .samples::<i16>()
        .fold((0.0, 0), |(sum_db, count), sample| {
            match sample {
                Ok(amplitude) => {
                    let amplitude = amplitude as f64;

                    // Ensure that amplitude is within the valid range
                    let amplitude = amplitude.max(-reference_amplitude).min(reference_amplitude);

                    // Ensure that amplitude is not zero to avoid log(0) issues
                    let amplitude = amplitude.abs().max(f64::EPSILON);

                    // Check if amplitude is above the threshold
                    if amplitude >= min_amplitude_threshold {
                        let db = 20.0 * (amplitude / reference_amplitude).log10();
                        (sum_db + db, count + 1)
                    } else {
                        (sum_db, count)
                    }
                }
                Err(err) => {
                    eprintln!("Error reading sample: {:?}", err);
                    (sum_db, count)
                }
            }
        });

    // Print the sum of dB values
    println!("Sum (dB): {}", sum_db);

    // Calculate the average
    let average_loudness_db = if count > 0 {
        sum_db / count as f64
    } else {
        0.0 // Avoid division by zero if there are no samples
    };

    println!("Average Loudness (dB): {:.2}", average_loudness_db);
}
