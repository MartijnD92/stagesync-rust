import { useAuth0 } from '@auth0/auth0-react'
import React from 'react'
import PageLayout from '../../components/PageLayout'
import { getUserDashboard } from '../../services/data.service'
import { ArtistFormatted } from '../../models/artist'

function ArtistOverviewPage() {
    const [artists, setArtists] = React.useState<ArtistFormatted[]>([])
    const [error, setError] = React.useState<boolean>(false)

    const { getAccessTokenSilently } = useAuth0()

    React.useEffect(() => {
        let isMounted = true

        const getMessage = async () => {
            const accessToken = await getAccessTokenSilently()
            const { data, error } = await getUserDashboard(accessToken)

            if (!isMounted) {
                return
            }

            if (data) {
                setArtists(data)
            }

            if (error) {
                setError(true)
            }
        }

        getMessage()

        return () => {
            isMounted = false
        }
    }, [getAccessTokenSilently])

    return (
        <PageLayout>
            {(!error && (
                <table>
                    <tbody>
                        <tr>
                            <th>Name</th>
                            <th>Description</th>
                            <th>Fee</th>
                            <th>Genre</th>
                            <th>Based at</th>
                            <th>Owner</th>
                            <th>Upcoming gig</th>
                        </tr>
                        {artists.map((a) => (
                            <tr key={a.id}>
                                <td>{a.name}</td>
                                <td>{a.description || '-'}</td>
                                <td>{a.feeWithCurrency}</td>
                                <td>{a.genre || '-'}</td>
                                <td>{a.location || '-'}</td>
                                <td>{a.ownerName}</td>
                                <td>{(a.gigs[0] && a.gigs[0].date) || '-'}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            )) || <span>ERROR</span>}
        </PageLayout>
    )
}

export default ArtistOverviewPage
