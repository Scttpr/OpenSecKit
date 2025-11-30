---
title: "E-Commerce Checkout - STRIDE Threat Model"
template_version: "1.0.0"
constitutional_principle: ["I"]
ssdlc_phase: "planning"
domain_applicability: "ecommerce"
last_updated: "2025-11-18"
reviewers: ["security-champion-lead", "security-champion-architect"]
related_templates:
  - "../../templates/threat-modeling/stride-threat-model-template-planning.md"
  - "../02-risk-analysis/_example-ecommerce-risk.md"
tags: ["example", "ecommerce", "payment", "STRIDE", "PCI-DSS"]
scenario_description: "Complete STRIDE threat model for an e-commerce checkout flow handling payment processing, user authentication, and order fulfillment"
complexity_level: "intermediate"
annotations_included: true
---

# Threat Model: E-Commerce Checkout Flow

**Feature**: Shopping Cart Checkout with Payment Processing
**Application**: SecureShop E-Commerce Platform
**Owner**: Product Team - Checkout Squad
**Threat Modeling Date**: 2025-11-18
**Threat Modeling Participants**: Security Architect, Backend Lead, Frontend Lead, Product Manager
**Review Status**: Approved by Security Champion

## Executive Summary

This threat model analyzes the security risks in the checkout flow for SecureShop, an e-commerce platform processing credit card payments. The checkout flow handles sensitive payment data (PCI-DSS scope), user authentication, and order fulfillment. Using the STRIDE methodology, we identified **23 distinct threats** across 6 categories, with **8 high-priority threats** requiring immediate mitigation before production release.

**Key Findings**:
- **Critical**: Payment data exposure during transmission (S03, I01)
- **Critical**: SQL injection in order processing (T02)
- **High**: Session hijacking vulnerabilities (S02)
- **High**: Insufficient authorization on payment endpoints (E02)

**Next Steps**: See [Risk Analysis](../02-risk-analysis/_example-ecommerce-risk.md) for prioritized remediation plan.

---

## 1. System Overview

### 1.1 Feature Description

The checkout flow enables customers to:
1. Review cart contents (items, quantities, subtotal)
2. Enter shipping address
3. Select shipping method
4. Enter payment information (credit card via Stripe)
5. Review order summary
6. Confirm and submit order
7. Receive order confirmation

### 1.2 Architecture Diagram

```
┌─────────────┐          ┌──────────────┐          ┌─────────────┐
│   Browser   │  HTTPS   │  Web Server  │   API    │   Backend   │
│  (React SPA)│◄────────►│   (Nginx)    │◄────────►│  (Node.js)  │
└─────────────┘          └──────────────┘          └─────────────┘
                                                           │
                                                           ▼
                         ┌──────────────┐          ┌─────────────┐
                         │    Stripe    │          │  PostgreSQL │
                         │  Payment API │          │  (Orders DB)│
                         └──────────────┘          └─────────────┘
                                │                         │
                                └─────────────────────────┘
                                  (Tokenized payment data)
```

### 1.3 Data Flow

**Step 1: Cart Review**
- User → Frontend: Request cart contents
- Frontend → Backend: GET /api/cart (JWT in Authorization header)
- Backend → Database: Query cart items for user
- Database → Backend → Frontend → User: Cart data (product IDs, quantities, prices)

**Step 2: Shipping Address**
- User → Frontend: Enter address (name, street, city, postal code, country)
- Frontend: Client-side validation (format, completeness)
- Frontend → Backend: POST /api/checkout/shipping (address data + JWT)
- Backend: Server-side validation, create shipping record
- Backend → Database: INSERT shipping_addresses

**Step 3: Payment Information**
- User → Frontend: Enter card details (number, expiry, CVV)
- Frontend → Stripe.js: Tokenize card data (PCI-DSS SAQ-A compliance - no card data touches our servers)
- Stripe → Frontend: Return payment token (tok_abc123)
- Frontend → Backend: POST /api/checkout/payment (token only + JWT)
- Backend → Stripe API: Create payment intent with token
- Stripe → Backend: Confirm payment intent status

**Step 4: Order Confirmation**
- Backend → Database: Create order record (user_id, items, total, shipping, payment_intent_id)
- Backend → Frontend: Order confirmation (order_id, total, estimated delivery)
- Frontend → User: Display confirmation page

### 1.4 Technology Stack

- **Frontend**: React 18.2, Stripe.js 3.x, Axios
- **Backend**: Node.js 20.x, Express 4.18, Sequelize ORM
- **Database**: PostgreSQL 15.x
- **Payment Gateway**: Stripe API v2023-10-16
- **Authentication**: JWT (access tokens), HTTP-only cookies (refresh tokens)
- **Web Server**: Nginx 1.24 (TLS 1.3, reverse proxy)

---

## 2. Assets

### 2.1 Data Assets

| Asset ID | Asset Name | Sensitivity | PII | PCI-DSS | Description |
|----------|------------|-------------|-----|---------|-------------|
| D01 | Payment card data | **CRITICAL** | Yes | Yes | Card number, expiry, CVV (tokenized via Stripe) |
| D02 | User credentials | **CRITICAL** | Yes | No | Email, hashed password, JWT tokens |
| D03 | Shipping addresses | **HIGH** | Yes | No | Name, street address, postal code, phone |
| D04 | Order history | **MEDIUM** | Yes | No | Order IDs, items purchased, totals, timestamps |
| D05 | Cart data | **LOW** | No | No | Product IDs, quantities (temporary) |
| D06 | Payment tokens | **CRITICAL** | No | Yes | Stripe payment tokens (single-use) |
| D07 | Session data | **HIGH** | Yes | No | JWT access tokens, refresh tokens, session IDs |

### 2.2 Functional Assets

| Asset ID | Asset Name | Criticality | Description |
|----------|------------|-------------|-------------|
| F01 | Payment processing | **CRITICAL** | Stripe API integration for charging cards |
| F02 | Order creation | **HIGH** | Backend service creating order records |
| F03 | Inventory management | **HIGH** | Stock validation and reservation |
| F04 | User authentication | **CRITICAL** | JWT-based authentication system |
| F05 | Shipping calculation | **MEDIUM** | Logic for calculating shipping costs |
| F06 | Email notifications | **MEDIUM** | Order confirmation emails |

### 2.3 Infrastructure Assets

| Asset ID | Asset Name | Criticality | Description |
|----------|------------|-------------|-------------|
| I01 | PostgreSQL database | **CRITICAL** | Stores all application data |
| I02 | Backend API servers | **CRITICAL** | Node.js application servers |
| I03 | Nginx reverse proxy | **HIGH** | TLS termination, load balancing |
| I04 | Stripe API keys | **CRITICAL** | Secret keys for payment processing |

---

## 3. Trust Boundaries

### 3.1 Boundary Definitions

1. **User ↔ Frontend (Browser)**
   - Trust Level: Untrusted
   - Communication: HTTPS (TLS 1.3)
   - Controls: CSP, SRI, CORS

2. **Frontend ↔ Backend API**
   - Trust Level: Semi-trusted (authenticated users)
   - Communication: HTTPS with JWT authentication
   - Controls: JWT validation, rate limiting, input validation

3. **Backend API ↔ Database**
   - Trust Level: Trusted (internal network)
   - Communication: PostgreSQL wire protocol (encrypted)
   - Controls: Parameterized queries, least privilege DB user

4. **Backend API ↔ Stripe API**
   - Trust Level: Semi-trusted (third-party service)
   - Communication: HTTPS with API key authentication
   - Controls: API key rotation, webhook signature validation

### 3.2 Entry Points

| Entry Point | Trust Boundary | Authentication | Authorization |
|-------------|----------------|----------------|---------------|
| GET /api/cart | Frontend → Backend | JWT required | User can only access own cart |
| POST /api/checkout/shipping | Frontend → Backend | JWT required | User can only update own shipping |
| POST /api/checkout/payment | Frontend → Backend | JWT required | User can only pay for own order |
| POST /api/orders | Frontend → Backend | JWT required | User can only create orders for own cart |
| POST /webhooks/stripe | Stripe → Backend | Webhook signature | N/A (verified via signature) |

---

## 4. STRIDE Threat Analysis

### 4.1 Spoofing Identity (S)

#### S01: User impersonation via stolen JWT token

**Description**: Attacker steals valid JWT access token (e.g., via XSS, network interception) and impersonates legitimate user.

**Assets Threatened**: D02 (User credentials), D07 (Session data), F04 (Authentication)

**Attack Scenario**:
1. Attacker injects malicious JavaScript via stored XSS vulnerability
2. Script exfiltrates JWT access token from localStorage
3. Attacker replays token in Authorization header to access victim's cart and place orders

**Impact**: Attacker can view victim's cart, modify shipping address, place unauthorized orders

**Likelihood**: Medium (requires XSS vulnerability)

**Countermeasures**:
- ✅ **IMPLEMENTED**: JWT tokens stored in memory only (not localStorage)
- ✅ **IMPLEMENTED**: HTTP-only cookies for refresh tokens
- ✅ **IMPLEMENTED**: Short token expiry (15 minutes for access tokens)
- ✅ **IMPLEMENTED**: CSP headers to prevent XSS
- ⏳ **PLANNED**: Bind JWT to device fingerprint (IP + User-Agent hash)

**Residual Risk**: Low (after planned countermeasures)

---

#### S02: Session hijacking via network interception

**Description**: Attacker intercepts HTTP traffic to steal session tokens in transit.

**Assets Threatened**: D07 (Session data), F04 (Authentication)

**Attack Scenario**:
1. User connects to public Wi-Fi (coffee shop, airport)
2. Attacker performs MITM attack (ARP spoofing, rogue AP)
3. Attacker downgrades HTTPS to HTTP or exploits weak TLS configuration
4. Attacker captures JWT token from intercepted requests

**Impact**: Complete account takeover, unauthorized purchases

**Likelihood**: Low (HTTPS enforced, but misconfigurations possible)

**Countermeasures**:
- ✅ **IMPLEMENTED**: TLS 1.3 only (Nginx config: `ssl_protocols TLSv1.3;`)
- ✅ **IMPLEMENTED**: HSTS header (max-age=31536000, includeSubDomains, preload)
- ✅ **IMPLEMENTED**: Certificate pinning (backend → Stripe)
- ❌ **MISSING**: Certificate pinning (frontend → backend) - browser limitation
- ✅ **IMPLEMENTED**: Secure cookie flags (Secure, HttpOnly, SameSite=Strict)

**Residual Risk**: Very Low

---

#### S03: Payment data tampering via Stripe token manipulation

**Description**: Attacker attempts to manipulate Stripe payment token to charge different amount or different card.

**Assets Threatened**: D01 (Payment card data), D06 (Payment tokens), F01 (Payment processing)

**Attack Scenario**:
1. Legitimate user tokenizes card via Stripe.js (tok_abc123 for $100)
2. Attacker intercepts POST /api/checkout/payment request
3. Attacker modifies payment token or amount in transit
4. Backend processes manipulated payment

**Impact**: Incorrect charges, payment fraud, financial loss

**Likelihood**: Very Low (Stripe tokens are single-use and amount-bound)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Stripe payment intents (amount locked server-side, not client-provided)
- ✅ **IMPLEMENTED**: Backend recalculates order total from cart (ignores client-provided amount)
- ✅ **IMPLEMENTED**: TLS for all payment requests
- ✅ **IMPLEMENTED**: Stripe webhook validation (signature verification)

**Residual Risk**: Very Low

---

### 4.2 Tampering (T)

#### T01: Cart price manipulation

**Description**: Attacker modifies cart item prices in frontend to purchase items at lower prices.

**Assets Threatened**: D05 (Cart data), F02 (Order creation)

**Attack Scenario**:
1. User adds $200 item to cart
2. Attacker opens browser DevTools, modifies cart state (React state mutation)
3. Frontend displays $10 price
4. Attacker proceeds to checkout
5. Backend uses client-provided price instead of database price

**Impact**: Financial loss, revenue theft, inventory sold below cost

**Likelihood**: High (common web attack)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Backend recalculates all prices from product database (never trusts client prices)
- ✅ **IMPLEMENTED**: Cart validation on checkout: `SELECT price FROM products WHERE id = ?`
- ✅ **IMPLEMENTED**: Order total validation before payment processing
- ⏳ **PLANNED**: Audit logging of price mismatches (frontend vs backend)

**Residual Risk**: Very Low (if backend validation works correctly)

---

#### T02: SQL injection in order processing

**Description**: Attacker injects SQL commands via user input (shipping address, product IDs) to manipulate database.

**Assets Threatened**: I01 (Database), D04 (Order history), D03 (Shipping addresses)

**Attack Scenario**:
1. Attacker enters malicious string in shipping address field: `'; DROP TABLE orders; --`
2. Backend concatenates user input into SQL query (vulnerable code)
3. Database executes malicious SQL, dropping orders table

**Impact**: Data loss, database corruption, unauthorized data access

**Likelihood**: Medium (if parameterized queries not used consistently)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Sequelize ORM with parameterized queries (all database access)
- ✅ **IMPLEMENTED**: Input validation (whitelist allowed characters for address fields)
- ✅ **IMPLEMENTED**: Least privilege database user (no DROP permissions)
- ✅ **IMPLEMENTED**: Database read replicas (separation of read/write access)

**Residual Risk**: Very Low

---

#### T03: Payment amount modification before Stripe charge

**Description**: Race condition where attacker modifies order total between validation and payment processing.

**Assets Threatened**: F01 (Payment processing), D06 (Payment tokens)

**Attack Scenario**:
1. User initiates checkout with $100 order
2. Backend validates total ($100) and creates payment intent
3. Attacker sends concurrent request to modify cart (adds $500 item)
4. Backend charges $100 but fulfills $600 order

**Impact**: Financial loss, incorrect charges

**Likelihood**: Low (requires precise timing)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Database transactions (BEGIN ... COMMIT)
- ✅ **IMPLEMENTED**: Cart locking during checkout (SELECT FOR UPDATE)
- ✅ **IMPLEMENTED**: Order total recalculation immediately before payment
- ⏳ **PLANNED**: Idempotency keys for payment requests

**Residual Risk**: Very Low

---

### 4.3 Repudiation (R)

#### R01: User denies placing order

**Description**: User places order, receives goods, then claims they didn't authorize the purchase.

**Assets Threatened**: D04 (Order history), F02 (Order creation)

**Attack Scenario**:
1. User completes checkout, order is fulfilled and shipped
2. User contacts support claiming account was hacked
3. User requests refund without returning goods

**Impact**: Financial loss (chargebacks), fraud

**Likelihood**: Medium (chargeback fraud is common)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Audit logs for all order creation events (timestamp, IP, User-Agent)
- ✅ **IMPLEMENTED**: Email confirmation sent to user's registered email
- ✅ **IMPLEMENTED**: SMS/2FA for high-value orders (>$500)
- ⏳ **PLANNED**: Device fingerprinting (track known devices)
- ⏳ **PLANNED**: Shipping address verification (compare to previous orders)

**Residual Risk**: Medium (chargebacks still possible, but audit trail supports investigation)

---

#### R02: Administrator denies configuration changes

**Description**: Admin modifies payment gateway configuration (e.g., changes Stripe API keys) and denies responsibility.

**Assets Threatened**: I04 (Stripe API keys), F01 (Payment processing)

**Attack Scenario**:
1. Malicious admin replaces production Stripe keys with personal account keys
2. Payments route to attacker's Stripe account instead of company account
3. Admin denies making changes when discovered

**Impact**: Payment theft, financial loss, compliance violations

**Likelihood**: Low (requires malicious insider)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Admin action audit logs (timestamp, user, IP, action)
- ✅ **IMPLEMENTED**: Change management process (2-person approval for config changes)
- ✅ **IMPLEMENTED**: Version control for configuration (Git history)
- ⏳ **PLANNED**: Alerting on Stripe API key changes (Slack notification)

**Residual Risk**: Low

---

### 4.4 Information Disclosure (I)

#### I01: Payment card data exposure in browser

**Description**: Sensitive payment card data (PAN, CVV) stored in browser memory or transmitted to backend.

**Assets Threatened**: D01 (Payment card data)

**Attack Scenario**:
1. User enters card details in frontend form
2. Vulnerable frontend code stores card data in localStorage or sends to backend
3. Attacker exploits XSS to exfiltrate card data
4. PCI-DSS compliance violated (SAQ-D scope instead of SAQ-A)

**Impact**: PCI-DSS violation, fines ($5,000-$100,000/month), card data breach

**Likelihood**: High (if not using Stripe.js correctly)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Stripe.js tokenization (card data NEVER touches our servers)
- ✅ **IMPLEMENTED**: PCI-DSS SAQ-A compliance (validated via Stripe)
- ✅ **IMPLEMENTED**: CSP headers blocking inline scripts
- ✅ **IMPLEMENTED**: No card data in logs, databases, or application code

**Residual Risk**: Very Low

---

#### I02: User data exposure via insecure API responses

**Description**: API responses include sensitive data (emails, phone numbers, addresses) for other users.

**Assets Threatened**: D03 (Shipping addresses), D04 (Order history)

**Attack Scenario**:
1. Attacker calls GET /api/orders/123
2. Backend fails to check authorization (order belongs to different user)
3. API returns order details including shipping address and items
4. Attacker iterates order IDs to harvest user data

**Impact**: PII breach, GDPR violations, user privacy loss

**Likelihood**: Medium (IDOR vulnerabilities are common)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Authorization checks on all endpoints (user_id = JWT.user_id)
- ✅ **IMPLEMENTED**: Database queries scoped to authenticated user
- ✅ **IMPLEMENTED**: Unit tests for authorization (negative test cases)
- ⏳ **PLANNED**: API security testing (automated IDOR detection)

**Residual Risk**: Low

---

#### I03: Shipping address disclosure via timing attack

**Description**: Attacker infers valid shipping addresses by measuring API response times.

**Assets Threatened**: D03 (Shipping addresses)

**Attack Scenario**:
1. Attacker submits checkout request with guessed address
2. Valid addresses trigger address verification service (slower response)
3. Invalid addresses fail fast (quicker response)
4. Attacker uses timing difference to enumerate valid addresses

**Impact**: Address enumeration, privacy breach

**Likelihood**: Very Low (requires sophisticated attacker)

**Countermeasures**:
- ⏳ **PLANNED**: Constant-time address validation (add delay to fast path)
- ✅ **IMPLEMENTED**: Rate limiting (max 10 requests/minute per IP)

**Residual Risk**: Very Low

---

### 4.5 Denial of Service (D)

#### D01: Payment processing exhaustion

**Description**: Attacker floods payment endpoint with requests to exhaust Stripe API quota or backend resources.

**Assets Threatened**: F01 (Payment processing), I02 (Backend servers)

**Attack Scenario**:
1. Attacker creates bot network (or uses compromised accounts)
2. Bot submits thousands of payment requests to POST /api/checkout/payment
3. Stripe API rate limits triggered (100 requests/second)
4. Legitimate users unable to complete checkout

**Impact**: Revenue loss, customer churn, SLA violations

**Likelihood**: Medium (DDoS attacks common for e-commerce)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Rate limiting (5 payment attempts/minute per user)
- ✅ **IMPLEMENTED**: CAPTCHA for suspicious behavior (multiple failed payments)
- ⏳ **PLANNED**: WAF with DDoS protection (Cloudflare)
- ⏳ **PLANNED**: Payment intent caching (reuse intent if duplicate request)

**Residual Risk**: Medium (DDoS protection requires CDN)

---

#### D02: Database connection pool exhaustion

**Description**: Attacker floods API with requests to exhaust database connection pool.

**Assets Threatened**: I01 (Database), I02 (Backend servers)

**Attack Scenario**:
1. Attacker sends rapid requests to GET /api/cart
2. Each request opens database connection
3. Connection pool (max 20 connections) exhausted
4. Legitimate requests fail with "connection timeout" errors

**Impact**: Service outage, customer impact

**Likelihood**: Medium

**Countermeasures**:
- ✅ **IMPLEMENTED**: Connection pooling (max 20 connections, queue overflow)
- ✅ **IMPLEMENTED**: Request timeout (30 seconds max)
- ✅ **IMPLEMENTED**: Rate limiting (100 requests/minute per IP)
- ⏳ **PLANNED**: Database read replicas (distribute read load)

**Residual Risk**: Low

---

### 4.6 Elevation of Privilege (E)

#### E01: Unauthorized admin access via JWT role manipulation

**Description**: Attacker modifies JWT payload to escalate privileges from user to admin.

**Assets Threatened**: F04 (Authentication), I01 (Database)

**Attack Scenario**:
1. Attacker obtains valid user JWT: `{ "user_id": 123, "role": "user" }`
2. Attacker decodes JWT, modifies role: `{ "user_id": 123, "role": "admin" }`
3. Attacker re-encodes JWT with same signature (if HMAC key leaked)
4. Backend accepts modified token, grants admin privileges

**Impact**: Complete system compromise, data breach, financial fraud

**Likelihood**: Low (requires secret key leak or algorithm confusion)

**Countermeasures**:
- ✅ **IMPLEMENTED**: JWT signed with RS256 (asymmetric, not HS256)
- ✅ **IMPLEMENTED**: Private key stored in secure environment variable (not in code)
- ✅ **IMPLEMENTED**: Role validation on every admin endpoint (not just JWT claim)
- ✅ **IMPLEMENTED**: Algorithm verification (reject unsigned tokens)

**Residual Risk**: Very Low

---

#### E02: Unauthorized payment modification via IDOR

**Description**: Attacker modifies payment method for another user's order.

**Assets Threatened**: F01 (Payment processing), D06 (Payment tokens)

**Attack Scenario**:
1. User A creates order (order_id=100)
2. Attacker (user B) guesses order ID
3. Attacker calls PUT /api/orders/100/payment with their own Stripe token
4. Backend fails to check order ownership
5. User A's order is charged to attacker's card (or vice versa)

**Impact**: Payment fraud, unauthorized charges

**Likelihood**: High (if authorization missing)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Order ownership validation (user_id = JWT.user_id)
- ✅ **IMPLEMENTED**: Payment intent locked after creation (immutable order_id)
- ✅ **IMPLEMENTED**: Stripe payment intent metadata (includes user_id for verification)

**Residual Risk**: Very Low

---

#### E03: Admin panel access via path traversal

**Description**: Attacker accesses admin dashboard via URL manipulation.

**Assets Threatened**: F04 (Authentication), I01 (Database)

**Attack Scenario**:
1. Attacker discovers admin panel at /admin/dashboard
2. Attacker tries path traversal: /admin/../api/users
3. Backend routing bypasses authentication middleware due to misconfiguration
4. Attacker accesses user management API without authentication

**Impact**: Unauthorized admin access, user data breach

**Likelihood**: Low (requires routing misconfiguration)

**Countermeasures**:
- ✅ **IMPLEMENTED**: Authentication middleware applied globally (all /api/* routes)
- ✅ **IMPLEMENTED**: Path normalization (prevent ../ traversal)
- ✅ **IMPLEMENTED**: Admin routes isolated to /admin namespace with separate middleware
- ✅ **IMPLEMENTED**: Role-based access control (RBAC) checks on every admin route

**Residual Risk**: Very Low

---

## 5. Threat Summary Table

| ID | Category | Threat | Likelihood | Impact | Risk | Countermeasures | Residual Risk |
|----|----------|--------|------------|--------|------|-----------------|---------------|
| S01 | Spoofing | JWT token theft | Medium | High | **HIGH** | CSP, short expiry, device binding | Low |
| S02 | Spoofing | Session hijacking | Low | High | Medium | TLS 1.3, HSTS, secure cookies | Very Low |
| S03 | Spoofing | Payment token manipulation | Very Low | Critical | Medium | Stripe payment intents | Very Low |
| T01 | Tampering | Cart price manipulation | High | High | **HIGH** | Backend price recalculation | Very Low |
| T02 | Tampering | SQL injection | Medium | Critical | **HIGH** | Parameterized queries | Very Low |
| T03 | Tampering | Payment race condition | Low | High | Medium | DB transactions, locking | Very Low |
| R01 | Repudiation | Order denial | Medium | Medium | Medium | Audit logs, email confirmation | Medium |
| R02 | Repudiation | Config change denial | Low | High | Medium | Admin audit logs | Low |
| I01 | Information Disclosure | Card data exposure | High | Critical | **CRITICAL** | Stripe.js tokenization | Very Low |
| I02 | Information Disclosure | User data exposure (IDOR) | Medium | High | **HIGH** | Authorization checks | Low |
| I03 | Information Disclosure | Address enumeration | Very Low | Low | Very Low | Constant-time validation | Very Low |
| D01 | Denial of Service | Payment flooding | Medium | High | **HIGH** | Rate limiting, CAPTCHA | Medium |
| D02 | Denial of Service | Connection pool exhaustion | Medium | High | **HIGH** | Connection pooling, rate limiting | Low |
| E01 | Elevation of Privilege | JWT role manipulation | Low | Critical | **HIGH** | RS256 signature, role validation | Very Low |
| E02 | Elevation of Privilege | Payment IDOR | High | High | **HIGH** | Ownership validation | Very Low |
| E03 | Elevation of Privilege | Path traversal admin access | Low | Critical | **HIGH** | Path normalization, RBAC | Very Low |

**Risk Levels**:
- **CRITICAL**: Immediate action required, blocks production deployment
- **HIGH**: Must address before production release
- **MEDIUM**: Address in next sprint
- **LOW**: Backlog, monitor for changes

**Threat Count**: 16 threats analyzed (6 more identified in full assessment)
**High/Critical Threats**: 8
**Mitigated to Low/Very Low**: 14 (88%)

---

## 6. Mitigation Summary

### 6.1 Implemented Controls

✅ **Authentication & Authorization**:
- JWT with RS256 (asymmetric signing)
- Short token expiry (15 minutes)
- Ownership validation on all user-specific resources
- Role-based access control (RBAC)

✅ **Data Protection**:
- TLS 1.3 for all communications
- Stripe.js tokenization (PCI-DSS SAQ-A)
- Parameterized queries (SQL injection prevention)
- Secure cookie flags (HttpOnly, Secure, SameSite)

✅ **Input Validation**:
- Backend price recalculation (never trust client)
- Whitelist validation for addresses
- Frontend + backend validation

✅ **Audit & Monitoring**:
- Audit logs for all sensitive operations
- Email confirmations for orders

### 6.2 Planned Controls (Sprint 2)

⏳ **Rate Limiting Enhancements**:
- WAF with DDoS protection (Cloudflare)
- Payment intent caching (idempotency)

⏳ **Advanced Monitoring**:
- Device fingerprinting
- Address verification service
- Price mismatch alerting

⏳ **Security Testing**:
- Automated IDOR detection
- Penetration testing

### 6.3 Deferred Controls (Backlog)

📋 **Advanced Features**:
- Behavioral analytics (fraud detection)
- Customer risk scoring
- Machine learning for anomaly detection

---

## 7. Testing Requirements

### 7.1 Security Test Cases

For each threat, corresponding test cases must pass:

| Threat ID | Test Case | Expected Result |
|-----------|-----------|-----------------|
| S01 | Replay stolen JWT after expiry | 401 Unauthorized |
| T01 | Modify cart price in browser, checkout | Backend uses DB price, not client price |
| T02 | Submit SQL injection payload in address | Input rejected, no SQL execution |
| I02 | Request other user's order | 403 Forbidden |
| E02 | Modify payment for other user's order | 403 Forbidden |

See [Security Testing Guide](../../templates/04-security-testing/sast-integration-guide-implementation.md) for full test suite.

### 7.2 Penetration Testing

Recommended scope:
- OWASP Top 10 testing
- Payment flow fuzzing
- Authorization bypass attempts
- Session management testing

---

## 8. Compliance Mapping

### PCI-DSS Requirements

| Requirement | Control | Implementation |
|-------------|---------|----------------|
| 6.5.1 (Injection) | Input validation | Parameterized queries |
| 6.5.3 (Cryptography) | TLS 1.3 | Nginx configuration |
| 6.5.10 (Broken Auth) | JWT + short expiry | Authentication middleware |
| 12.3 (SAQ) | SAQ-A compliance | Stripe.js tokenization |

### GDPR Requirements

| Requirement | Control | Implementation |
|-------------|---------|----------------|
| Art. 32 (Security) | Encryption | TLS 1.3, database encryption |
| Art. 5 (Data minimization) | Minimal PII collection | Only required fields |
| Art. 17 (Right to erasure) | Data deletion | User account deletion API |

---

## 9. Next Steps

1. ✅ **Threat modeling complete** - This document
2. ⏳ **Risk analysis** - See [Risk Analysis](../02-risk-analysis/_example-ecommerce-risk.md)
3. ⏳ **Security requirements** - Document required controls in spec
4. ⏳ **Implementation** - Implement planned countermeasures
5. ⏳ **Testing** - Execute security test cases
6. ⏳ **Review** - Security champion sign-off

---

## 10. Review and Approval

**Threat Model Author**: Security Architect (security-champion-lead)
**Review Date**: 2025-11-18
**Reviewers**: security-champion-lead, security-champion-architect
**Approval Status**: ✅ **APPROVED** with conditions (implement planned controls)
**Next Review**: 2026-02-18 (quarterly review)

**Conditions for Production**:
- ✅ All CRITICAL threats mitigated to Low
- ✅ All HIGH threats mitigated to Low or Medium with compensating controls
- ⏳ DDoS protection (WAF) implemented before Black Friday sale
- ⏳ Penetration test completed and findings remediated

---

**Template Used**: [stride-threat-model-template-planning.md](../../templates/01-threat-modeling/stride-threat-model-template-planning.md)
**Related Documents**:
- [Risk Analysis](../02-risk-analysis/_example-ecommerce-risk.md)
- [Security Requirements](../../templates/03-security-requirements/authentication-requirements-template-design.md)
- [PCI-DSS Compliance](../../domaines/rgpd/templates/gdpr-dpia-template.md) (data protection)
