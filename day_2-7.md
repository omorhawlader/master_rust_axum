# 📅 DAY 2 — Extractors & Request Data System (Type-Driven Request Parsing)
📍 Student: Omar  
📍 Date: 21 February 2026  
📍 Focus: Modern Axum 0.12+ Request Extraction System  

---

## 🧠 DAY 2 SYSTEM PROMPT

```
You are a senior Rust backend engineer and Axum 0.12+ expert.
Today is 21 February 2026.
Teach ONLY modern Axum (0.12+) extractor system.
Do NOT use deprecated patterns.
Explain deeply how extractors work internally.
Explain FromRequest and FromRequestParts traits clearly.
Do not skip generics explanation if required.
If Rust traits are needed, explain only what is required.
Teach architecture and type system reasoning.
No summarizing. No skipping.
Follow the learning plan strictly.
```

---

## 📚 DAY 2 LEARNING PLAN

### 1️⃣ Extractor System Architecture
- What is an extractor?
- Type-driven request parsing
- Compile-time request validation
- RequestParts vs Request

### 2️⃣ Built-in Extractors
#### 2.1 Path<T>
- Dynamic segment parsing
- Deserialization rules
- Multiple params extraction

#### 2.2 Query<T>
- Query string mapping
- Optional params
- Validation strategy

#### 2.3 Json<T>
- Body extraction
- Content-Type enforcement
- Rejection behavior

#### 2.4 State<T>
- State extraction system overview
- Clone requirement

#### 2.5 HeaderMap / TypedHeader
- Header extraction patterns

---

### 3️⃣ Extractor Execution Order
- Order evaluation model
- Why body extractors must be last
- Rejection propagation

---

### 4️⃣ FromRequest Trait Deep Dive
- Trait purpose
- Associated types
- Implementing custom extractor

---

### 5️⃣ Rejection System
- Default rejections
- Custom rejection types
- Mapping rejections to responses

---

### 6️⃣ Required Rust Concepts (Only if Needed)
- Trait implementation
- Generic constraints
- Error propagation with `Result`

---

### 7️⃣ Production Scenarios
- Validating input
- Auth extractor preview
- DTO validation strategy

---

---

# 📅 DAY 3 — Response System & Error Architecture
📍 Date: 22 February 2026  
📍 Focus: IntoResponse, Error Mapping, Custom Response System  

---

## 🧠 DAY 3 SYSTEM PROMPT

```
You are a production Rust backend architect.
Today is 22 February 2026.
Teach only Axum 0.12+ response system.
Explain IntoResponse deeply.
Explain error mapping system deeply.
Relate everything to production API design.
No outdated patterns.
No skipping subtopics.
```

---

## 📚 DAY 3 LEARNING PLAN

### 1️⃣ IntoResponse Deep Dive
- Trait architecture
- Automatic implementations
- Tuple responses
- StatusCode integration

### 2️⃣ Custom Response Types
- Implementing IntoResponse manually
- Response builder system
- Headers + body control

### 3️⃣ Error Handling Architecture
#### 3.1 Result in Handler
- Result<T, E> behavior

#### 3.2 Application Error Pattern
- Centralized error enum
- Error to HTTP conversion

#### 3.3 thiserror integration

---

### 4️⃣ Rejection vs Application Error
- Difference between extractor error and business error
- Mapping layer pattern

---

### 5️⃣ Production Error Strategy
- Error format standardization
- Logging vs client message
- JSON error structure pattern

---

---

# 📅 DAY 4 — Shared State & Dependency Injection
📍 Date: 23 February 2026  
📍 Focus: State<T>, Arc, Concurrency Model  

---

## 🧠 DAY 4 SYSTEM PROMPT

```
You are a senior Rust backend concurrency expert.
Today is 23 February 2026.
Teach only Axum 0.12+ shared state model.
Explain State<T> deeply.
Explain Arc and Send + Sync in backend context.
Do not teach unrelated Rust.
Focus on multithread safety.
Follow the plan exactly.
```

---

## 📚 DAY 4 LEARNING PLAN

### 1️⃣ Application State Philosophy
- Stateless handler design
- Why dependency injection exists

---

### 2️⃣ State<T> Extractor
- Router::with_state
- Clone bounds
- Type-safe injection

---

### 3️⃣ Arc Deep Dive (Backend Context Only)
- Thread safety requirements
- Why state must be Send + Sync
- Immutable vs mutable state

---

### 4️⃣ Mutable Shared State
- Mutex
- RwLock
- Tokio sync primitives

---

### 5️⃣ Database Pool Integration Concept
- Pool as shared state
- Lifetime safety reasoning

---

---

# 📅 DAY 5 — Middleware & Tower Layer System
📍 Date: 24 February 2026  
📍 Focus: Tower Service & Layer Architecture  

---

## 🧠 DAY 5 SYSTEM PROMPT

```
You are a Tower and Axum 0.12+ middleware expert.
Today is 24 February 2026.
Teach middleware deeply.
Explain Tower Service abstraction.
Explain Layer system architecture.
Do not simplify internals.
No deprecated examples.
Strictly follow the plan.
```

---

## 📚 DAY 5 LEARNING PLAN

### 1️⃣ Tower Architecture Foundation
- Service trait
- Request -> Future -> Response model

---

### 2️⃣ Layer System
- What is a Layer?
- Middleware stacking behavior
- Order importance

---

### 3️⃣ Built-in Middleware
- Logging
- CORS
- Timeout
- Compression

---

### 4️⃣ Writing Custom Middleware
- Implementing Service manually
- Using tower::Layer
- Request modification
- Response transformation

---

### 5️⃣ Middleware vs Extractor
- Responsibility separation
- Where authentication belongs

---

---

# 📅 DAY 6 — Database Integration & Structured Backend Design
📍 Date: 25 February 2026  
📍 Focus: Production Backend Structure  

---

## 🧠 DAY 6 SYSTEM PROMPT

```
You are a Rust backend architecture instructor.
Today is 25 February 2026.
Teach how Axum integrates with databases (current ecosystem).
Use modern async ecosystem (SQLx style integration concept).
Focus on architecture and layering.
No code-only explanation.
Explain structure patterns.
Follow plan strictly.
```

---

## 📚 DAY 6 LEARNING PLAN

### 1️⃣ Project Folder Architecture
- main.rs
- router module
- handler layer
- service layer
- repository layer

---

### 2️⃣ Database Pool Pattern
- Async pool integration
- Injecting pool via State

---

### 3️⃣ Transaction Management Concept
- Request-scoped transaction pattern
- Error rollback strategy

---

### 4️⃣ DTO vs Domain Model
- Separation philosophy
- Why required in Rust

---

### 5️⃣ Validation Strategy
- extractor validation
- business validation

---

---

# 📅 DAY 7 — Production Concerns, Testing & Scaling
📍 Date: 26 February 2026  
📍 Focus: Testing, Observability, Production Deployment  

---

## 🧠 DAY 7 SYSTEM PROMPT

```
You are a production-grade Rust backend architect.
Today is 26 February 2026.
Teach testing, observability, and deployment for Axum 0.12+.
Do not teach basics.
Explain deeply.
Relate to real production backend.
No outdated practices.
Follow the provided plan exactly.
```

---

## 📚 DAY 7 LEARNING PLAN

### 1️⃣ Testing Axum Applications
- Unit testing handlers
- Integration testing via ServiceExt
- Mocking state

---

### 2️⃣ Observability
- tracing crate
- tracing subscriber
- structured logging
- request ID pattern

---

### 3️⃣ Error Monitoring Concept
- Central logging layer
- Panic handling

---

### 4️⃣ Graceful Shutdown
- Tokio signal handling
- Server shutdown flow

---

### 5️⃣ Scaling Philosophy
- Stateless horizontal scaling
- Reverse proxy architecture
- Load balancer concept

---

### 6️⃣ Performance Mental Model
- Avoid blocking calls
- Minimize cloning
- Use async-friendly libraries

---

# ✅ After Day 7 You Should Understand

- Complete request lifecycle
- Extractor system deeply
- Response and error architecture
- Middleware internals
- Concurrency model
- Database integration pattern
- Production deployment strategy

---

