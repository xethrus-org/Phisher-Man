# PhisherMan

Phishing simulation and security awareness training platform built with Rust.

## What is this?

PhisherMan helps organizations test their employees' vulnerability to phishing attacks in a controlled, educational environment. Create campaigns, send simulated phishing emails, track interactions, and analyze results.

**Important**: This is for authorized security testing and education only.

## Tech Stack

- **Backend**: Rust + Axum web framework
- **Database**: PostgreSQL (multi-tenant architecture)
- **Frontend**: HTML/CSS/JavaScript (served as static files)
- **Deployment**: Docker

## Features

- **Multi-tenant**: Support for multiple companies/organizations
- **Employee Management**: Store employee data with custom metadata
- **Campaign Management**: Create and track phishing campaigns
- **Email Templates**: Pre-built and custom email templates
- **Tracking**: Monitor email opens, link clicks, and interactions (coming soon)
- **Analytics**: Campaign metrics and reporting (coming soon)

## Quick Start

### Prerequisites

- Rust 1.88+ ([install rustup](https://rustup.rs/))
- Docker (for PostgreSQL)

### Setup

1. Clone the repo
```bash
git clone git@github.com:xethrus-org/Phisher-Man.git
cd PhisherMan
```

2. Start PostgreSQL
```bash
docker run --name phisherman-db \
  -e POSTGRES_PASSWORD=dev123 \
  -e POSTGRES_DB=phisherman \
  -p 5432:5432 \
  -d postgres:15
```

3. Set up environment
```bash
cp .env.example .env
# Edit .env with your database URL if needed
```

4. Run migrations
```bash
docker exec -i phisherman-db psql -U postgres -d phisherman < migrations/001_initial_schema.sql
```

5. Build and run
```bash
cargo build
cargo run
```

Server runs on `http://localhost:3000`

## API Endpoints

### Companies
- `POST /api/companies` - Create company
- `GET /api/companies` - List companies
- `GET /api/companies/:id` - Get company
- `PATCH /api/companies/:id` - Update company
- `DELETE /api/companies/:id` - Delete company

### Employees
- `POST /api/employees` - Create employee
- `GET /api/employees?company_id=<uuid>` - List employees (filter by company)
- `GET /api/employees/:id` - Get employee
- `PATCH /api/employees/:id` - Update employee
- `DELETE /api/employees/:id` - Delete employee

### Campaigns
- `POST /api/campaigns` - Create campaign
- `GET /api/campaigns?company_id=<uuid>` - List campaigns
- `GET /api/campaigns/:id` - Get campaign
- `PATCH /api/campaigns/:id` - Update campaign
- `DELETE /api/campaigns/:id` - Delete campaign

### Templates
- `POST /api/templates` - Create template
- `GET /api/templates` - List templates
- `GET /api/templates/:id` - Get template
- `PATCH /api/templates/:id` - Update template
- `DELETE /api/templates/:id` - Delete template

### Example

Create a company:
```bash
curl -X POST http://localhost:3000/api/companies \
  -H "Content-Type: application/json" \
  -d '{"name":"Acme Corp","domain":"acme.com"}'
```

## Project Structure

```
PhisherMan/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error handling
│   ├── models/              # Database models
│   ├── handlers/            # API request handlers
│   ├── services/            # Business logic (future)
│   └── db/                  # Database utilities
├── migrations/              # SQL migrations
├── static/                  # Frontend files
├── DEVELOPMENT.md           # Development guide
└── .env.example             # Environment template
```

## Development

See [DEVELOPMENT.md](./DEVELOPMENT.md) for branching conventions, commit format, and workflow.

## Roadmap

- [x] Multi-tenant database schema
- [x] Company/Employee/Campaign/Template CRUD APIs
- [ ] Email sending (SMTP integration)
- [ ] Tracking pixels and link clicks
- [ ] GPT API integration for AI-generated emails
- [ ] Campaign analytics and metrics
- [ ] Frontend integration with API
- [ ] Docker deployment setup
- [ ] Email spoofing service integration

## License

ISC
