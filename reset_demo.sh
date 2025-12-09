#!/bin/bash
# Quick reset script to clear database between demos

echo "🔄 Resetting PhisherMan database..."

docker-compose exec -T db psql -U postgres -d phisherman <<-EOSQL
    TRUNCATE companies, employees, campaigns, templates, sent_emails, interactions CASCADE;
EOSQL

if [ $? -eq 0 ]; then
    echo "✅ Database reset complete! All data cleared."
    echo ""
    echo "Next steps:"
    echo "1. Follow DEMO_GUIDE.md starting from Part 2 (Create Demo Data)"
    echo "2. Or run: docker-compose down -v && docker-compose up --build"
else
    echo "❌ Reset failed. Make sure Docker containers are running."
    echo "Try: docker-compose up"
fi
