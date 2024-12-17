# LLM Cancer Screening


## Overview

LLM Cancer Screening is a Rust-based application designed to analyze text data for indications of cancer. The application reads input from a CSV file, sends the text data to an API for analysis, and writes the results to an output CSV file. The project leverages Rust's safety and concurrency features to ensure reliable and efficient processing.

## Features

* **CSV Reading** : Reads input data from a CSV file using the Polars library.
* **Asynchronous API Calls** : Sends text data to an API endpoint asynchronously for analysis.
* **Concurrency** : Utilizes Rust's async/await and Tokio runtime for concurrent API requests.
* **DataFrame Manipulation** : Adds analysis results to the DataFrame and writes the updated DataFrame to an output CSV file.
* **Mock Server** : Includes a mock server for testing API calls without hitting the real endpoint.

## Prerequisites

* Rust (latest stable version)
* Cargo (Rust package manager)
* OpenAI API key (if using the real API)

## Installation

**Clone the repository**:

```bash
git clone https://github.com/yourusername/llm_cancer_screening.git
cd llm_cancer_screening
```

**Install dependencies:**

```bash
cargo build
```

**Set up environment variables** : Create a `.env` file in the project root with the following content:

```bash
OPENAI_API_KEY=your_openai_api_key
USE_MOCK_SERVER=false
```

**Run the application**

```bash
cargo run
```


## **Why Rust?**

* Python is my first love but it can be unsafe during during dtype inference and mutability. I choose Rust because it's a very safe language for something as serious as cancer detection, as well as how it watches out for you due to it's ownership capabilities. I decided against C++ because Rust automatically avoids dangling references and memory leaks so why deal with that noise if you can get the same speed & concurrency without managing that overhead. For modern C++ where smart-pointers take care of that same hassle I still chose Rust for it's ease of use if someone else picks up this project and extends it. For the healthcare providers reading this the analogy.

## **Healthcare Provider Analogy**

* **Device A (Python)** is like an older, well-loved diagnostic tool that you've used for years. It's quick and familiar, but sometimes it can give you uncertain results because it doesn't always handle the nuances of different patient data correctly. It can also be a bit unpredictable if not handled carefully, much like how Python can be unsafe during data type inference and mutability.

* **Device B (C++)** is a powerful and precise tool, but it requires a lot of manual adjustments and constant monitoring to ensure it doesn't malfunction. It's like a device that can give you excellent results but requires you to constantly check for issues like calibration errors or maintenance problems, similar to how C++ requires careful management to avoid dangling references and memory leaks.

* **Device C (Rust)** is a newer diagnostic tool that combines the best of both worlds. It's as powerful and fast as Device B, but it has built-in safeguards that automatically prevent common issues. It's like a device that ensures you never get calibration errors or maintenance problems because it takes care of those things for you. This makes it not only safe and reliable but also easier for other healthcare providers to use and maintain if they need to take over your work.
