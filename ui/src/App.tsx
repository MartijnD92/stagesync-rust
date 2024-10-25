import { useAuth0 } from '@auth0/auth0-react'
import PageLoader from './components/PageLoader'
import AuthenticationGuard from './components/AuthenticationGuard'
import { Route, Routes } from 'react-router-dom'
import HomePage from './pages/HomePage'
import CallbackPage from './pages/CallbackPage'
import NotFoundPage from './pages/NotFoundPage'
import ProfilePage from './pages/ProfilePage'
import DashboardPage from './pages/DashboardPage'

function App() {
    const { isLoading } = useAuth0()
    console.log(isLoading)

    if (isLoading) {
        return (
            <div className="page-layout">
                <PageLoader />
            </div>
        )
    }
    return (
        <Routes>
            <Route path="/" element={<HomePage />} />
            <Route
                path="/dashboard"
                element={<AuthenticationGuard component={DashboardPage} />}
            />
            <Route
                path="/profile"
                element={<AuthenticationGuard component={ProfilePage} />}
            />
            <Route
                path="/404"
                element={<AuthenticationGuard component={NotFoundPage} />}
            />
            <Route path="/callback" element={<CallbackPage />} />
            <Route path="*" element={<NotFoundPage />} />
        </Routes>
    )
}

export default App
