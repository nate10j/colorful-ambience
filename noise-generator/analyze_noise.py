import numpy as np
import matplotlib.pyplot as plt
from scipy.signal import welch

def plot_power_spectrum(filename, noise_type='pink'):
    # Read the data from the file
    data = np.loadtxt(filename)

    # Compute the power spectrum using Welch's method
    fs = 1.0  # Sampling frequency (adjust if necessary)
    frequencies, power_spectrum = welch(data, fs, nperseg=1024)

    # Convert power spectrum to dB
    power_spectrum_db = 10 * np.log10(power_spectrum)

    # Create the reference line
    if noise_type == 'pink':
        reference_line = -10 * np.log10(frequencies) - 30
    elif noise_type == 'brown':
        reference_line = -20 * np.log10(frequencies) - 50
    else:
        raise ValueError("Unsupported noise type. Use 'pink' or 'brown'.")

    reference_line[0] = reference_line[1]  # Handle the zero frequency case

    # Adjust the reference line and power spectrum with the offset
    adjusted_power_spectrum_db = power_spectrum_db
    adjusted_reference_line = reference_line

    # Plot the power spectrum
    plt.figure(figsize=(10, 6))
    plt.plot(frequencies, adjusted_power_spectrum_db, label='Power Spectrum')
    plt.plot(frequencies, adjusted_reference_line, label=f'1/f^{2 if noise_type == "brown" else 1} Reference', linestyle='--', color='red')
    plt.title(f'Power Spectrum of {noise_type.capitalize()} Noise')
    plt.xlabel('Frequency (Hz)')
    plt.ylabel('Power (dB)')
    plt.legend()
    plt.grid(True)
    plt.show()

# Example usage:
# For pink noise
plot_power_spectrum('pink_noise_samples.txt', noise_type='pink')

# For brown noise
plot_power_spectrum('brown_noise_samples.txt', noise_type='brown')
