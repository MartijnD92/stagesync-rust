import React from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import './styles/styles.css'
import App from './App.tsx'
import Auth0ProviderWithNavigate from './auth0-provider-with-navigate'

createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <BrowserRouter>
            <Auth0ProviderWithNavigate>
                <App />
            </Auth0ProviderWithNavigate>
        </BrowserRouter>
    </React.StrictMode>
)
