import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import linregress

def analyze_noise(file_path, title):
    # Load the samples
    samples = np.loadtxt(file_path)

    # Perform FFT
    fft_result = np.fft.fft(samples)
    fft_freqs = np.fft.fftfreq(len(samples), d=1.0)  # Assuming a sampling rate of 1 Hz for simplicity
    psd = np.abs(fft_result) ** 2  # Power spectral density

    # Only take the positive half of the frequencies
    positive_freqs = fft_freqs[:len(fft_freqs)//2]
    positive_psd = psd[:len(psd)//2]

    # Logarithmic binning for better visualization
    num_bins = 50
    log_bins = np.logspace(np.log10(positive_freqs[1]), np.log10(positive_freqs[-1]), num_bins)
    bin_indices = np.digitize(positive_freqs, log_bins)

    # Compute average PSD for each bin
    binned_psd = np.array([positive_psd[bin_indices == i].mean() for i in range(1, len(log_bins))])
    binned_freqs = log_bins[1:]

    # Remove any NaN values from the binning process
    valid_indices = ~np.isnan(binned_psd)
    binned_psd = binned_psd[valid_indices]
    binned_freqs = binned_freqs[valid_indices]

    # Log-log plot of the PSD
    plt.figure()
    plt.loglog(binned_freqs, binned_psd, marker='o', linestyle='none', label='PSD')

    # Add a reference line for 1/f
    ref_freqs = np.linspace(binned_freqs[0], binned_freqs[-1], 100)
    ref_line = 1 / ref_freqs
    plt.loglog(ref_freqs, ref_line * (binned_psd[0] / ref_line[0]), linestyle='--', color='red', label='1/f Reference')

    # Line of best fit
    log_freqs = np.log10(binned_freqs)
    log_psd = np.log10(binned_psd)
    slope, intercept, r_value, p_value, std_err = linregress(log_freqs, log_psd)
    plt.loglog(binned_freqs, 10**(intercept + slope * log_freqs), linestyle='-', color='green', label='Line of Best Fit')

    # Plot settings
    plt.title(title)
    plt.xlabel('Frequency')
    plt.ylabel('Power')
    plt.legend()
    plt.grid(True)
    plt.show()

# Example usage:
analyze_noise('pink_noise_samples.txt', 'Power Spectral Density of Pink Noise')
