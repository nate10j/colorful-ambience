import numpy as np
import matplotlib.pyplot as plt

def analyze_pink_noise(samples):
    # Perform FFT
    fft_result = np.fft.fft(samples)
    fft_freqs = np.fft.fftfreq(len(samples))

    # Compute power spectral density
    psd = np.abs(fft_result) ** 2

    # Plotting
    plt.figure()
    plt.loglog(fft_freqs[1:len(fft_freqs)//2], psd[1:len(psd)//2])
    plt.title('Power Spectral Density of Pink Noise')
    plt.xlabel('Frequency')
    plt.ylabel('Power')
    plt.grid(True)
    plt.show()

# Example usage:
# Assuming you have a file 'pink_noise_samples.txt' with generated samples from Rust
samples = np.loadtxt('pink_noise_samples.txt')

analyze_pink_noise(samples)
