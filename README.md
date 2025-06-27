# Fileshare API

## 🚀 Overview

Welcome to the **Fileshare API**! This project aims to provide a robust, secure, and highly performant file sharing service built with Rust. Our goal is to offer a comprehensive set of features for managing, sharing, and collaborating on files, leveraging Rust's strengths in concurrency, memory safety, and performance.

## ✨ Current Features

Our Fileshare API currently supports the following core functionalities:

* **📤 File Upload:** Securely upload files to the system.

* **📥 File Download:** Retrieve files efficiently.

* **📝 User Sign Up:** Create new user accounts.

* **🔑 User Login:** Authenticate existing users.

## 🗺️ Roadmap: Enhancing the Fileshare API with Rust

This roadmap outlines the planned stages for expanding the Fileshare API. Each phase introduces new features and highlights the key Rust concepts that will be central to their implementation, offering opportunities to dive deeper into Rust's powerful ecosystem.

### Phase 1: Core System & Data Foundation

This phase focuses on solidifying the basic file and folder management capabilities, along with fundamental data structures and reliable error handling in Rust.

**Features to Implement:**

* **Folder Management:** Allow users to create, rename, move, and delete folders, supporting hierarchical structures.

* **File Renaming and Deletion:** Implement API endpoints for renaming existing files and securely deleting them.

* **Basic Metadata Management:** Store and retrieve essential file metadata such as file size, upload timestamp, original filename, and content type.

* **Admin User Management:** Create endpoints for an administrator to view, create, update, and delete user accounts, laying the groundwork for broader permission systems.

**Key Rust Concepts to Focus On:**

* **`struct`s and `enum`s:** Extensive use for modeling core entities like `User`, `File`, `Folder`, and `Permission`. `enum`s will also represent different states or types.

* **Ownership, Borrowing, and Lifetimes:** Crucial for efficient data management (file paths, contents) without copies, ensuring memory safety and performance.

* **Error Handling (`Result<T, E>` and `?` operator):** Fundamental for robust error management. Define custom error types for specific scenarios (e.g., "file not found," "permission denied").

* **Collections (`HashMap`, `BTreeMap`):** `HashMap` for fast lookups by ID, `BTreeMap` if sorted iteration over keys is required.

* **Traits (`Debug`, `Clone`, `Eq`, `PartialEq`, `Serialize`, `Deserialize`):** Essential for debugging, copying data, comparing types, and (de)serializing data for API communication using `serde`.

### Phase 2: Secure Sharing & Versioning

This phase introduces advanced sharing capabilities, robust data versioning, and resource management.

**Features to Implement:**

* **Shareable Links with Controls:**

    * Generate unique, shareable URLs for files/folders.

    * Add options for password protection, expiration dates, and optional download limits on links.

* **Version Control:** Automatically save previous file versions on new uploads and provide endpoints to list and revert to specific older versions.

* **Storage Quotas:** Implement per-user storage limits and prevent uploads if quotas are exceeded.

**Key Rust Concepts to Focus On:**

* **Generics:** Use for designing flexible `ShareLink` structures and functions that can work across different item types (files, folders) or sharing configurations, enhancing code reusability.

* **Concurrency Primitives (`Arc`, `Mutex`, `RwLock`):** Essential for safely sharing mutable state (e.g., global metadata, user quotas) between concurrent API requests, preventing race conditions.

* **Time and Date Handling (`chrono` crate):** Utilize for accurate management of link expirations, file timestamps, and versioning.

* **Cryptography Crates (e.g., `argon2`, hashing algorithms):** For secure password hashing (user passwords) and link password verification.

* **Custom Iterators:** Consider implementing for efficient traversal of file/folder hierarchies or file versions.

### Phase 3: Eventing, Security & Scalability

This phase introduces real-time notifications, enhanced security measures, and patterns for handling concurrent operations.

**Features to Implement:**

* **Webhooks:** Allow users to register URLs to receive real-time notifications for specific events (e.g., file uploaded, file deleted).

* **Audit Logs:** Maintain detailed logs of all significant user and system actions for security and compliance.

* **Rate Limiting:** Implement mechanisms to control API request frequency per user/IP to prevent abuse and ensure service stability.

* **Two-Factor Authentication (2FA/MFA) API Support:** Provide endpoints for managing 2FA/MFA, including code generation and verification.

* **Virus Scanning Integration:** Integrate with an external virus scanning service to scan uploaded files before they are made fully available.

**Key Rust Concepts to Focus On:**

* **Asynchronous Programming (`tokio`, `async`/`await`):** Crucial for handling long-running I/O operations (like sending webhooks or external API calls) without blocking the main thread, greatly improving API scalability and responsiveness.

* **Channels (e.g., `tokio::sync::mpsc`):** Use for inter-thread communication, decoupling tasks like virus scanning or webhook sending from the immediate API response.

* **Database Interaction (if applicable):** If persisting data to a database, utilize a suitable database client crate (e.g., `sqlx`) that often comes with `async` interfaces.

* **HTTP Clients (`reqwest`):** Powerful asynchronous HTTP client for making outgoing requests to webhooks and external services.

* **Advanced Error Handling (Context and Propagation):):** Leverage crates like `anyhow` or `thiserror` to add rich context to errors, making debugging more efficient.

### Phase 4: Enhanced User Experience & Analytics

This final phase focuses on improving the user experience with features like file previews and advanced search, along with providing valuable insights.


## 🚀 Getting Started

To get started with this project, clone the repository and follow the instructions in the `CONTRIBUTING.md` file (once created).
