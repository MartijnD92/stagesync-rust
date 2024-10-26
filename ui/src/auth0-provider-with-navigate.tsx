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

    const domain = __AUTH0_DOMAIN__
    const clientId = __AUTH0_CLIENT_ID__
    const redirectUri = __AUTH0_CALLBACK_URL__
    const audience = __AUTH0_AUDIENCE__

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
