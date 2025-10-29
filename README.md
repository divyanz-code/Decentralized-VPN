# Gate Pass Management System

## Project Title
**Decentralized Gate Pass Management System** - A Blockchain-Based Entry/Exit Management Solution

## Project Description
The Gate Pass Management System is a decentralized smart contract built on the Stellar blockchain using the Soroban SDK. This innovative solution digitizes and automates the traditional gate pass management process for organizations, campuses, residential complexes, and corporate offices.

The system enables users to create digital gate passes, allows administrators to approve them, and tracks the complete lifecycle of each pass from creation to expiration. All transactions are recorded on the blockchain, ensuring transparency, immutability, and eliminating manual paperwork. The smart contract maintains comprehensive statistics including approved passes, pending approvals, expired passes, and total passes created.

Unlike traditional paper-based or centralized digital systems, this blockchain-based approach provides tamper-proof records, eliminates single points of failure, and creates an auditable trail of all entry and exit activities.

## Project Vision
Our vision is to revolutionize access control and visitor management by creating a decentralized, transparent, and efficient gate pass system that:

- **Eliminates Manual Processes**: Removes the need for paper-based gate passes and manual approval workflows
- **Enhances Security**: Provides immutable records of all entry and exit activities for enhanced security auditing
- **Improves Efficiency**: Automates approval workflows and real-time pass status tracking
- **Ensures Transparency**: Makes all pass activities visible and auditable on the blockchain
- **Promotes Accountability**: Creates timestamped records that hold both users and administrators accountable

We envision a future where every organization, from educational institutions to corporate campuses, uses blockchain-based access control systems that are secure, transparent, and efficient.

## Key Features

### 1. **Digital Pass Creation**
- Users can create digital gate passes with title and description
- Automatic unique ID generation for each pass
- Timestamp recording for pass creation time
- Real-time validation to prevent multiple pending passes

### 2. **Admin Approval Workflow**
- Administrators can approve pending passes
- Automatic timestamp recording for approval time (out-time)
- Status change from pending to approved
- Authorization checks to ensure only valid passes are approved

### 3. **Pass Expiration Management**
- Users can expire their passes when exiting
- Automatic timestamp recording for entry time (in-time)
- Status change from approved to expired
- Validation to ensure only approved passes can be expired

### 4. **Comprehensive Statistics Dashboard**
- **Total Passes**: Count of all passes created in the system
- **Approved Passes**: Number of passes approved by administrators
- **Pending Passes**: Count of passes awaiting approval
- **Expired Passes**: Number of passes that have been completed

### 5. **Pass Lifecycle Tracking**
- Complete history from creation to expiration
- Immutable blockchain records
- Timestamped events (creation, approval, expiration)
- Status tracking at every stage

### 6. **Query Capabilities**
- View individual pass details by unique ID
- Check admin control records for any pass
- Access overall system statistics
- Retrieve pass status in real-time

## Future Scope

### Phase 1: Enhanced Functionality
- **Multi-level Approval**: Implement hierarchical approval workflows (security → admin → super admin)
- **Pass Types**: Support different pass categories (visitor, vendor, employee, emergency)
- **Time-based Auto-expiry**: Automatically expire passes after a set duration
- **QR Code Generation**: Generate scannable QR codes for each pass
- **Notification System**: Alert users and admins about pass status changes

### Phase 2: Advanced Security Features
- **Biometric Integration**: Link passes with biometric authentication
- **Geofencing**: Validate passes based on location
- **Access Level Management**: Define different access zones and permissions
- **Blacklist/Whitelist**: Maintain lists for enhanced security
- **Audit Logs**: Detailed logging of all activities with export capabilities

### Phase 3: User Experience Enhancements
- **Mobile Application**: Native apps for iOS and Android
- **Web Dashboard**: Comprehensive web interface for users and administrators
- **Bulk Pass Creation**: Create multiple passes for scheduled visits or events
- **Pass Templates**: Pre-defined templates for common pass types
- **Schedule Management**: Pre-schedule passes for future dates

### Phase 4: Integration & Analytics
- **CCTV Integration**: Link pass data with security camera footage
- **Access Control Systems**: Integrate with physical turnstiles and gates
- **Analytics Dashboard**: Visual insights on pass patterns, peak hours, and trends
- **Reporting Tools**: Generate compliance and security reports
- **API Gateway**: Enable third-party integrations

### Phase 5: Enterprise Features
- **Multi-tenant Support**: Single contract supporting multiple organizations
- **Role-based Access Control (RBAC)**: Granular permission management
- **Compliance Modules**: Support for regulatory requirements (GDPR, data protection)
- **Backup & Recovery**: Automated backup of critical pass data
- **Scalability Enhancements**: Optimize for high-volume environments

### Long-term Vision
- **Interoperability**: Cross-organization pass recognition and validation
- **AI-powered Risk Assessment**: Predict and flag potentially risky passes
- **Blockchain Interoperability**: Support multiple blockchain networks
- **Smart City Integration**: Integrate with broader smart city infrastructure
- **Decentralized Identity**: Link with self-sovereign identity systems

---

## Technical Details

### Smart Contract Architecture

#### Data Structures

**Pass Structure**
```rust
pub struct Pass {
    pub unique_id: u64,      // Unique identifier for the pass
    pub title: String,        // Pass title/purpose
    pub descrip: String,      // Detailed description
    pub crt_time: u64,        // Creation timestamp
    pub in_time: u64,         // Entry/expiry timestamp
    pub isexpired: bool,      // Expiration status
}
```

**Admincontrol Structure**
```rust
pub struct Admincontrol {
    pub ac_id: u64,          // Admin control ID (matches unique_id)
    pub out_time: u64,       // Approval/exit timestamp
    pub approval: bool,      // Approval status
}
```

**ApprovalStatus Structure**
```rust
pub struct ApprovalStatus {
    pub approved: u64,       // Count of approved passes
    pub pending: u64,        // Count of pending passes
    pub expired: u64,        // Count of expired passes
    pub total: u64          // Total passes created
}
```

### Smart Contract Functions

#### `create_pass(env: Env, title: String, descrip: String) -> u64`
Creates a new gate pass with the provided details.
- **Parameters**: title, description
- **Returns**: Unique pass ID
- **Validation**: Ensures user doesn't have an existing pending pass

#### `approve_pass(env: Env, ac_id: u64)`
Approves a pending pass (admin function).
- **Parameters**: Pass ID to approve
- **Effect**: Changes status from pending to approved
- **Validation**: Ensures pass exists and is not already approved

#### `expire_pass(env: Env, unique_id: u64)`
Expires an approved pass (user function).
- **Parameters**: Pass ID to expire
- **Effect**: Changes status from approved to expired
- **Validation**: Ensures pass is approved and not already expired

#### `view_all_pass_status(env: Env) -> ApprovalStatus`
Retrieves overall system statistics.
- **Returns**: ApprovalStatus with counts of all pass types

#### `view_my_pass(env: Env, uniqueid: u64) -> Pass`
Retrieves detailed information about a specific pass.
- **Parameters**: Unique pass ID
- **Returns**: Complete pass details

#### `view_ac_pass_by_unique_id(env: Env, unique_id: u64) -> Admincontrol`
Retrieves admin control information for a pass.
- **Parameters**: Unique pass ID
- **Returns**: Admin control details

### Storage Mechanism
- **Instance Storage**: All data stored using Soroban's instance storage
- **TTL Management**: Storage extended with 5000 ledger TTL
- **Key-Value Mapping**: Enum-based storage keys for efficient retrieval

---

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo package manager
- Soroban CLI
- Stellar account with testnet lumens

### Installation

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install Soroban CLI**
```bash
cargo install --locked soroban-cli
```

3. **Add WebAssembly target**
```bash
rustup target add wasm32-unknown-unknown
```

### Building the Contract

```bash
# Navigate to project directory
cd gate-pass-contract

# Build the contract
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be located at:
`target/wasm32-unknown-unknown/release/gate_pass_contract.wasm`

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Deployment

#### 1. Configure Stellar Network
```bash
# Configure testnet
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

#### 2. Create Identity
```bash
# Generate a new identity
soroban keys generate deployer --network testnet

# Fund the account (get testnet lumens)
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"
```

#### 3. Deploy Contract
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/gate_pass_contract.wasm \
  --source deployer \
  --network testnet
```

### Interacting with the Contract

#### Create a Pass
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user \
  --network testnet \
  -- \
  create_pass \
  --title "Visitor Pass" \
  --descrip "Meeting with HR Department"
```

#### Approve a Pass (Admin)
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network testnet \
  -- \
  approve_pass \
  --ac_id 1
```

#### Expire a Pass
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user \
  --network testnet \
  -- \
  expire_pass \
  --unique_id 1
```

#### View Statistics
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- \
  view_all_pass_status
```

---

## Use Cases

### 1. **Corporate Offices**
- Manage visitor passes for clients and vendors
- Track employee entry/exit times
- Maintain security compliance records

### 2. **Educational Institutions**
- Student gate passes for campus facilities
- Visitor management for parents and guests
- Event-based pass creation for campus activities

### 3. **Residential Complexes**
- Visitor passes for residents' guests
- Service provider entry tracking (plumbers, electricians)
- Emergency access management

### 4. **Healthcare Facilities**
- Patient visitor management
- Vendor and supplier tracking
- Staff movement monitoring

### 5. **Event Management**
- Conference and seminar passes
- VIP access control
- Exhibitor and attendee tracking

---

## Security Considerations

1. **Data Immutability**: All pass records are immutable once written to the blockchain
2. **Authorization**: Functions include validation to prevent unauthorized access
3. **State Management**: Careful state transitions prevent invalid pass statuses
4. **Timestamp Integrity**: Uses blockchain ledger timestamps for accuracy
5. **Storage TTL**: Automatic storage extension prevents data loss

---

## Best Practices

1. **Pass Creation**: Users should create passes well in advance of their visit
2. **Admin Approval**: Admins should promptly review and approve valid passes
3. **Pass Expiry**: Users must expire passes when exiting to maintain accurate records
4. **Status Monitoring**: Regularly check pass status through view functions
5. **Error Handling**: Always check function return values and error messages

---

## Troubleshooting

### Common Issues

**Issue**: "You can't create a pass!"
- **Cause**: User already has a pending or active pass
- **Solution**: Wait for current pass to be approved and expired

**Issue**: "Cannot Approved!!"
- **Cause**: Pass is already approved or doesn't exist
- **Solution**: Verify pass ID and current status

**Issue**: "Pass is not approved yet"
- **Cause**: Attempting to expire an unapproved pass
- **Solution**: Wait for admin approval before expiring

---

## Contributing

We welcome contributions from the community! Here's how you can help:

1. **Report Bugs**: Open an issue describing the problem
2. **Suggest Features**: Share your ideas for improvements
3. **Submit Pull Requests**: Fork, develop, and submit PRs
4. **Documentation**: Help improve documentation and examples
5. **Testing**: Write and improve test coverage

### Development Setup
```bash
# Fork and clone the repository
git clone https://github.com/yourusername/gate-pass-contract.git

# Create a feature branch
git checkout -b feature/your-feature-name

# Make changes and test
cargo test

# Commit and push
git commit -m "Add your feature"
git push origin feature/your-feature-name
```

---

## License
This project is licensed under the MIT License. See the LICENSE file for details.

---

## Support & Contact

- **Documentation**: [Project Wiki](#)
- **Issues**: [GitHub Issues](#)
- **Discussions**: [GitHub Discussions](#)
- **Email**: support@gatepass.example.com

---

## Acknowledgments

- Built with [Soroban SDK](https://soroban.stellar.org/)
- Powered by [Stellar Blockchain](https://stellar.org/)
- Inspired by the need for secure, decentralized access control

---

**Built with 🔐 on Stellar Blockchain using Soroban SDK**

*Making gate pass management transparent, secure, and efficient through blockchain technology.*