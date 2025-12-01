// API utility functions
const API_BASE = '/api';

async function apiCall(endpoint, options = {}) {
    const url = `${API_BASE}${endpoint}`;
    const config = {
        headers: {
            'Content-Type': 'application/json',
            ...options.headers,
        },
        ...options,
    };

    try {
        const response = await fetch(url, config);

        if (!response.ok) {
            const error = await response.json().catch(() => ({ error: 'Request failed' }));
            throw new Error(error.error || `HTTP ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('API Error:', error);
        throw error;
    }
}

// API methods
const API = {
    // Companies
    companies: {
        list: () => apiCall('/companies'),
        get: (id) => apiCall(`/companies/${id}`),
        create: (data) => apiCall('/companies', { method: 'POST', body: JSON.stringify(data) }),
        update: (id, data) => apiCall(`/companies/${id}`, { method: 'PATCH', body: JSON.stringify(data) }),
        delete: (id) => apiCall(`/companies/${id}`, { method: 'DELETE' }),
    },

    // Employees
    employees: {
        list: (companyId) => apiCall(`/employees${companyId ? `?company_id=${companyId}` : ''}`),
        get: (id) => apiCall(`/employees/${id}`),
        create: (data) => apiCall('/employees', { method: 'POST', body: JSON.stringify(data) }),
        update: (id, data) => apiCall(`/employees/${id}`, { method: 'PATCH', body: JSON.stringify(data) }),
        delete: (id) => apiCall(`/employees/${id}`, { method: 'DELETE' }),
    },

    // Campaigns
    campaigns: {
        list: (companyId) => apiCall(`/campaigns${companyId ? `?company_id=${companyId}` : ''}`),
        get: (id) => apiCall(`/campaigns/${id}`),
        create: (data) => apiCall('/campaigns', { method: 'POST', body: JSON.stringify(data) }),
        update: (id, data) => apiCall(`/campaigns/${id}`, { method: 'PATCH', body: JSON.stringify(data) }),
        delete: (id) => apiCall(`/campaigns/${id}`, { method: 'DELETE' }),
        send: (id, templateId) => apiCall(`/campaigns/${id}/send`, {
            method: 'POST',
            body: JSON.stringify({ template_id: templateId })
        }),
    },

    // Templates
    templates: {
        list: () => apiCall('/templates'),
        get: (id) => apiCall(`/templates/${id}`),
        create: (data) => apiCall('/templates', { method: 'POST', body: JSON.stringify(data) }),
        update: (id, data) => apiCall(`/templates/${id}`, { method: 'PATCH', body: JSON.stringify(data) }),
        delete: (id) => apiCall(`/templates/${id}`, { method: 'DELETE' }),
    },
};

// UI helpers
function showMessage(message, type = 'success') {
    const messageDiv = document.createElement('div');
    messageDiv.className = `message message-${type}`;
    messageDiv.textContent = message;
    messageDiv.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        padding: 15px 20px;
        background: ${type === 'success' ? '#4CAF50' : '#f44336'};
        color: white;
        border-radius: 4px;
        box-shadow: 0 2px 5px rgba(0,0,0,0.2);
        z-index: 1000;
        animation: slideIn 0.3s ease-out;
    `;

    document.body.appendChild(messageDiv);

    setTimeout(() => {
        messageDiv.style.animation = 'slideOut 0.3s ease-out';
        setTimeout(() => messageDiv.remove(), 300);
    }, 3000);
}

function formatDate(dateString) {
    return new Date(dateString).toLocaleString();
}
