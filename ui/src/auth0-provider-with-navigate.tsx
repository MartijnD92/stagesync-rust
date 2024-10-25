import { Auth0Provider, AppState } from '@auth0/auth0-react'
import React, { PropsWithChildren } from 'react'
import { useNavigate } from 'react-router-dom'

interface Auth0ProviderWithNavigateProps {
    children: React.ReactNode
}

export const Auth0ProviderWithNavigate = ({
    children,
}: PropsWithChildren<Auth0ProviderWithNavigateProps>): JSX.Element | null => {
    const navigate = useNavigate()

    const domain = import.meta.env.AUTH0_DOMAIN
    const clientId = import.meta.env.AUTH0_CLIENT_ID
    const redirectUri = import.meta.env.AUTH0_CALLBACK_URL
    const audience = import.meta.env.AUTH0_AUDIENCE

    console.log(__AUTH0_DOMAIN__)

    const onRedirectCallback = (appState?: AppState) => {
        navigate(appState?.returnTo || window.location.pathname)
    }

    if (!(domain && clientId && redirectUri && audience)) {
        return null
    }

    return (
        <Auth0Provider
            domain={domain}
            clientId={clientId}
            authorizationParams={{
                audience: audience,
                redirect_uri: redirectUri,
            }}
            onRedirectCallback={onRedirectCallback}
        >
            {children}
        </Auth0Provider>
    )
}
